use std::io::{self, Write};

pub struct SingleWriter {
    pub inner: Box<dyn Write>,
    pub name: String,
}

impl Write for SingleWriter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.inner.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.inner.flush()
    }
}
