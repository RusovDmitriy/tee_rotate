mod cli;
mod readers;
mod writers;

use std::io::{copy, stdin, stdout, Error, ErrorKind, Read, Result, Write};

use readers::single_reader::SingleReader;
use writers::generic_writer::GenericWriter;
use writers::multi_writer::MultiWriter;
use writers::rotating_writer::RotatingWriter;
use writers::single_writer::SingleWriter;

fn main() -> Result<()> {
    let options = cli::init();

    // Create a vector of RotatingWriter instances for each file in the options.files vector
    let mut writers: Vec<GenericWriter> = options
        .files
        .iter()
        .filter_map(|name| {
            let rotating_writer = RotatingWriter::new(
                &name,
                options.append,
                options.rotate,
                options.max_size_bytes,
            );
            let log_writer = Ok(GenericWriter::Rotating(rotating_writer.unwrap()));
            Some(log_writer)
        })
        .collect::<Result<Vec<GenericWriter>>>()?;

    // Insert a SingleWriter instance for stdout at the beginning of the writers vector
    writers.insert(
        0,
        GenericWriter::Single(SingleWriter {
            inner: Box::new(stdout()),
            name: "stdout".to_owned(),
        }),
    );

    // Create a MultiWriter instance from the writers vector
    let mut output = MultiWriter::new(writers, options.output_error);

    // Create a SingleReader instance for stdin
    let input = &mut SingleReader {
        inner: Box::new(stdin()) as Box<dyn Read>,
    };

    // Copy the contents of stdin to the MultiWriter instance
    let res = match copy(input, &mut output) {
        Err(e) if e.kind() != ErrorKind::Other => Err(e),
        _ => Ok(()),
    };

    if res.is_err() || output.flush().is_err() {
        Err(Error::from(ErrorKind::Other))
    } else {
        Ok(())
    }
}
