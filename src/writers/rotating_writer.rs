use glob::glob;
use std::fs::{self, File, OpenOptions};
use std::io::{self, Write};
use std::path::PathBuf;

pub struct RotatingWriter {
    pub name: String,
    files: Vec<String>,
    append: bool,
    rotate: bool,
    file: File,
    max_size_bytes: u64,
    max_log_files: u64,
    current_size: u64,
    base_name: String,
    current_file_index: u32,
}

impl RotatingWriter {
    pub fn new(
        base_name: &str,
        append: bool,
        rotate: bool,
        max_size_bytes: u64,
        max_log_files: u64,
    ) -> io::Result<Self> {
        let current_file_index = Self::get_current_index(base_name)?;
        let file = Self::open_file(base_name, current_file_index, append)?;
        let current_size = file.metadata()?.len();
        let files = Self::fill_files(base_name)?;
        Ok(Self {
            append,
            rotate,
            name: base_name.to_owned(),
            file,
            files,
            max_size_bytes,
            max_log_files,
            current_size,
            base_name: base_name.to_owned(),
            current_file_index,
        })
    }

    fn fill_files(base_name: &str) -> io::Result<Vec<String>> {
        let pattern = format!("{}.*", base_name);
        let mut files = Vec::new();
        let paths = glob(&pattern);
        for entry in paths.expect("Failed to read glob pattern") {
            match entry {
                Ok(path) => files.insert(files.len(), path.display().to_string()),
                Err(_e) => {
                    // TODO: Add conditional ignore errors if output_error is exit or exit-nopipe
                }
            }
        }
        files.sort();
        Ok(files)
    }

    fn get_current_index(base_name: &str) -> io::Result<u32> {
        let mut index = 0;
        while fs::metadata(Self::get_log_file_name(base_name, index)).is_ok() {
            index += 1;
        }

        // Minus 1 to get the last index that does exist
        if index > 0 {
            index -= 1;
        }
        Ok(index)
    }

    // TODO: Add conditional ignore errors if output_error is exit or exit-nopipe
    fn open_file(base_name: &str, index: u32, append: bool) -> io::Result<File> {
        let file_name = Self::get_log_file_name(base_name, index);
        let path = PathBuf::from(file_name);
        let mut options = OpenOptions::new();
        let mode = if append {
            options.append(true)
        } else {
            options.truncate(true)
        };

        mode.write(true).create(true).open(path.as_path())
    }

    fn get_log_file_name(base_name: &str, index: u32) -> String {
        format!("{}.{}", base_name, index)
    }

    fn rotate_file(&mut self) -> io::Result<()> {
        self.current_size = 0;
        self.current_file_index += 1;
        self.file = Self::open_file(&self.base_name, self.current_file_index, self.append)?;
        self.files = Self::fill_files(&self.base_name)?;
        if self.files.len() > self.max_log_files as usize {
            let file_to_remove = self.files.remove(0);
            fs::remove_file(file_to_remove)?;
        }
        Ok(())
    }
}

impl Write for RotatingWriter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let bytes_written = self.file.write(buf)?;
        self.current_size += bytes_written as u64;

        if self.rotate && self.current_size >= self.max_size_bytes {
            self.rotate_file()?;
        }
        Ok(bytes_written)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.file.flush()
    }
}
