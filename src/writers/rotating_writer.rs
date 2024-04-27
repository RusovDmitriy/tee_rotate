use std::fs::{self, File, OpenOptions};
use std::io::{self, Write};
use std::path::PathBuf;

pub struct RotatingWriter {
    pub name: String,
    append: bool,
    rotate: bool,
    file: File,
    max_size: u64,
    current_size: u64,
    base_name: String,
    current_file_index: u32,
}

impl RotatingWriter {
    pub fn new(base_name: &str, append: bool, rotate: bool, max_size: u64) -> io::Result<Self> {
        let current_file_index = Self::get_current_index(base_name)?;
        let file = Self::open_file(base_name, current_file_index, append)?;
        let current_size = file.metadata()?.len();
        Ok(Self {
            append,
            rotate,
            name: base_name.to_owned(),
            file,
            max_size,
            current_size,
            base_name: base_name.to_owned(),
            current_file_index,
        })
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
        Ok(())
    }
}

impl Write for RotatingWriter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let bytes_written = self.file.write(buf)?;
        self.current_size += bytes_written as u64;

        if self.rotate && self.current_size >= self.max_size {
            self.rotate_file()?;
        }
        Ok(bytes_written)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.file.flush()
    }
}
