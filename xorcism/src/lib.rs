use std::borrow::Borrow;
use std::io::{Read, Write};
use std::iter::Cycle;
use std::slice::Iter;

/// A munger which XORs a key with some data
#[derive(Clone)]
pub struct Xorcism<'a> {
    key: Cycle<Iter<'a, u8>>,
}

impl<'a> Xorcism<'a> {
    /// Create a new Xorcism munger from a key
    ///
    /// Should accept anything which has a cheap conversion to a byte slice.
    pub fn new<Key: AsRef<[u8]> + ?Sized>(key: &'a Key) -> Xorcism<'a> {
        Self {
            key: key.as_ref().iter().cycle(),
        }
    }

    pub fn munge_in_place(&mut self, data: &mut [u8]) {
        data.iter_mut()
            .for_each(|d| *d = *d ^ self.key.next().unwrap());
    }

    pub fn munge<'b, T, Data>(
        &'b mut self,
        data: Data,
    ) -> impl Iterator<Item = u8> + Captures<'a> + 'b
    where
        Data: IntoIterator<Item = T> + 'b,
        T: Borrow<u8>,
    {
        data.into_iter()
            .map(move |d| d.borrow() ^ self.key.next().unwrap())
    }

    #[cfg(feature = "io")]
    pub fn reader(self, reader: impl Read + 'a) -> impl Read + 'a {
        XorcimReaderWriter::new(reader, self)
    }

    #[cfg(feature = "io")]
    pub fn writer(self, writer: impl Write + 'a) -> impl Write + 'a {
        XorcimReaderWriter::new(writer, self)
    }
}

pub trait Captures<'a> {}

impl<'a, T: ?Sized> Captures<'a> for T {}

#[cfg(feature = "io")]
struct XorcimReaderWriter<'a, Inner> {
    inner: Inner,
    munger: Xorcism<'a>,
    buf: Vec<u8>,
}

#[cfg(feature = "io")]
impl<'a, R> XorcimReaderWriter<'a, R> {
    pub fn new(inner: R, m: Xorcism<'a>) -> Self {
        Self {
            inner,
            munger: m,
            buf: vec![],
        }
    }
}

#[cfg(feature = "io")]
impl<'a, R: Read> Read for XorcimReaderWriter<'a, R> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.inner.read(buf).map(|n| {
            self.munger.munge_in_place(&mut buf[..n]);
            n
        })
    }
}

#[cfg(feature = "io")]
impl<'a, W: Write> Write for XorcimReaderWriter<'a, W> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.buf.extend(self.munger.munge(buf));
        let bytes = self.inner.write(&self.buf)?;
        self.buf.clear();
        Ok(bytes)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.inner.flush()
    }
}
