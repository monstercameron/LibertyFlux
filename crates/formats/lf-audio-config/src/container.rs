//! The versioned metadata container (`*.dat11/12/15/16`).
//!
//! This module parses the outer container shared by the effects, curves,
//! categories, sounds and game files: version suffix, object blob,
//! archive-name table, object directory and the two relocation tables. Use
//! [`crate::schema::Schema`] and [`crate::decode::decode_object`] to decode
//! the object bodies.

use crate::{Cursor, Error, ErrorKind};

/// A parsed versioned metadata file borrowing its input.
#[derive(Debug)]
pub struct MetaFile<'a> {
    buf: &'a [u8],
    suffix: u32,
    blob: &'a [u8],
    archives: Vec<Archive>,
    objects: Vec<ObjectEntry<'a>>,
    hash_offsets: Vec<u32>,
    archive_offsets: Vec<u32>,
}

/// One `ARCHIVE\BANK` path from the archive-name table.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Archive {
    name: String,
}

impl Archive {
    /// The bank path as stored (archive and bank separated by `\`).
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }
}

/// One object directory entry with its blob slice attached.
#[derive(Debug, Clone, Copy)]
pub struct ObjectEntry<'a> {
    name: &'a str,
    offset: u32,
    size: u32,
    data: &'a [u8],
}

impl<'a> ObjectEntry<'a> {
    /// Object name from the directory.
    #[must_use]
    pub fn name(&self) -> &'a str {
        self.name
    }

    /// Byte offset of the object within the blob.
    #[must_use]
    pub fn offset(&self) -> u32 {
        self.offset
    }

    /// Object size in bytes, including the type id and name offset.
    #[must_use]
    pub fn size(&self) -> u32 {
        self.size
    }

    /// The raw object bytes.
    #[must_use]
    pub fn data(&self) -> &'a [u8] {
        self.data
    }

    /// Leading type-id byte.
    #[must_use]
    pub fn type_id(&self) -> u8 {
        self.data[0]
    }

    /// The u32 name offset stored after the type id.
    #[must_use]
    pub fn name_offset(&self) -> u32 {
        u32::from_le_bytes([self.data[1], self.data[2], self.data[3], self.data[4]])
    }

    /// Build an entry over hand-made bytes for tests.
    #[cfg(test)]
    pub(crate) fn for_test<'x>(name: &'x str, offset: u32, data: &'x [u8]) -> ObjectEntry<'x> {
        ObjectEntry {
            name,
            offset,
            size: data.len() as u32,
            data,
        }
    }
}

impl<'a> MetaFile<'a> {
    /// Parse a whole file. The input must end exactly after the relocation
    /// tables; anything else is [`ErrorKind::Invalid`].
    ///
    /// # Errors
    ///
    /// Returns an error if the input is truncated or malformed.
    pub fn parse(buf: &'a [u8]) -> Result<MetaFile<'a>, Error> {
        let mut cur = Cursor::new(buf);
        let suffix = cur.u32("version suffix")?;
        let data_size = cur.u32("blob size")? as usize;
        let blob_start = cur.pos();
        let blob = cur.bytes(data_size, "object blob")?;

        // Archive table: block size (counted after the size word itself),
        // count, offsets, then the name heap.
        let block_size = cur.u32("archive block size")? as usize;
        let archive_count = cur.u32("archive count")? as usize;
        let mut archive_offsets =
            Vec::with_capacity(archive_count.min(cur.remaining() / 4 + 1));
        for _ in 0..archive_count {
            archive_offsets.push(cur.u32("archive offset")? as usize);
        }
        let heap_len = block_size
            .checked_sub(4 + archive_count * 4)
            .ok_or_else(|| {
                Error::new(
                    cur.pos() as u64,
                    ErrorKind::Invalid,
                    "archive block smaller than its offset table",
                )
            })?;
        let heap = cur.bytes(heap_len, "archive name heap")?;
        let mut archives = Vec::with_capacity(archive_offsets.len());
        for off in &archive_offsets {
            archives.push(Archive {
                name: heap_string(heap, *off, blob_start + data_size)?,
            });
        }

        // Object directory.
        let object_count = cur.u32("object count")? as usize;
        let _names_capacity = cur.u32("names capacity")?;
        let mut objects = Vec::with_capacity(object_count.min(cur.remaining() / 9 + 1));
        for _ in 0..object_count {
            let entry_pos = cur.pos();
            let name_len = cur.u8("name length")? as usize;
            let name_bytes = cur.bytes(name_len, "object name")?;
            let name = core::str::from_utf8(name_bytes).map_err(|_| {
                Error::new(
                    entry_pos as u64,
                    ErrorKind::Invalid,
                    "object name not UTF-8",
                )
            })?;
            let offset = cur.u32("object offset")?;
            let size = cur.u32("object size")?;
            let end = (offset as usize).checked_add(size as usize).ok_or_else(|| {
                Error::new(
                    entry_pos as u64,
                    ErrorKind::Invalid,
                    "object range outside blob or too small for a header",
                )
            })?;
            if offset as usize > blob.len() || end > blob.len() || size < 5 {
                return Err(Error::new(
                    entry_pos as u64,
                    ErrorKind::Invalid,
                    "object range outside blob or too small for a header",
                ));
            }
            objects.push(ObjectEntry {
                name,
                offset,
                size,
                data: &blob[offset as usize..end],
            });
        }

        // Relocation tables: absolute file offsets into the blob.
        let hash_count = cur.u32("hash relocation count")? as usize;
        let mut hash_offsets = Vec::with_capacity(hash_count.min(cur.remaining() / 4 + 1));
        for _ in 0..hash_count {
            let off = cur.u32("hash relocation")?;
            check_relocation(off, blob_start, data_size, cur.pos())?;
            hash_offsets.push(off);
        }
        let archive_ref_count = cur.u32("archive relocation count")? as usize;
        let mut archive_refs =
            Vec::with_capacity(archive_ref_count.min(cur.remaining() / 4 + 1));
        for _ in 0..archive_ref_count {
            let off = cur.u32("archive relocation")?;
            check_relocation(off, blob_start, data_size, cur.pos())?;
            archive_refs.push(off);
        }

        if cur.remaining() != 0 {
            return Err(Error::new(
                cur.pos() as u64,
                ErrorKind::Invalid,
                "trailing bytes after relocation tables",
            ));
        }

        Ok(MetaFile {
            buf,
            suffix,
            blob,
            archives,
            objects,
            hash_offsets,
            archive_offsets: archive_refs,
        })
    }

    /// Version suffix from the file header (11, 12, 15 or 16 in shipped files).
    #[must_use]
    pub fn suffix(&self) -> u32 {
        self.suffix
    }

    /// The raw object blob.
    #[must_use]
    pub fn blob(&self) -> &'a [u8] {
        self.blob
    }

    /// The full input the file was parsed from.
    #[must_use]
    pub fn input(&self) -> &'a [u8] {
        self.buf
    }

    /// Archive-name table entries in file order.
    #[must_use]
    pub fn archives(&self) -> &[Archive] {
        &self.archives
    }

    /// Object directory entries in file order.
    #[must_use]
    pub fn objects(&self) -> &[ObjectEntry<'a>] {
        &self.objects
    }

    /// Find an object by exact name.
    #[must_use]
    pub fn find(&self, name: &str) -> Option<&ObjectEntry<'a>> {
        self.objects.iter().find(|o| o.name == name)
    }

    /// Absolute file offsets of hashed cross-reference fields.
    #[must_use]
    pub fn hash_offsets(&self) -> &[u32] {
        &self.hash_offsets
    }

    /// Absolute file offsets of hashed archive-reference fields.
    #[must_use]
    pub fn archive_offsets(&self) -> &[u32] {
        &self.archive_offsets
    }
}

fn check_relocation(
    off: u32,
    blob_start: usize,
    data_size: usize,
    pos: usize,
) -> Result<(), Error> {
    let off = off as usize;
    if off < blob_start || off + 4 > blob_start + data_size {
        return Err(Error::new(
            pos as u64,
            ErrorKind::Invalid,
            "relocation offset outside object blob",
        ));
    }
    Ok(())
}

fn heap_string(heap: &[u8], off: usize, base: usize) -> Result<String, Error> {
    if off >= heap.len() {
        return Err(Error::new(
            (base + off) as u64,
            ErrorKind::Invalid,
            "archive offset outside name heap",
        ));
    }
    let end = heap[off..].iter().position(|b| *b == 0).ok_or_else(|| {
        Error::new(
            (base + off) as u64,
            ErrorKind::Invalid,
            "unterminated archive name",
        )
    })?;
    let bytes = &heap[off..off + end];
    String::from_utf8(bytes.to_vec()).map_err(|_| {
        Error::new(
            (base + off) as u64,
            ErrorKind::Invalid,
            "archive name not UTF-8",
        )
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    /// Build a minimal container: suffix 11, one 8-byte object named "AB",
    /// one archive "R\\B", empty relocation tables.
    pub(crate) fn tiny_file() -> Vec<u8> {
        let mut b = Vec::new();
        b.extend_from_slice(&11u32.to_le_bytes());
        // blob: pad byte + object(type 1, nameoff 0, 2 body bytes)
        let blob = [0u8, 1, 0, 0, 0, 0, 7, 8, 9];
        b.extend_from_slice(&(blob.len() as u32).to_le_bytes());
        b.extend_from_slice(&blob);
        // archive block: size word excluded from count
        let heap = b"R\\B\0";
        let block = 4 + 4 + heap.len() as u32;
        b.extend_from_slice(&block.to_le_bytes());
        b.extend_from_slice(&1u32.to_le_bytes());
        b.extend_from_slice(&0u32.to_le_bytes());
        b.extend_from_slice(heap);
        // directory
        b.extend_from_slice(&1u32.to_le_bytes());
        b.extend_from_slice(&3u32.to_le_bytes());
        b.push(2);
        b.extend_from_slice(b"AB");
        b.extend_from_slice(&1u32.to_le_bytes());
        b.extend_from_slice(&8u32.to_le_bytes());
        // relocations: none
        b.extend_from_slice(&0u32.to_le_bytes());
        b.extend_from_slice(&0u32.to_le_bytes());
        b
    }

    #[test]
    fn parses_tiny_file() {
        let b = tiny_file();
        let f = MetaFile::parse(&b).unwrap();
        assert_eq!(f.suffix(), 11);
        assert_eq!(f.archives().len(), 1);
        assert_eq!(f.archives()[0].name(), "R\\B");
        assert_eq!(f.objects().len(), 1);
        let o = &f.objects()[0];
        assert_eq!(o.name(), "AB");
        assert_eq!(o.type_id(), 1);
        assert_eq!(o.name_offset(), 0);
        assert_eq!(o.data(), &[1, 0, 0, 0, 0, 7, 8, 9]);
        assert_eq!(f.find("AB").unwrap().offset(), 1);
        assert!(f.find("ZZ").is_none());
        assert!(f.hash_offsets().is_empty());
        assert!(f.archive_offsets().is_empty());
    }

    #[test]
    fn rejects_truncated_and_bad_input() {
        let b = tiny_file();
        assert_eq!(
            MetaFile::parse(&[]).unwrap_err().kind(),
            ErrorKind::Truncated
        );
        assert_eq!(
            MetaFile::parse(&b[..b.len() - 1]).unwrap_err().kind(),
            ErrorKind::Truncated
        );
        // object range past the blob
        let mut bad = b.clone();
        let at = bad.len() - 8 - 8;
        bad[at..at + 4].copy_from_slice(&500u32.to_le_bytes());
        assert_eq!(
            MetaFile::parse(&bad).unwrap_err().kind(),
            ErrorKind::Invalid
        );
        // trailing garbage
        let mut tail = b.clone();
        tail.push(0);
        assert_eq!(
            MetaFile::parse(&tail).unwrap_err().kind(),
            ErrorKind::Invalid
        );
    }

    #[test]
    fn relocation_outside_blob_is_invalid() {
        let mut b = tiny_file();
        // replace trailing [hash_count=0, arch_count=0] with
        // [hash_count=1, out-of-range offset, arch_count=0]
        b.truncate(b.len() - 8);
        b.extend_from_slice(&1u32.to_le_bytes());
        b.extend_from_slice(&0xFFFFu32.to_le_bytes());
        b.extend_from_slice(&0u32.to_le_bytes());
        assert_eq!(MetaFile::parse(&b).unwrap_err().kind(), ErrorKind::Invalid);
    }
}
