//! RPF version 2 and 3 archive reader.
//!
//! See the [crate-level documentation](crate) for the format layout.

use std::collections::HashMap;
use std::io::{Read, Seek, SeekFrom};

use crate::{
    Archive, Entry, EntryKind, Error, Key, ResourceInfo, Result, crypto, inflate_raw,
    read_exact_or, stream_len,
};

/// File offset of the table of contents in every RPF file.
pub const RPF_TOC_OFFSET: u64 = 0x800;

/// Size of one table record in bytes.
pub const RPF_RECORD_SIZE: usize = 16;

/// Sanity cap on the declared record count.
const MAX_RECORDS: u32 = 4_000_000;

/// Sanity cap on the declared table size in bytes.
const MAX_TOC_SIZE: u32 = 256_000_000;

/// Parsed RPF header (offsets 0x00-0x17 of the file).
#[derive(Clone, Debug)]
pub struct RpfHeader {
    /// Container version: 2 or 3.
    pub version: u8,
    /// Table-of-contents size in bytes.
    pub toc_size: u32,
    /// Number of table records (files plus directories).
    pub entry_count: u32,
    /// Header word at 0x0C. Always zero in shipped files; meaning unknown.
    pub unknown: u32,
    /// True when the table is encrypted (every shipped file).
    pub toc_encrypted: bool,
    /// True when file payloads are encrypted too (one shipped file).
    pub content_encrypted: bool,
}

/// A parsed RPF table of contents.
///
/// Entries are stored in pre-order: directories immediately precede their
/// children, matching the on-disk record order.
#[derive(Clone, Debug)]
pub struct RpfArchive {
    header: RpfHeader,
    entries: Vec<Entry>,
}

impl RpfArchive {
    /// Open an RPF archive from a reader positioned anywhere.
    ///
    /// Reads the header and the table of contents, decrypting the table
    /// when flagged (which requires `key`). Payload bytes are not read
    /// here; use [`Archive::read_raw`] or [`Archive::read_file`].
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
        let mut header = [0u8; 24];
        read_exact_or(reader, &mut header, "RPF header")?;
        let version = match &header[0..4] {
            b"RPF2" => 2,
            b"RPF3" => 3,
            _ => {
                return Err(Error::BadMagic {
                    expected: "RPF2 or RPF3",
                    found: header[0..4].to_vec(),
                });
            }
        };
        let toc_size = u32::from_le_bytes(header[4..8].try_into().unwrap());
        let entry_count = u32::from_le_bytes(header[8..12].try_into().unwrap());
        let unknown = u32::from_le_bytes(header[12..16].try_into().unwrap());
        let enc_flag = u32::from_le_bytes(header[16..20].try_into().unwrap());
        let content_tag = u32::from_le_bytes(header[20..24].try_into().unwrap());
        if entry_count > MAX_RECORDS {
            return Err(Error::TooLarge {
                what: "RPF record count",
                size: u64::from(entry_count),
            });
        }
        if toc_size > MAX_TOC_SIZE {
            return Err(Error::TooLarge {
                what: "RPF table size",
                size: u64::from(toc_size),
            });
        }
        let records_bytes = u64::from(entry_count) * RPF_RECORD_SIZE as u64;
        if u64::from(toc_size) < records_bytes {
            return Err(Error::BadEntry(format!(
                "table size {toc_size} smaller than {entry_count} records"
            )));
        }

        let file_len = stream_len(reader);
        if let Some(len) = file_len
            && RPF_TOC_OFFSET.saturating_add(u64::from(toc_size)) > len
        {
            return Err(Error::Truncated("RPF table of contents"));
        }

        reader.seek(SeekFrom::Start(RPF_TOC_OFFSET))?;
        let mut toc = vec![0u8; toc_size as usize];
        read_exact_or(reader, &mut toc, "RPF table of contents")?;

        let toc_encrypted = enc_flag != 0;
        if toc_encrypted {
            let key = key.ok_or(Error::EncryptedTable)?;
            crypto::decrypt_tables(&mut toc, key);
        }

        let entries = build_entries(version, entry_count, &toc)?;
        let header = RpfHeader {
            version,
            toc_size,
            entry_count,
            unknown,
            toc_encrypted,
            content_encrypted: content_tag != 0,
        };
        Ok(RpfArchive { header, entries })
    }

    /// The parsed file header.
    #[must_use]
    pub fn header(&self) -> &RpfHeader {
        &self.header
    }

    /// Resolve RPF3 content hashes to names from an external table.
    ///
    /// `names` maps content hashes to plain names (without directories).
    /// Entries whose hash resolves get their name and full path rebuilt;
    /// the rest keep their hash placeholder. Returns the number resolved.
    /// RPF2 archives need no resolution and always return 0.
    pub fn resolve_names(&mut self, names: &HashMap<u32, String>) -> usize {
        if self.header.version != 3 {
            return 0;
        }
        // Patch each entry's own trailing segment in place.
        let mut resolved = 0;
        for entry in &mut self.entries {
            let Some(h) = entry.hash else { continue };
            let Some(name) = names.get(&h) else { continue };
            resolved += 1;
            entry.name = Some(name.clone());
            // A directory path ends in '/'; its own segment is the one
            // before that slash, not the empty string after it.
            let own_end = if entry.kind == EntryKind::Directory && entry.path.len() > 1 {
                entry.path.trim_end_matches('/').len()
            } else {
                entry.path.len()
            };
            if let Some(pos) = entry.path[..own_end].rfind('/') {
                entry.path.truncate(pos + 1);
                entry.path.push_str(name);
                if entry.kind == EntryKind::Directory {
                    entry.path.push('/');
                }
            }
        }
        // Renamed directories change their children's path prefixes.
        let mut renamed_dirs: HashMap<String, String> = HashMap::new();
        for entry in &self.entries {
            if entry.kind == EntryKind::Directory
                && let (Some(h), Some(name)) = (entry.hash, &entry.name)
            {
                renamed_dirs.insert(format!("<hash:{h:08X}>"), name.clone());
            }
        }
        if !renamed_dirs.is_empty() {
            for entry in &mut self.entries {
                let mut segments: Vec<String> = entry.path.split('/').map(str::to_string).collect();
                let last = segments.len().saturating_sub(1);
                for (i, seg) in segments.iter_mut().enumerate() {
                    if i == last {
                        break; // own segment already final
                    }
                    if let Some(real) = renamed_dirs.get(seg.as_str()) {
                        *seg = real.clone();
                    }
                }
                entry.path = segments.join("/");
            }
        }
        resolved
    }
}

impl Archive for RpfArchive {
    fn entries(&self) -> &[Entry] {
        &self.entries
    }

    fn read_raw<R: Read + Seek>(
        &self,
        reader: &mut R,
        index: usize,
        key: Option<&Key>,
    ) -> Result<Vec<u8>> {
        let entry = self.entries.get(index).ok_or_else(|| {
            Error::BadEntry(format!(
                "entry index {index} out of range ({} entries)",
                self.entries.len()
            ))
        })?;
        if entry.kind != EntryKind::File {
            return Err(Error::BadEntry(format!(
                "entry {} is a directory",
                entry.path
            )));
        }
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
        if self.header.content_encrypted {
            let key = key.ok_or(Error::EncryptedContent)?;
            crypto::decrypt_tables(&mut buf, key);
        }
        Ok(buf)
    }

    fn read_file<R: Read + Seek>(
        &self,
        reader: &mut R,
        index: usize,
        key: Option<&Key>,
    ) -> Result<Vec<u8>> {
        let raw = self.read_raw(reader, index, key)?;
        let entry = self.entries.get(index).ok_or_else(|| {
            Error::BadEntry(format!(
                "entry index {index} out of range ({} entries)",
                self.entries.len()
            ))
        })?;
        if entry.compressed {
            let out = inflate_raw(&raw, entry.size)?;
            if out.len() as u64 != entry.size {
                return Err(Error::Decompress(format!(
                    "size mismatch for {}: got {}, table says {}",
                    entry.path,
                    out.len(),
                    entry.size
                )));
            }
            Ok(out)
        } else {
            Ok(raw)
        }
    }
}

/// Raw decoded record before path assignment.
enum Record {
    Dir {
        name_ref: u32,
        first_child: usize,
        child_count: usize,
    },
    File {
        name_ref: u32,
        size: u64,
        offset: u64,
        stored_size: u64,
        compressed: bool,
        resource: Option<ResourceInfo>,
    },
}

/// Deepest directory nesting accepted. Shipped paths are a handful of
/// levels deep; the cap keeps a hostile table from exhausting the stack.
const MAX_DEPTH: usize = 256;

// Assign paths depth-first. Records are pre-order on disk, so
// children always follow their directory. Each record is visited at most
// once: a hostile table whose directory ranges overlap or contain their own
// directory would otherwise recurse forever (found by the mutation fuzzer).
#[allow(clippy::too_many_arguments)]
fn walk(
    records: &[Record],
    record: usize,
    dir_path: &str,
    version: u8,
    lookup_name: &dyn Fn(u32, usize) -> Result<String>,
    entries: &mut Vec<Entry>,
    index_of: &mut [Option<usize>],
    depth: usize,
) -> Result<()> {
    if index_of[record].is_some() {
        return Ok(());
    }
    if depth > MAX_DEPTH {
        return Err(Error::BadTree(format!(
            "directory nesting deeper than {MAX_DEPTH} at record {record}"
        )));
    }
    match &records[record] {
        Record::Dir {
            name_ref,
            first_child,
            child_count,
        } => {
            let (name, hash, segment) = if version == 3 {
                (None, Some(*name_ref), format!("<hash:{name_ref:08X}>"))
            } else {
                let n = lookup_name(*name_ref, record)?;
                let segment = if n == "/" { String::new() } else { n.clone() };
                (Some(n), None, segment)
            };
            let path = if dir_path == "/" || dir_path.is_empty() {
                if segment.is_empty() {
                    "/".to_string()
                } else {
                    format!("/{segment}")
                }
            } else if segment.is_empty() {
                dir_path.to_string()
            } else {
                format!("{dir_path}/{segment}")
            };
            index_of[record] = Some(entries.len());
            entries.push(Entry {
                path: if path == "/" {
                    "/".to_string()
                } else {
                    format!("{path}/")
                },
                name,
                hash,
                kind: EntryKind::Directory,
                size: 0,
                stored_size: 0,
                offset: 0,
                compressed: false,
                resource: None,
            });
            let child_base = path.clone();
            for child in *first_child..*first_child + *child_count {
                walk(
                    records,
                    child,
                    &child_base,
                    version,
                    lookup_name,
                    entries,
                    index_of,
                    depth + 1,
                )?;
            }
            Ok(())
        }
        Record::File {
            name_ref,
            size,
            offset,
            stored_size,
            compressed,
            resource,
        } => {
            let (name, hash, segment) = if version == 3 {
                (None, Some(*name_ref), format!("<hash:{name_ref:08X}>"))
            } else {
                let n = lookup_name(*name_ref, record)?;
                (Some(n.clone()), None, n)
            };
            let path = if dir_path == "/" || dir_path.is_empty() {
                format!("/{segment}")
            } else {
                format!("{dir_path}/{segment}")
            };
            index_of[record] = Some(entries.len());
            entries.push(Entry {
                path,
                name,
                hash,
                kind: EntryKind::File,
                size: *size,
                stored_size: *stored_size,
                offset: *offset,
                compressed: *compressed,
                resource: *resource,
            });
            Ok(())
        }
    }
}

// Table decode with per-record validation; splitting would scatter record handling.
#[allow(clippy::too_many_lines)]
fn build_entries(version: u8, entry_count: u32, toc: &[u8]) -> Result<Vec<Entry>> {
    let count = entry_count as usize;
    let records_end = count * RPF_RECORD_SIZE;
    let name_block = toc
        .get(records_end..)
        .ok_or(Error::Truncated("RPF name block"))?;

    let mut records = Vec::with_capacity(count);
    for i in 0..count {
        let rec = &toc[i * RPF_RECORD_SIZE..(i + 1) * RPF_RECORD_SIZE];
        let marker = i32::from_le_bytes(rec[8..12].try_into().unwrap());
        if marker < 0 {
            let name_ref = u32::from_le_bytes(rec[0..4].try_into().unwrap());
            let first_child =
                (u32::from_le_bytes(rec[8..12].try_into().unwrap()) & 0x7FFF_FFFF) as usize;
            let child_count =
                usize::try_from(i32::from_le_bytes(rec[12..16].try_into().unwrap()) & 0x0FFF_FFFF)
                    .map_err(|_| {
                        Error::BadEntry(format!("directory record {i} bad child count"))
                    })?;
            if first_child.saturating_add(child_count) > count {
                return Err(Error::BadEntry(format!(
                    "directory record {i} range {first_child}+{child_count} exceeds {count}"
                )));
            }
            records.push(Record::Dir {
                name_ref,
                first_child,
                child_count,
            });
        } else {
            let name_ref = u32::from_le_bytes(rec[0..4].try_into().unwrap());
            let size = i32::from_le_bytes(rec[4..8].try_into().unwrap());
            let offset_raw = i32::from_le_bytes(rec[8..12].try_into().unwrap());
            let control = u32::from_le_bytes(rec[12..16].try_into().unwrap());
            if size < 0 {
                return Err(Error::BadEntry(format!(
                    "file record {i} has negative size"
                )));
            }
            let (offset, stored_size, compressed, resource) =
                if control & 0xC000_0000 == 0xC000_0000 {
                    let raw_bits = u32::from_ne_bytes(offset_raw.to_ne_bytes());
                    let offset = u64::from(raw_bits & 0x7FFF_FF00);
                    let type_id = i32::from(rec[8]);
                    (
                        offset,
                        u64::try_from(size).map_err(|_| {
                            Error::BadEntry(format!("file record {i} has negative size"))
                        })?,
                        false,
                        Some(ResourceInfo {
                            type_id,
                            flags: control,
                        }),
                    )
                } else {
                    let stored = u64::from(control & 0xBFFF_FFFF);
                    let compressed = control & 0x4000_0000 != 0;
                    if offset_raw < 0 {
                        return Err(Error::BadEntry(format!(
                            "file record {i} has negative offset"
                        )));
                    }
                    (
                        u64::try_from(offset_raw).map_err(|_| {
                            Error::BadEntry(format!("file record {i} has negative offset"))
                        })?,
                        stored,
                        compressed,
                        None,
                    )
                };
            records.push(Record::File {
                name_ref,
                size: u64::try_from(size)
                    .map_err(|_| Error::BadEntry(format!("file record {i} has negative size")))?,
                offset,
                stored_size,
                compressed,
                resource,
            });
        }
    }

    // Name lookup for RPF2: offsets into the name block.
    let lookup_name = |name_ref: u32, record: usize| -> Result<String> {
        let start = name_ref as usize;
        if start >= name_block.len() {
            return Err(Error::BadEntry(format!(
                "record {record} name offset {start} outside name block of {} bytes",
                name_block.len()
            )));
        }
        let end = name_block[start..]
            .iter()
            .position(|&b| b == 0)
            .map_or(name_block.len(), |p| start + p);
        Ok(String::from_utf8_lossy(&name_block[start..end]).into_owned())
    };

    // Find root directories: directory records not covered by any other
    // directory's child range.
    let mut covered = vec![false; count];
    for record in &records {
        if let Record::Dir {
            first_child,
            child_count,
            ..
        } = record
        {
            for slot in covered
                .iter_mut()
                .take(first_child + child_count)
                .skip(*first_child)
            {
                *slot = true;
            }
        }
    }

    // Walk the tree depth-first, assigning paths. Records are pre-order on
    // disk, so children always follow their directory.
    let mut entries: Vec<Entry> = Vec::with_capacity(count);
    let mut index_of: Vec<Option<usize>> = vec![None; count];

    // Shipped files nest directories inside the root's range; walk every
    // uncovered record so orphans still appear.
    for (record, is_covered) in covered.iter().enumerate() {
        if !is_covered {
            let base = String::new();
            walk(
                &records,
                record,
                &base,
                version,
                &lookup_name,
                &mut entries,
                &mut index_of,
                0,
            )?;
        }
    }
    // Any record the tree did not reach (a directory range that skips
    // records) is appended at top level rather than dropped.
    for record in 0..count {
        if index_of[record].is_none() {
            let base = String::new();
            walk(
                &records,
                record,
                &base,
                version,
                &lookup_name,
                &mut entries,
                &mut index_of,
                0,
            )?;
        }
    }
    Ok(entries)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    /// Hand-built RPF2 fixture (unencrypted). Layout, all written by this
    /// test, no game bytes:
    ///
    /// ```text
    /// record 0: dir  "/"        children 1..3
    /// record 1: dir  "data"     children 3..5
    /// record 2: file "model.wdr" resource, type 0x6E
    /// record 3: file "note.txt" stored "hello"
    /// record 4: file "blob.bin" raw-deflate of 300 'A's
    /// ```
    fn fixture_rpf2() -> Vec<u8> {
        let mut toc_records: Vec<u8> = Vec::new();
        // name block layout: "/"@0 "data"@2 "note.txt"@7 "blob.bin"@16 "model.wdr"@25
        let names = b"/\x00data\x00note.txt\x00blob.bin\x00model.wdr\x00";
        assert_eq!(names.len(), 35);
        let mut rec = |a: u32, b: u32, c: u32, d: u32| {
            toc_records.extend_from_slice(&a.to_le_bytes());
            toc_records.extend_from_slice(&b.to_le_bytes());
            toc_records.extend_from_slice(&c.to_le_bytes());
            toc_records.extend_from_slice(&d.to_le_bytes());
        };
        // dir: name_ref, unknown, 0x80000000|index, count
        rec(0, 0, 0x8000_0001, 2); // "/" -> records 1..3
        rec(2, 0, 0x8000_0003, 2); // "data" -> records 3..5
        // model.wdr: resource type 0x6E at 0x3000 (record 2)
        rec(25, 64, 0x3000 | 0x6E, 0xC000_0000);
        // file: name_ref, size, offset, control(stored size, stored)
        rec(7, 5, 0x1000, 5); // note.txt (record 3)
        // blob.bin: compressed; payload filled below (record 4)
        rec(16, 300, 0x2000, 0x4000_0000 | 9); // placeholder, fixed below

        let mut toc = toc_records;
        toc.extend_from_slice(names);
        while !toc.len().is_multiple_of(16) {
            toc.push(0);
        }

        let plain = vec![b'A'; 300];
        let mut enc =
            flate2::write::DeflateEncoder::new(Vec::new(), flate2::Compression::default());
        use std::io::Write;
        enc.write_all(&plain).unwrap();
        let deflated = enc.finish().unwrap();
        // Patch blob.bin control word with the real stored size.
        let off = 4 * 16 + 12;
        let control = 0x4000_0000 | (deflated.len() as u32);
        toc[off..off + 4].copy_from_slice(&control.to_le_bytes());

        let mut file = vec![0u8; RPF_TOC_OFFSET as usize];
        file[0..4].copy_from_slice(b"RPF2");
        file[4..8].copy_from_slice(&(toc.len() as u32).to_le_bytes());
        file[8..12].copy_from_slice(&5u32.to_le_bytes());
        // 0x0C unknown = 0, 0x10 enc flag = 0, 0x14 content tag = 0.
        file.extend_from_slice(&toc);
        // Pad to payload area and write payloads.
        file.resize(0x1000, 0);
        file.extend_from_slice(b"hello");
        file.resize(0x2000, 0);
        file.extend_from_slice(&deflated);
        file.resize(0x3000, 0);
        file.extend_from_slice(&[0x52u8; 64]); // fake resource payload
        file
    }

    #[test]
    fn open_rpf2_tree_and_reads() {
        let bytes = fixture_rpf2();
        let mut cursor = Cursor::new(bytes);
        let archive = RpfArchive::open(&mut cursor, None).unwrap();
        assert_eq!(archive.header().version, 2);
        assert!(!archive.header().toc_encrypted);
        let paths: Vec<&str> = archive.entries().iter().map(|e| e.path.as_str()).collect();
        assert_eq!(
            paths,
            vec![
                "/",
                "/data/",
                "/data/note.txt",
                "/data/blob.bin",
                "/model.wdr"
            ]
        );

        let note = archive.find("/data/note.txt").unwrap();
        assert_eq!(
            archive.read_file(&mut cursor, note, None).unwrap(),
            b"hello"
        );

        let blob = archive.find("/data/blob.bin").unwrap();
        assert!(archive.entries()[blob].compressed);
        assert_eq!(
            archive.read_file(&mut cursor, blob, None).unwrap(),
            vec![b'A'; 300]
        );

        let model = archive.find("/model.wdr").unwrap();
        let res = archive.entries()[model].resource.unwrap();
        assert_eq!(res.type_id, 0x6E);
        assert_eq!(archive.entries()[model].offset, 0x3000);

        // Reading a directory is an error, not a panic.
        assert!(archive.read_file(&mut cursor, 0, None).is_err());
        // Out-of-range index is an error, not a panic.
        assert!(archive.read_file(&mut cursor, 99, None).is_err());
    }

    #[test]
    fn rejects_bad_magic() {
        let mut bytes = fixture_rpf2();
        bytes[0] = b'X';
        let mut cursor = Cursor::new(bytes);
        assert!(matches!(
            RpfArchive::open(&mut cursor, None),
            Err(Error::BadMagic { .. })
        ));
    }

    #[test]
    fn rejects_truncated_table() {
        let bytes = fixture_rpf2();
        // Cut mid-table: the header promises more table than the file holds.
        let mut cursor = Cursor::new(bytes[..0x810].to_vec());
        assert!(RpfArchive::open(&mut cursor, None).is_err());
    }

    #[test]
    fn encrypted_table_needs_key() {
        let mut bytes = fixture_rpf2();
        bytes[0x10] = 1; // set encryption flag
        let mut cursor = Cursor::new(bytes);
        assert!(matches!(
            RpfArchive::open(&mut cursor, None),
            Err(Error::EncryptedTable)
        ));
    }

    /// Hand-built RPF3 fixture: root dir (filler hash), one file by hash.
    #[test]
    fn open_rpf3_hashes_and_resolve() {
        let hash = crate::hash::name_hash("example_clip");
        let mut toc: Vec<u8> = Vec::new();
        // dir record: hash filler, unknown, index|0x80000000, count
        toc.extend_from_slice(&0xBAAD_F00Du32.to_le_bytes());
        toc.extend_from_slice(&1u32.to_le_bytes());
        toc.extend_from_slice(&0x8000_0001u32.to_le_bytes());
        toc.extend_from_slice(&1u32.to_le_bytes());
        // file record: hash, size, offset, control
        toc.extend_from_slice(&hash.to_le_bytes());
        toc.extend_from_slice(&4u32.to_le_bytes());
        toc.extend_from_slice(&0x1000u32.to_le_bytes());
        toc.extend_from_slice(&4u32.to_le_bytes());
        toc.extend_from_slice(b"/\x00");
        while !toc.len().is_multiple_of(16) {
            toc.push(0);
        }

        let mut file = vec![0u8; RPF_TOC_OFFSET as usize];
        file[0..4].copy_from_slice(b"RPF3");
        file[4..8].copy_from_slice(&(toc.len() as u32).to_le_bytes());
        file[8..12].copy_from_slice(&2u32.to_le_bytes());
        file.extend_from_slice(&toc);
        file.resize(0x1000, 0);
        file.extend_from_slice(b"data");

        let mut cursor = Cursor::new(file);
        let mut archive = RpfArchive::open(&mut cursor, None).unwrap();
        assert_eq!(archive.header().version, 3);
        assert_eq!(archive.entries()[1].hash, Some(hash));
        assert!(archive.entries()[1].name.is_none());
        assert!(archive.entries()[1].path.contains("<hash:"));

        let mut table = HashMap::new();
        table.insert(hash, "example_clip".to_string());
        assert_eq!(archive.resolve_names(&table), 1);
        assert_eq!(archive.entries()[1].path, "/<hash:BAADF00D>/example_clip");
        assert_eq!(archive.read_file(&mut cursor, 1, None).unwrap(), b"data");
    }

    #[test]
    fn rpf3_encrypted_round_trip() {
        // Same fixture with the table encrypted by the test key.
        let key: Key = [0x5Au8; 32];
        let hash = 0x1234_5678u32;
        let mut toc: Vec<u8> = Vec::new();
        toc.extend_from_slice(&0xBAAD_F00Du32.to_le_bytes());
        toc.extend_from_slice(&1u32.to_le_bytes());
        toc.extend_from_slice(&0x8000_0001u32.to_le_bytes());
        toc.extend_from_slice(&1u32.to_le_bytes());
        toc.extend_from_slice(&hash.to_le_bytes());
        toc.extend_from_slice(&4u32.to_le_bytes());
        toc.extend_from_slice(&0x1000u32.to_le_bytes());
        toc.extend_from_slice(&4u32.to_le_bytes());
        toc.extend_from_slice(b"/\x00");
        while !toc.len().is_multiple_of(16) {
            toc.push(0);
        }
        crypto::encrypt_tables(&mut toc, &key);

        let mut file = vec![0u8; RPF_TOC_OFFSET as usize];
        file[0..4].copy_from_slice(b"RPF3");
        file[4..8].copy_from_slice(&(toc.len() as u32).to_le_bytes());
        file[8..12].copy_from_slice(&2u32.to_le_bytes());
        file[16] = 1; // encrypted
        file.extend_from_slice(&toc);
        file.resize(0x1000, 0);
        file.extend_from_slice(b"data");

        let mut cursor = Cursor::new(file);
        let archive = RpfArchive::open(&mut cursor, Some(&key)).unwrap();
        assert_eq!(archive.entries()[1].hash, Some(hash));
        assert_eq!(
            archive.read_file(&mut cursor, 1, Some(&key)).unwrap(),
            b"data"
        );
    }
}
