//! RSC5 resource container: 12-byte header plus a zlib payload.
//!
//! Layout in words: 4 magic bytes (`RSC` + `0x05`), a u32 resource type id, a
//! u32 flags word, then a zlib stream to the end of the file.

use crate::{Error, u32le};
use std::io::Read;

/// Magic bytes opening every RSC5 resource.
pub const MAGIC: [u8; 4] = *b"RSC\x05";

/// Size of the container header in bytes.
pub const HEADER_LEN: usize = 12;

/// Largest inflated payload accepted; bounds hostile zlib streams.
const MAX_PAYLOAD: u64 = 256 * 1024 * 1024;

/// A parsed RSC5 resource with its inflated payload.
#[derive(Debug, Clone)]
pub struct Resource {
    type_id: u32,
    flags: u32,
    payload: Vec<u8>,
}

impl Resource {
    /// Parse a resource from its file bytes, inflating the payload.
    ///
    /// # Errors
    ///
    /// Returns an error if the input is truncated or malformed.
    pub fn parse(bytes: &[u8]) -> Result<Self, Error> {
        if bytes.len() < HEADER_LEN {
            return Err(Error::UnexpectedEnd { offset: 0 });
        }
        if bytes[0..4] != MAGIC {
            return Err(Error::BadMagic);
        }
        let resource_type = u32le(bytes, 4)?;
        let flags = u32le(bytes, 8)?;
        let decoder = flate2::read::ZlibDecoder::new(&bytes[HEADER_LEN..]);
        let mut payload = Vec::new();
        decoder
            .take(MAX_PAYLOAD + 1)
            .read_to_end(&mut payload)
            .map_err(|_| Error::Inflate)?;
        if payload.len() as u64 > MAX_PAYLOAD {
            return Err(Error::BadSize {
                expected: usize::try_from(MAX_PAYLOAD).unwrap_or(usize::MAX),
                actual: payload.len(),
            });
        }
        Ok(Self {
            type_id: resource_type,
            flags,
            payload,
        })
    }

    /// Resource type id (1 for navigation meshes).
    #[must_use]
    pub fn resource_type(&self) -> u32 {
        self.type_id
    }

    /// Flags word from the header.
    #[must_use]
    pub fn flags(&self) -> u32 {
        self.flags
    }

    /// Inflated payload bytes.
    #[must_use]
    pub fn payload(&self) -> &[u8] {
        &self.payload
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    fn wrap(payload: &[u8]) -> Vec<u8> {
        let mut enc = flate2::write::ZlibEncoder::new(Vec::new(), flate2::Compression::default());
        enc.write_all(payload).unwrap();
        let mut out = vec![b'R', b'S', b'C', 5, 1, 0, 0, 0, 0, 0, 0, 0];
        out.extend_from_slice(&enc.finish().unwrap());
        out
    }

    #[test]
    fn round_trip() {
        let bytes = wrap(b"hello tile");
        let r = Resource::parse(&bytes).unwrap();
        assert_eq!(r.resource_type(), 1);
        assert_eq!(r.payload(), b"hello tile");
    }

    #[test]
    fn rejects_bad_magic_and_short_input() {
        assert_eq!(
            Resource::parse(b"short").unwrap_err(),
            Error::UnexpectedEnd { offset: 0 }
        );
        let mut bytes = wrap(b"data");
        bytes[0] = b'X';
        assert_eq!(Resource::parse(&bytes).unwrap_err(), Error::BadMagic);
    }

    #[test]
    fn rejects_broken_zlib() {
        let bytes = vec![b'R', b'S', b'C', 5, 1, 0, 0, 0, 0, 0, 0, 0, 0x78, 0x00];
        assert_eq!(Resource::parse(&bytes).unwrap_err(), Error::Inflate);
    }
}
