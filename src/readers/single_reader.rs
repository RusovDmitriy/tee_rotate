use std::io::{Read, Result};

pub struct SingleReader {
    pub inner: Box<dyn Read>,
}

impl Read for SingleReader {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        match self.inner.read(buf) {
            Err(f) => {
                println!("stdin: {}", f);
                Err(f)
            }
            okay => okay,
        }
    }
}
