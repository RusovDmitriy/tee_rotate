use std::io::{Error, ErrorKind, Result, Write};

use super::generic_writer::GenericWriter;
use crate::cli::OutputErrorMode;

pub struct MultiWriter {
    pub writers: Vec<GenericWriter>,
    output_error_mode: Option<OutputErrorMode>,
    ignored_errors: usize,
}

impl MultiWriter {
    pub fn new(writers: Vec<GenericWriter>, output_error_mode: Option<OutputErrorMode>) -> Self {
        Self {
            writers,
            output_error_mode,
            ignored_errors: 0,
        }
    }
}

impl Write for MultiWriter {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let mut aborted = None;
        let mode = self.output_error_mode.clone();
        let mut errors = 0;
        self.writers.retain_mut(|writer| {
            let result = writer.write_all(buf);
            match result {
                Err(f) => {
                    if let Err(e) = process_error(mode.as_ref(), f, writer, &mut errors) {
                        if aborted.is_none() {
                            aborted = Some(e);
                        }
                    }
                    false
                }
                _ => true,
            }
        });

        if let Some(e) = aborted {
            Err(e)
        } else if self.writers.is_empty() {
            // This error kind will never be raised by the standard
            // library, so we can use it for early termination of
            // `copy`
            Err(Error::from(ErrorKind::Other))
        } else {
            Ok(buf.len())
        }
    }

    fn flush(&mut self) -> Result<()> {
        let mut aborted = None;
        let mode = self.output_error_mode.clone();
        let mut errors = 0;
        self.writers.retain_mut(|writer| {
            let result = writer.flush();
            match result {
                Err(f) => {
                    if let Err(e) = process_error(mode.as_ref(), f, writer, &mut errors) {
                        if aborted.is_none() {
                            aborted = Some(e);
                        }
                    }
                    false
                }
                _ => true,
            }
        });
        self.ignored_errors += errors;
        if let Some(e) = aborted {
            Err(e)
        } else {
            Ok(())
        }
    }
}

fn process_error(
    mode: Option<&OutputErrorMode>,
    f: Error,
    writer: &mut GenericWriter,
    ignored_errors: &mut usize,
) -> Result<()> {
    match mode {
        Some(OutputErrorMode::Warn) => {
            println!("{}: {}", writer.name(), f);
            *ignored_errors += 1;
            Ok(())
        }
        Some(OutputErrorMode::WarnNoPipe) | None => {
            if f.kind() != ErrorKind::BrokenPipe {
                println!("{}: {}", writer.name(), f);
                *ignored_errors += 1;
            }
            Ok(())
        }
        Some(OutputErrorMode::Exit) => {
            println!("{}: {}", writer.name(), f);
            Err(f)
        }
        Some(OutputErrorMode::ExitNoPipe) => {
            if f.kind() == ErrorKind::BrokenPipe {
                Ok(())
            } else {
                println!("{}: {}", writer.name(), f);
                Err(f)
            }
        }
    }
}
