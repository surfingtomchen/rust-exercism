use std::io::{Read, Result, Write};

pub struct ReadStats<R> {
    wrapped: R,
    reads: usize,
    bytes_through: usize,
}

impl<R: Read> ReadStats<R> {
    pub fn new(wrapped: R) -> ReadStats<R> {
        Self {
            wrapped,
            reads: 0,
            bytes_through: 0,
        }
    }

    pub fn get_ref(&self) -> &R {
        &self.wrapped
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes_through
    }

    pub fn reads(&self) -> usize {
        self.reads
    }
}

impl<R: Read> Read for ReadStats<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.wrapped.read(buf).and_then(|size| {
            self.reads += 1;
            self.bytes_through += size;
            Ok(size)
        })
    }
}

pub struct WriteStats<W> {
    wrapped: W,
    bytes_through: usize,
    writes: usize,
}

impl<W: Write> WriteStats<W> {
    pub fn new(wrapped: W) -> WriteStats<W> {
        Self {
            wrapped,
            bytes_through: 0,
            writes: 0,
        }
    }

    pub fn get_ref(&self) -> &W {
        &self.wrapped
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes_through
    }

    pub fn writes(&self) -> usize {
        self.writes
    }
}

impl<W: Write> Write for WriteStats<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.wrapped.write(buf).and_then(|size| {
            self.bytes_through += size;
            self.writes += 1;
            Ok(size)
        })
    }

    fn flush(&mut self) -> Result<()> {
        self.wrapped.flush()
    }
}
