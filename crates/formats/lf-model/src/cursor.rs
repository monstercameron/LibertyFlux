//! Small bounds-checked little-endian reader over a byte slice.

use crate::Error;

/// Cursor over an immutable byte slice. All reads are explicit
/// little-endian and return [`Error::Truncated`] past the end.
#[derive(Debug, Clone, Copy)]
pub struct Cursor<'a> {
    buf: &'a [u8],
    pos: usize,
}

impl<'a> Cursor<'a> {
    /// New cursor at the given offset.
    pub fn at(buf: &'a [u8], pos: usize) -> Result<Cursor<'a>, Error> {
        if pos > buf.len() {
            return Err(Error::Truncated {
                offset: pos,
                len: buf.len(),
            });
        }
        Ok(Cursor { buf, pos })
    }

    /// Current position.
    pub fn pos(&self) -> usize {
        self.pos
    }

    /// Move to an absolute offset.
    pub fn seek(&mut self, pos: usize) -> Result<(), Error> {
        if pos > self.buf.len() {
            return Err(Error::Truncated {
                offset: pos,
                len: self.buf.len(),
            });
        }
        self.pos = pos;
        Ok(())
    }

    /// Skip forward by `n` bytes.
    pub fn skip(&mut self, n: usize) -> Result<(), Error> {
        self.seek(self.pos.checked_add(n).ok_or(Error::Truncated {
            offset: usize::MAX,
            len: self.buf.len(),
        })?)
    }

    fn take(&mut self, n: usize) -> Result<&'a [u8], Error> {
        let end = self.pos.checked_add(n).ok_or(Error::Truncated {
            offset: usize::MAX,
            len: self.buf.len(),
        })?;
        if end > self.buf.len() {
            return Err(Error::Truncated {
                offset: end,
                len: self.buf.len(),
            });
        }
        let out = &self.buf[self.pos..end];
        self.pos = end;
        Ok(out)
    }

    /// Read one byte.
    pub fn u8(&mut self) -> Result<u8, Error> {
        Ok(self.take(1)?[0])
    }

    /// Read a little-endian u16.
    pub fn u16(&mut self) -> Result<u16, Error> {
        Ok(u16::from_le_bytes(self.take(2)?.try_into().unwrap()))
    }

    /// Read a little-endian i16.
    pub fn i16(&mut self) -> Result<i16, Error> {
        Ok(i16::from_le_bytes(self.take(2)?.try_into().unwrap()))
    }

    /// Read a little-endian u32.
    pub fn u32(&mut self) -> Result<u32, Error> {
        Ok(u32::from_le_bytes(self.take(4)?.try_into().unwrap()))
    }

    /// Read a little-endian i32.
    pub fn i32(&mut self) -> Result<i32, Error> {
        Ok(i32::from_le_bytes(self.take(4)?.try_into().unwrap()))
    }

    /// Read a little-endian u64.
    pub fn u64(&mut self) -> Result<u64, Error> {
        Ok(u64::from_le_bytes(self.take(8)?.try_into().unwrap()))
    }

    /// Read a little-endian f32.
    pub fn f32(&mut self) -> Result<f32, Error> {
        Ok(f32::from_le_bytes(self.take(4)?.try_into().unwrap()))
    }

    /// Read four little-endian f32 values.
    pub fn vec4(&mut self) -> Result<[f32; 4], Error> {
        Ok([self.f32()?, self.f32()?, self.f32()?, self.f32()?])
    }

    /// Read a sixteen-float matrix in file order.
    pub fn mat4(&mut self) -> Result<[[f32; 4]; 4], Error> {
        Ok([
            [self.f32()?, self.f32()?, self.f32()?, self.f32()?],
            [self.f32()?, self.f32()?, self.f32()?, self.f32()?],
            [self.f32()?, self.f32()?, self.f32()?, self.f32()?],
            [self.f32()?, self.f32()?, self.f32()?, self.f32()?],
        ])
    }
}
