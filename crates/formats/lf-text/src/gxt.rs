//! Parser for the GXT localisation text database.
//!
//! See the crate documentation for the on-disk layout.

use std::fmt;
use std::str::Utf8Error;

use crate::hash::label_hash;

/// A parsed GXT file: one language's text database.
#[derive(Debug, Clone)]
pub struct GxtFile {
    version: u16,
    bits_per_char: u16,
    tables: Vec<GxtTable>,
}

/// One named table of key/string pairs.
#[derive(Debug, Clone)]
pub struct GxtTable {
    /// Table name as stored (upper-case ASCII, up to 8 bytes).
    pub name: String,
    /// Entries in file order (string order, not hash order).
    pub entries: Vec<GxtEntry>,
}

/// One keyed string: a label hash plus the raw glyph codes.
///
/// The codes are the game's font indices, not Unicode. The terminating zero
/// unit is not included. See [`decode_western_lossy`] for rendering.
#[derive(Debug, Clone)]
pub struct GxtEntry {
    /// [`label_hash`] of the text label.
    pub hash: u32,
    /// Raw 16-bit glyph codes without the terminator.
    pub text: Vec<u16>,
}

/// Error describing why a GXT buffer could not be parsed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GxtError {
    /// The buffer ends before the structure does.
    Truncated {
        /// Byte offset that could not be read.
        offset: usize,
    },
    /// A tag word is not the expected value.
    BadTag {
        /// Byte offset of the tag.
        offset: usize,
        /// Expected tag.
        expected: [u8; 4],
        /// Bytes found (zero padded when truncated).
        found: [u8; 4],
    },
    /// An offset or size points outside the buffer.
    OutOfRange {
        /// Byte offset of the offending field.
        offset: usize,
    },
    /// A string has no terminator before the end of its data block.
    Unterminated {
        /// Table index of the entry.
        table: usize,
        /// Entry index within the table.
        entry: usize,
    },
    /// A table name is not valid ASCII.
    BadTableName {
        /// Table index.
        table: usize,
    },
    /// The header requests an unknown text unit width.
    BadUnitWidth {
        /// Value of the bits-per-character field.
        bits: u16,
    },
}

impl fmt::Display for GxtError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Truncated { offset } => write!(f, "truncated at offset {offset}"),
            Self::BadTag {
                offset,
                expected,
                found,
            } => write!(
                f,
                "bad tag at offset {offset}: expected {:?}, found {:?}",
                String::from_utf8_lossy(expected),
                String::from_utf8_lossy(found)
            ),
            Self::OutOfRange { offset } => {
                write!(f, "offset or size out of range at {offset}")
            }
            Self::Unterminated { table, entry } => {
                write!(f, "entry {entry} of table {table} is not terminated")
            }
            Self::BadTableName { table } => {
                write!(f, "table {table} has a non-ASCII name")
            }
            Self::BadUnitWidth { bits } => {
                write!(f, "unsupported bits-per-character value {bits}")
            }
        }
    }
}

impl std::error::Error for GxtError {}

impl GxtFile {
    /// Parse a GXT file from a byte slice.
    ///
    /// Accepts any format version but only 8- and 16-bit text units; every
    /// shipped file uses version 4 with 16-bit units.
    ///
    /// # Errors
    ///
    /// Returns an error if the input is truncated or malformed.
    pub fn parse(data: &[u8]) -> Result<Self, GxtError> {
        let header = data.get(0..12).ok_or(GxtError::Truncated { offset: 0 })?;
        let version = u16::from_le_bytes([header[0], header[1]]);
        let bits = u16::from_le_bytes([header[2], header[3]]);
        if bits != 8 && bits != 16 {
            return Err(GxtError::BadUnitWidth { bits });
        }
        expect_tag(data, 4, *b"TABL")?;
        let dir_size = read_u32(data, 8)? as usize;
        if !dir_size.is_multiple_of(12) {
            return Err(GxtError::OutOfRange { offset: 8 });
        }
        let table_count = dir_size / 12;
        let mut tables = Vec::with_capacity(table_count.min(data.len() / 12 + 1));
        for index in 0..table_count {
            let base = 12usize.saturating_add(index.saturating_mul(12));
            let raw = data
                .get(base..base.saturating_add(12))
                .ok_or(GxtError::Truncated { offset: base })?;
            let name_len = raw[0..8].iter().position(|b| *b == 0).unwrap_or(8);
            let name_bytes = &raw[0..name_len];
            if !name_bytes.is_ascii() {
                return Err(GxtError::BadTableName { table: index });
            }
            let name = String::from_utf8_lossy(name_bytes).into_owned();
            let table_off = u32::from_le_bytes([raw[8], raw[9], raw[10], raw[11]]) as usize;
            tables.push(Self::parse_table(data, index, name, table_off, bits)?);
        }
        Ok(Self {
            version,
            bits_per_char: bits,
            tables,
        })
    }

    fn parse_table(
        data: &[u8],
        index: usize,
        name: String,
        mut offset: usize,
        bits: u16,
    ) -> Result<GxtTable, GxtError> {
        // Non-MAIN tables prefix the key block with an 8-byte copy of the
        // table name; MAIN points at the key block directly.
        if peek_tag(data, offset) != Some(*b"TKEY") {
            offset = offset.saturating_add(8);
        }
        expect_tag(data, offset, *b"TKEY")?;
        let key_size = read_u32(data, offset.saturating_add(4))? as usize;
        if !key_size.is_multiple_of(8) {
            return Err(GxtError::OutOfRange {
                offset: offset.saturating_add(4),
            });
        }
        let data_off = offset.saturating_add(8).saturating_add(key_size);
        expect_tag(data, data_off, *b"TDAT")?;
        let data_size = read_u32(data, data_off.saturating_add(4))? as usize;
        let strings_base = data_off.saturating_add(8);
        let strings_end = strings_base.saturating_add(data_size);
        if strings_end > data.len() {
            return Err(GxtError::Truncated {
                offset: strings_end,
            });
        }
        let entry_count = key_size / 8;
        let mut entries = Vec::with_capacity(entry_count.min(data.len() / 8 + 1));
        for entry in 0..entry_count {
            let field = offset
                .saturating_add(8)
                .saturating_add(entry.saturating_mul(8));
            let rel = read_u32(data, field)? as usize;
            let hash = read_u32(data, field.saturating_add(4))?;
            let start = strings_base.saturating_add(rel);
            if start >= strings_end {
                return Err(GxtError::OutOfRange { offset: field });
            }
            let text =
                read_units(data, start, strings_end, bits).ok_or(GxtError::Unterminated {
                    table: index,
                    entry,
                })?;
            entries.push(GxtEntry { hash, text });
        }
        Ok(GxtTable { name, entries })
    }

    /// Format version from the header (observed: 4).
    #[must_use]
    pub fn version(&self) -> u16 {
        self.version
    }

    /// Bits per text unit from the header (observed: 16).
    #[must_use]
    pub fn bits_per_char(&self) -> u16 {
        self.bits_per_char
    }

    /// All tables in file order.
    #[must_use]
    pub fn tables(&self) -> &[GxtTable] {
        &self.tables
    }

    /// Find a table by name (exact, case sensitive).
    #[must_use]
    pub fn table(&self, name: &str) -> Option<&GxtTable> {
        self.tables.iter().find(|t| t.name == name)
    }

    /// Look up a label's glyph codes within one table.
    ///
    /// Returns `None` when the table or the label hash is absent.
    #[must_use]
    pub fn lookup(&self, table: &str, label: &str) -> Option<&[u16]> {
        self.lookup_hash(table, label_hash(label))
    }

    /// Look up glyph codes by precomputed label hash within one table.
    #[must_use]
    pub fn lookup_hash(&self, table: &str, hash: u32) -> Option<&[u16]> {
        self.table(table)
            .and_then(|t| t.entries.iter().find(|e| e.hash == hash))
            .map(|e| e.text.as_slice())
    }

    /// Iterate over every entry as (table name, label hash, glyph codes).
    pub fn iter_entries(&self) -> impl Iterator<Item = (&str, u32, &[u16])> {
        self.tables.iter().flat_map(|t| {
            t.entries
                .iter()
                .map(|e| (t.name.as_str(), e.hash, e.text.as_slice()))
        })
    }

    /// Total number of entries across all tables.
    #[must_use]
    pub fn entry_count(&self) -> usize {
        self.tables.iter().map(|t| t.entries.len()).sum()
    }
}

/// Decode western glyph codes to text with lossy fallback.
///
/// Codes below 256 map through Windows-1252 (the game's western page);
/// codes from other font pages become U+FFFD. Russian and Japanese strings
/// therefore decode only partially; use the raw [`GxtEntry::text`] codes with
/// the font maps for faithful handling.
#[must_use]
pub fn decode_western_lossy(units: &[u16]) -> String {
    units.iter().map(|u| decode_unit(*u)).collect()
}

fn decode_unit(unit: u16) -> char {
    if unit < 256 {
        // Windows-1252 printable range 0x80-0x9F; the rest match Latin-1.
        const TABLE: [char; 32] = [
            '€', '\u{81}', '‚', 'ƒ', '„', '…', '†', '‡', 'ˆ', '‰', 'Š', '‹', 'Œ', '\u{8d}', 'Ž',
            '\u{8f}', '\u{90}', '‘', '’', '“', '”', '•', '–', '—', '˜', '™', 'š', '›', 'œ',
            '\u{9d}', 'ž', 'Ÿ',
        ];
        let byte = u8::try_from(unit).unwrap_or(0);
        if (0x80..0xA0).contains(&byte) {
            TABLE[usize::from(byte - 0x80)]
        } else {
            // 0x00-0x7F and 0xA0-0xFF are identical in Latin-1.
            char::from_u32(u32::from(byte)).unwrap_or('\u{FFFD}')
        }
    } else {
        '\u{FFFD}'
    }
}

fn read_u32(data: &[u8], offset: usize) -> Result<u32, GxtError> {
    data.get(offset..offset.saturating_add(4))
        .and_then(|b| b.try_into().ok())
        .map(u32::from_le_bytes)
        .ok_or(GxtError::Truncated { offset })
}

fn peek_tag(data: &[u8], offset: usize) -> Option<[u8; 4]> {
    data.get(offset..offset.saturating_add(4))
        .and_then(|b| b.try_into().ok())
}

fn expect_tag(data: &[u8], offset: usize, expected: [u8; 4]) -> Result<(), GxtError> {
    match peek_tag(data, offset) {
        Some(found) if found == expected => Ok(()),
        Some(found) => Err(GxtError::BadTag {
            offset,
            expected,
            found,
        }),
        None => {
            // The tag may start past the end of a truncated file (found by
            // the mutation fuzzer; slicing from `offset` used to panic).
            let mut found = [0u8; 4];
            let tail = data.get(offset..).unwrap_or(&[]);
            let have = tail.len().min(4);
            found[..have].copy_from_slice(&tail[..have]);
            Err(GxtError::BadTag {
                offset,
                expected,
                found,
            })
        }
    }
}

/// Read NUL-terminated units; `None` when no terminator precedes `end`.
fn read_units(data: &[u8], start: usize, end: usize, bits: u16) -> Option<Vec<u16>> {
    if bits == 8 {
        let mut out = Vec::new();
        let mut pos = start;
        loop {
            let byte = *data.get(pos..end)?.first()?;
            if byte == 0 {
                return Some(out);
            }
            out.push(u16::from(byte));
            pos += 1;
        }
    }
    let mut out = Vec::new();
    let mut pos = start;
    loop {
        let bytes: [u8; 2] = data.get(pos..pos.saturating_add(2)).and_then(|b| {
            if pos + 2 <= end {
                b.try_into().ok()
            } else {
                None
            }
        })?;
        let unit = u16::from_le_bytes(bytes);
        if unit == 0 {
            return Some(out);
        }
        out.push(unit);
        pos += 2;
    }
}

/// Decode a NUL-padded ASCII table name (helper kept total for tests).
#[allow(dead_code)]
fn table_name(raw: &[u8]) -> Result<String, Utf8Error> {
    let len = raw.iter().position(|b| *b == 0).unwrap_or(raw.len());
    std::str::from_utf8(&raw[..len]).map(str::to_owned)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hash::label_hash;

    /// Build a minimal two-table MAIN + second-table buffer by hand.
    fn fixture_bits16() -> Vec<u8> {
        let mut buf = Vec::new();
        buf.extend_from_slice(&4u16.to_le_bytes());
        buf.extend_from_slice(&16u16.to_le_bytes());
        buf.extend_from_slice(b"TABL");
        buf.extend_from_slice(&24u32.to_le_bytes()); // two entries
        let mut main = [0u8; 8];
        main[..4].copy_from_slice(b"MAIN");
        buf.extend_from_slice(&main);
        let main_off_pos = buf.len();
        buf.extend_from_slice(&0u32.to_le_bytes());
        let mut aux = [0u8; 8];
        aux[..3].copy_from_slice(b"AUX");
        buf.extend_from_slice(&aux);
        let aux_off_pos = buf.len();
        buf.extend_from_slice(&0u32.to_le_bytes());
        // MAIN table: direct TKEY.
        let main_off = buf.len() as u32;
        let h0 = label_hash("MO_OFF");
        let h1 = label_hash("MO_ON");
        let s0: Vec<u8> = "Off".encode_utf16().flat_map(u16::to_le_bytes).collect();
        let s1: Vec<u8> = "On".encode_utf16().flat_map(u16::to_le_bytes).collect();
        let key_size = 16u32;
        buf.extend_from_slice(b"TKEY");
        buf.extend_from_slice(&key_size.to_le_bytes());
        buf.extend_from_slice(&0u32.to_le_bytes());
        buf.extend_from_slice(&h0.to_le_bytes());
        buf.extend_from_slice(&((s0.len() + 2) as u32).to_le_bytes());
        buf.extend_from_slice(&h1.to_le_bytes());
        buf.extend_from_slice(b"TDAT");
        let tdat = [&s0[..], &[0, 0][..], &s1[..], &[0, 0][..]].concat();
        buf.extend_from_slice(&(tdat.len() as u32).to_le_bytes());
        buf.extend_from_slice(&tdat);
        // AUX table: 8-byte name prefix then TKEY with one entry.
        let aux_off = buf.len() as u32;
        buf.extend_from_slice(&aux);
        let ha = label_hash("BLIPS");
        let sa: Vec<u8> = "X".encode_utf16().flat_map(u16::to_le_bytes).collect();
        buf.extend_from_slice(b"TKEY");
        buf.extend_from_slice(&8u32.to_le_bytes());
        buf.extend_from_slice(&0u32.to_le_bytes());
        buf.extend_from_slice(&ha.to_le_bytes());
        buf.extend_from_slice(b"TDAT");
        let ta = [&sa[..], &[0, 0][..]].concat();
        buf.extend_from_slice(&(ta.len() as u32).to_le_bytes());
        buf.extend_from_slice(&ta);
        buf[main_off_pos..main_off_pos + 4].copy_from_slice(&main_off.to_le_bytes());
        buf[aux_off_pos..aux_off_pos + 4].copy_from_slice(&aux_off.to_le_bytes());
        buf
    }

    #[test]
    fn parses_two_tables() {
        let buf = fixture_bits16();
        let file = GxtFile::parse(&buf).expect("fixture must parse");
        assert_eq!(file.version(), 4);
        assert_eq!(file.bits_per_char(), 16);
        assert_eq!(file.tables().len(), 2);
        assert_eq!(file.entry_count(), 3);
        assert_eq!(
            file.lookup("MAIN", "MO_OFF"),
            Some("Off".encode_utf16().collect::<Vec<_>>().as_slice())
        );
        assert_eq!(
            decode_western_lossy(file.lookup("MAIN", "mo_on").unwrap()),
            "On"
        );
        assert_eq!(file.lookup("AUX", "BLIPS").unwrap().len(), 1);
        assert_eq!(file.lookup("MAIN", "NOPE"), None);
        assert_eq!(file.lookup("MISSING", "MO_OFF"), None);
    }

    #[test]
    fn iter_entries_covers_all() {
        let buf = fixture_bits16();
        let file = GxtFile::parse(&buf).unwrap();
        let all: Vec<_> = file.iter_entries().collect();
        assert_eq!(all.len(), 3);
        assert_eq!(all[0].0, "MAIN");
        assert_eq!(all[2].0, "AUX");
    }

    #[test]
    fn rejects_bad_tag() {
        let mut buf = fixture_bits16();
        buf[4] = b'X';
        let err = GxtFile::parse(&buf).unwrap_err();
        assert!(matches!(err, GxtError::BadTag { .. }));
    }

    #[test]
    fn rejects_truncation() {
        let buf = fixture_bits16();
        let err = GxtFile::parse(&buf[..10]).unwrap_err();
        assert!(matches!(err, GxtError::Truncated { .. }));
    }

    #[test]
    fn rejects_unterminated_string() {
        let mut buf = fixture_bits16();
        // Corrupt the final terminator of the last data block.
        let len = buf.len();
        buf[len - 2] = b'Q';
        buf[len - 1] = b'Q';
        let err = GxtFile::parse(&buf).unwrap_err();
        assert!(matches!(err, GxtError::Unterminated { .. }));
    }

    #[test]
    fn parses_8bit_units() {
        let mut buf = Vec::new();
        buf.extend_from_slice(&4u16.to_le_bytes());
        buf.extend_from_slice(&8u16.to_le_bytes());
        buf.extend_from_slice(b"TABL");
        buf.extend_from_slice(&12u32.to_le_bytes());
        let mut main = [0u8; 8];
        main[..4].copy_from_slice(b"MAIN");
        buf.extend_from_slice(&main);
        buf.extend_from_slice(&(24u32).to_le_bytes());
        buf.extend_from_slice(b"TKEY");
        buf.extend_from_slice(&8u32.to_le_bytes());
        buf.extend_from_slice(&0u32.to_le_bytes());
        buf.extend_from_slice(&label_hash("A").to_le_bytes());
        buf.extend_from_slice(b"TDAT");
        buf.extend_from_slice(&3u32.to_le_bytes());
        buf.extend_from_slice(b"Hi\0");
        let file = GxtFile::parse(&buf).unwrap();
        assert_eq!(
            file.lookup("MAIN", "a").unwrap(),
            &[b'H' as u16, b'i' as u16]
        );
    }

    #[test]
    fn western_decode_maps_cp1252() {
        assert_eq!(decode_western_lossy(&[0x41, 0x99, 0x201]), "A™�");
    }
}
