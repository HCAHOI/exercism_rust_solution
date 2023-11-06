use std::{
    borrow::Borrow,
    io::{Read, Write},
    iter::Cycle,
};

/// A munger which XORs a key with some data
#[derive(Clone)]
pub struct Xorcism<'a> {
    key: Cycle<std::slice::Iter<'a, u8>>,
}

impl<'a> Xorcism<'a> {
    /// Create a new Xorcism munger from a key
    ///
    /// Should accept anything which has a cheap conversion to a byte slice.
    pub fn new<K>(key: &'a K) -> Xorcism<'a>
    where
        K: AsRef<[u8]> + ?Sized,
    {
        Self {
            key: key.as_ref().iter().cycle(),
        }
    }

    /// XOR each byte of the input buffer with a byte from the key.
    ///
    /// Note that this is stateful: repeated calls are likely to produce different results,
    /// even with identical inputs.
    pub fn munge_in_place(&mut self, data: &mut [u8]) {
        data.iter_mut()
            .zip(&mut self.key)
            .for_each(|(d, k)| *d ^= k)
    }

    /// XOR each byte of the data with a byte from the key.
    ///
    /// Note that this is stateful: repeated calls are likely to produce different results,
    /// even with identical inputs.
    ///
    /// Should accept anything which has a cheap conversion to a byte iterator.
    /// Shouldn't matter whether the byte iterator's values are owned or borrowed.
    pub fn munge<'b, D, I>(&'b mut self, data: D) -> impl Iterator<Item = u8> + 'b + Captures<'a>
    where
        D: IntoIterator<Item = I> + 'b,
        I: Borrow<u8>,
    {
        data.into_iter()
            .zip(&mut self.key)
            .map(|(d, &k)| d.borrow() ^ k)
    }

    pub fn reader(self, reader: impl Read + 'a) -> impl Read + 'a {
        XorReader { xor: self, reader }
    }

    pub fn writer(self, writer: impl Write + 'a) -> impl Write + 'a {
        XorWriter { xor: self, writer }
    }
}

struct XorReader<'a, R> {
    xor: Xorcism<'a>,
    reader: R,
}

impl<'a, R: Read> Read for XorReader<'a, R> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let nbytes = self.reader.read(buf)?;
        self.xor.munge_in_place(&mut buf[..nbytes]);
        Ok(nbytes)
    }
}

struct XorWriter<'a, W> {
    xor: Xorcism<'a>,
    writer: W,
}

impl<'a, W: Write> Write for XorWriter<'a, W> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let mut buf = buf.to_owned();
        self.xor.munge_in_place(&mut buf);
        self.writer.write(&buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.writer.flush()
    }
}

pub trait Captures<'a> {}
impl<'a, T: ?Sized> Captures<'a> for T {}
