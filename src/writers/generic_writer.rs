use super::rotating_writer::RotatingWriter;
use super::single_writer::SingleWriter;
use std::io::{self, Write};

pub enum GenericWriter {
    Single(SingleWriter),
    Rotating(RotatingWriter),
}

impl GenericWriter {
    pub fn name(&self) -> &str {
        match self {
            GenericWriter::Single(writer) => &writer.name,
            GenericWriter::Rotating(writer) => &writer.name,
        }
    }
}

impl Write for GenericWriter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match self {
            GenericWriter::Single(writer) => writer.write(buf),
            GenericWriter::Rotating(writer) => writer.write(buf),
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        match self {
            GenericWriter::Single(writer) => writer.flush(),
            GenericWriter::Rotating(writer) => writer.flush(),
        }
    }
}
