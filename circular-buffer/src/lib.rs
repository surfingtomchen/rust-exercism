use crate::Error::{EmptyBuffer, FullBuffer};

pub struct CircularBuffer<T> {
    buf: Vec<T>,
}

#[derive(Debug, PartialEq)]
pub enum Error {
    EmptyBuffer,
    FullBuffer,
}

impl<T> CircularBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        Self {
            buf: Vec::with_capacity(capacity),
        }
    }

    pub fn write(&mut self, element: T) -> Result<(), Error> {
        if self.buf.len() == self.buf.capacity() {
            Err(FullBuffer)
        } else {
            Ok(self.buf.push(element))
        }
    }

    pub fn read(&mut self) -> Result<T, Error> {
        if self.buf.is_empty() {
            Err(EmptyBuffer)
        } else {
            Ok(self.buf.remove(0))
        }
    }

    pub fn clear(&mut self) {
        self.buf.clear()
    }

    pub fn overwrite(&mut self, element: T) {
        if self.buf.len() == self.buf.capacity() {
            self.buf.remove(0);
        }
        self.buf.push(element)
    }
}
