//! IMG version 3 archive reader.
//!
//! See the [crate-level documentation](crate) for the format layout.

use std::io::{Read, Seek, SeekFrom};

use crate::{
    Archive, Entry, EntryKind, Error, Key, ResourceInfo, Result, crypto, read_exact_or, stream_len,
};

/// IMG magic word (first header word of an unencrypted file).
pub const IMG_MAGIC: u32 = 0xA94E_2A52;

/// The only IMG version the game ships.
pub const IMG_VERSION: u32 = 3;

/// Size of one table record in bytes.
pub const IMG_RECORD_SIZE: usize = 16;

/// Size of the file header in bytes.
pub const IMG_HEADER_SIZE: usize = 20;

/// Payload block size in bytes.
pub const IMG_BLOCK_SIZE: u64 = 0x800;

/// Sanity cap on the declared entry count.
const MAX_ENTRIES: u32 = 4_000_000;

/// Sanity cap on the declared table size in bytes.
const MAX_TABLE_SIZE: u32 = 512_000_000;

/// Parsed IMG header (offsets 0x00-0x13 of the file).
#[derive(Clone, Debug)]
pub struct ImgHeader {
    /// Number of entries in the table.
    pub entry_count: u32,
    /// Table size in bytes (records plus the name area).
    pub table_size: u32,
    /// Record size; always 16 in shipped files.
    pub item_size: u16,
    /// Header word at 0x12. Meaning unknown.
    pub unknown: u16,
    /// True when the header and table were encrypted.
    pub encrypted: bool,
}

/// A parsed IMG table of contents (flat: no directories).
#[derive(Clone, Debug)]
pub struct ImgArchive {
    header: ImgHeader,
    entries: Vec<Entry>,
}

impl ImgArchive {
    /// Open an IMG archive from a reader positioned anywhere.
    ///
    /// Reads the header and table, decrypting both when the magic shows
    /// the file is encrypted (which requires `key`). Payload bytes are
    /// not read here; use [`Archive::read_raw`] or [`Archive::read_file`].
    ///
    /// # Errors
    ///
    /// Returns an error if the input is truncated or malformed.
    ///
    /// # Panics
    ///
    /// Never panics: header reads use fixed-size ranges of a stack buffer, and table walks are range-checked.
    pub fn open<R: Read + Seek>(reader: &mut R, key: Option<&Key>) -> Result<Self> {
        reader.seek(SeekFrom::Start(0))?;
        let mut header = [0u8; IMG_HEADER_SIZE];
        read_exact_or(reader, &mut header, "IMG header")?;

        let encrypted = u32::from_le_bytes(header[0..4].try_into().unwrap()) != IMG_MAGIC;
        if encrypted {
            let key = key.ok_or(Error::EncryptedTable)?;
            // Only the first 16 header bytes are encrypted; bytes 16-19
            // (record size and the unknown word) stay plaintext.
            crypto::decrypt_tables(&mut header[0..16], key);
        }
        let magic = u32::from_le_bytes(header[0..4].try_into().unwrap());
        if magic != IMG_MAGIC {
            return Err(Error::BadMagic {
                expected: "IMG v3",
                found: header[0..4].to_vec(),
            });
        }
        let version = u32::from_le_bytes(header[4..8].try_into().unwrap());
        if version != IMG_VERSION {
            return Err(Error::UnsupportedVersion {
                format: "IMG",
                version,
            });
        }
        let entry_count = u32::from_le_bytes(header[8..12].try_into().unwrap());
        let table_size = u32::from_le_bytes(header[12..16].try_into().unwrap());
        let item_size = u16::from_le_bytes(header[16..18].try_into().unwrap());
        let unknown = u16::from_le_bytes(header[18..20].try_into().unwrap());
        if item_size as usize != IMG_RECORD_SIZE {
            return Err(Error::BadEntry(format!(
                "IMG record size {item_size}, expected {IMG_RECORD_SIZE}"
            )));
        }
        if entry_count > MAX_ENTRIES {
            return Err(Error::TooLarge {
                what: "IMG entry count",
                size: u64::from(entry_count),
            });
        }
        if table_size > MAX_TABLE_SIZE {
            return Err(Error::TooLarge {
                what: "IMG table size",
                size: u64::from(table_size),
            });
        }
        let records_bytes = u64::from(entry_count) * IMG_RECORD_SIZE as u64;
        if u64::from(table_size) < records_bytes {
            return Err(Error::BadEntry(format!(
                "table size {table_size} smaller than {entry_count} records"
            )));
        }

        let file_len = stream_len(reader);
        if let Some(len) = file_len
            && IMG_HEADER_SIZE as u64 + u64::from(table_size) > len
        {
            return Err(Error::Truncated("IMG table"));
        }

        reader.seek(SeekFrom::Start(IMG_HEADER_SIZE as u64))?;
        let mut table = vec![0u8; table_size as usize];
        read_exact_or(reader, &mut table, "IMG table")?;
        if encrypted {
            let key = key.ok_or(Error::EncryptedTable)?;
            crypto::decrypt_tables(&mut table, key);
        }

        let entries = build_entries(entry_count, &table)?;
        let header = ImgHeader {
            entry_count,
            table_size,
            item_size,
            unknown,
            encrypted,
        };
        Ok(ImgArchive { header, entries })
    }

    /// The parsed file header.
    #[must_use]
    pub fn header(&self) -> &ImgHeader {
        &self.header
    }
}

impl Archive for ImgArchive {
    fn entries(&self) -> &[Entry] {
        &self.entries
    }

    fn read_raw<R: Read + Seek>(
        &self,
        reader: &mut R,
        index: usize,
        _key: Option<&Key>,
    ) -> Result<Vec<u8>> {
        let entry = self.entries.get(index).ok_or_else(|| {
            Error::BadEntry(format!(
                "entry index {index} out of range ({} entries)",
                self.entries.len()
            ))
        })?;
        if let Some(len) = stream_len(reader)
            && entry.offset.saturating_add(entry.stored_size) > len
        {
            return Err(Error::EntryOutOfRange {
                path: entry.path.clone(),
            });
        }
        reader.seek(SeekFrom::Start(entry.offset))?;
        let stored = usize::try_from(entry.stored_size).map_err(|_| Error::TooLarge {
            what: "entry payload",
            size: entry.stored_size,
        })?;
        let mut buf = vec![0u8; stored];
        read_exact_or(reader, &mut buf, "entry payload")?;
        Ok(buf)
    }

    fn read_file<R: Read + Seek>(
        &self,
        reader: &mut R,
        index: usize,
        key: Option<&Key>,
    ) -> Result<Vec<u8>> {
        // IMG payloads are always stored; resource payloads carry their own
        // internal compression (the RSC container), which is not the
        // archive's business to decode.
        self.read_raw(reader, index, key)
    }
}

fn build_entries(entry_count: u32, table: &[u8]) -> Result<Vec<Entry>> {
    let count = entry_count as usize;
    let records_end = count * IMG_RECORD_SIZE;
    let name_area = table
        .get(records_end..)
        .ok_or(Error::Truncated("IMG name area"))?;

    // Names are sequential null-terminated strings, one per entry in order.
    let mut names: Vec<String> = Vec::with_capacity(count);
    let mut pos = 0;
    for _ in 0..count {
        if pos > name_area.len() {
            return Err(Error::Truncated("IMG names"));
        }
        let end = name_area[pos..]
            .iter()
            .position(|&b| b == 0)
            .map_or(name_area.len(), |p| pos + p);
        names.push(String::from_utf8_lossy(&name_area[pos..end]).into_owned());
        pos = end + 1;
    }

    let mut entries = Vec::with_capacity(count);
    for (i, name) in names.iter().enumerate() {
        let rec = &table[i * IMG_RECORD_SIZE..(i + 1) * IMG_RECORD_SIZE];
        let w0 = u32::from_le_bytes(rec[0..4].try_into().unwrap());
        let type_id = i32::from_le_bytes(rec[4..8].try_into().unwrap());
        let block_offset = i32::from_le_bytes(rec[8..12].try_into().unwrap());
        let used_blocks = u16::from_le_bytes(rec[12..14].try_into().unwrap());
        let flags = u16::from_le_bytes(rec[14..16].try_into().unwrap());
        if block_offset < 0 {
            return Err(Error::BadEntry(format!(
                "entry {i} ({name}) has negative offset"
            )));
        }
        let offset = u64::try_from(block_offset)
            .map_err(|_| Error::BadEntry(format!("entry {i} ({name}) has negative offset")))?
            * IMG_BLOCK_SIZE;
        let (size, resource) = if w0 & 0xC000_0000 != 0 {
            let padding = u64::from(flags & 0x07FF);
            let size = u64::from(used_blocks) * IMG_BLOCK_SIZE - padding;
            (size, Some(ResourceInfo { type_id, flags: w0 }))
        } else {
            (u64::from(w0), None)
        };
        entries.push(Entry {
            path: format!("/{name}"),
            name: Some(name.clone()),
            hash: None,
            kind: EntryKind::File,
            size,
            stored_size: size,
            offset,
            compressed: false,
            resource,
        });
    }
    Ok(entries)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    /// Hand-built IMG v3 fixture (unencrypted): one plain entry holding
    /// "hello" and one resource entry (type 1, 1 block, 8 bytes padding).
    fn fixture_img(open: bool) -> (Vec<u8>, Key) {
        let key: Key = [0x3Cu8; 32];
        let mut table: Vec<u8> = Vec::new();
        // plain: w0=size, rtype, block, used, flags
        table.extend_from_slice(&5u32.to_le_bytes());
        table.extend_from_slice(&0i32.to_le_bytes());
        table.extend_from_slice(&2i32.to_le_bytes()); // block 2 -> 0x1000
        table.extend_from_slice(&1u16.to_le_bytes());
        table.extend_from_slice(&0u16.to_le_bytes());
        // resource: w0 flags with top bit, type 1, block 3, 1 block, pad 8
        table.extend_from_slice(&0x8000_0041u32.to_le_bytes());
        table.extend_from_slice(&1i32.to_le_bytes());
        table.extend_from_slice(&3i32.to_le_bytes()); // block 3 -> 0x1800
        table.extend_from_slice(&1u16.to_le_bytes());
        table.extend_from_slice(&8u16.to_le_bytes());
        table.extend_from_slice(b"note.txt\x00model.wdr\x00");

        let mut header = [0u8; IMG_HEADER_SIZE];
        header[0..4].copy_from_slice(&IMG_MAGIC.to_le_bytes());
        header[4..8].copy_from_slice(&IMG_VERSION.to_le_bytes());
        header[8..12].copy_from_slice(&2u32.to_le_bytes());
        header[12..16].copy_from_slice(&(table.len() as u32).to_le_bytes());
        header[16..18].copy_from_slice(&16u16.to_le_bytes());
        header[18..20].copy_from_slice(&233u16.to_le_bytes());

        let mut file = header.to_vec();
        if !open {
            crypto::encrypt_tables(&mut file[0..16], &key);
            crypto::encrypt_tables(&mut table, &key);
        }
        file.extend_from_slice(&table);
        // Payloads at block-aligned offsets (table length may not be a
        // multiple of the block size; pad the file, not the table).
        file.resize(0x1000, 0);
        file.extend_from_slice(b"hello");
        file.resize(0x1800, 0);
        file.extend_from_slice(&[0x52u8; 0x800 - 8]);
        (file, key)
    }

    #[test]
    fn open_img_plain_and_resource() {
        let (bytes, _) = fixture_img(true);
        let mut cursor = Cursor::new(bytes);
        let archive = ImgArchive::open(&mut cursor, None).unwrap();
        assert!(!archive.header().encrypted);
        assert_eq!(archive.len(), 2);

        let note = &archive.entries()[0];
        assert_eq!(note.path, "/note.txt");
        assert_eq!(note.size, 5);
        assert!(note.resource.is_none());
        assert_eq!(archive.read_file(&mut cursor, 0, None).unwrap(), b"hello");

        let model = &archive.entries()[1];
        assert_eq!(model.resource.unwrap().type_id, 1);
        assert_eq!(model.size, 0x800 - 8);
        assert_eq!(
            archive.read_file(&mut cursor, 1, None).unwrap().len(),
            0x800 - 8
        );
    }

    #[test]
    fn open_img_encrypted_round_trip() {
        let (bytes, key) = fixture_img(false);
        // Without the key the magic is unreadable.
        let mut cursor = Cursor::new(bytes.clone());
        assert!(matches!(
            ImgArchive::open(&mut cursor, None),
            Err(Error::EncryptedTable)
        ));
        let mut cursor = Cursor::new(bytes);
        let archive = ImgArchive::open(&mut cursor, Some(&key)).unwrap();
        assert!(archive.header().encrypted);
        assert_eq!(archive.len(), 2);
        assert_eq!(archive.entries()[0].path, "/note.txt");
        assert_eq!(
            archive.read_file(&mut cursor, 0, Some(&key)).unwrap(),
            b"hello"
        );
    }

    #[test]
    fn rejects_bad_magic() {
        let (mut bytes, _) = fixture_img(true);
        bytes[0] = b'X';
        // An undecryptable first word without a key is "encrypted, no key".
        let mut cursor = Cursor::new(bytes);
        assert!(ImgArchive::open(&mut cursor, None).is_err());
    }

    #[test]
    fn empty_img() {
        let mut header = [0u8; IMG_HEADER_SIZE];
        header[0..4].copy_from_slice(&IMG_MAGIC.to_le_bytes());
        header[4..8].copy_from_slice(&IMG_VERSION.to_le_bytes());
        header[16..18].copy_from_slice(&16u16.to_le_bytes());
        let mut cursor = Cursor::new(header.to_vec());
        let archive = ImgArchive::open(&mut cursor, None).unwrap();
        assert!(archive.is_empty());
    }
}
