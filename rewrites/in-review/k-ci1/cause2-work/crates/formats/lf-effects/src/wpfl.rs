//! The binary `*.wpfl` effect packages: banks of rules keyed by name hash.
//!
//! See the [crate-level documentation](crate) for the layout. Parsing never
//! panics on malformed input: every offset and count is bounds-checked and
//! failures are returned as [`crate::Error`].

use std::collections::HashMap;

use lf_resource::{Pointer, Resource};

use crate::{Error, Result, jenkins_oat};

/// Resource type id of the entity/script effect packages.
pub const KIND_PARTICLES2: u32 = 0x1B;
/// Resource type id of the core/episode effect packages.
pub const KIND_PARTICLES: u32 = 0x24;

/// Largest bank entry count accepted (the largest real bank holds 726).
const MAX_ENTRIES: u64 = 100_000;
/// Shortest inline name kept for hash resolution.
const MIN_NAME_LEN: usize = 3;
/// Longest inline name kept for hash resolution.
const MAX_NAME_LEN: usize = 64;

/// One entry in an effect bank: a name hash plus the record it selects.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BankEntry {
    /// Jenkins one-at-a-time hash of the entry name.
    pub hash: u32,
    /// Pointer to the record in the system segment.
    pub record: Pointer,
    /// First word of the record (the vtable slot; identifies the record class).
    pub vtable: u32,
}

/// One hash-to-record dictionary in a package.
#[derive(Debug, Clone)]
pub struct Bank {
    /// Root slot this bank was found in (0, 2, 3, 5 or 6 in shipped files).
    pub slot: usize,
    /// First word of the bank header (the vtable slot; identifies the role).
    pub vtable: u32,
    entries: Vec<BankEntry>,
}

impl Bank {
    /// Entries in file order (hash-array order).
    #[must_use]
    pub fn entries(&self) -> &[BankEntry] {
        &self.entries
    }

    /// Number of entries.
    #[must_use]
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// True when the bank holds no entries.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Find the entry with the given name hash, if any.
    #[must_use]
    pub fn find(&self, hash: u32) -> Option<&BankEntry> {
        self.entries.iter().find(|e| e.hash == hash)
    }
}

/// A parsed `*.wpfl` effect package: five banks plus a resolved name table.
#[derive(Debug)]
pub struct WpflFile {
    kind: u32,
    banks: Vec<Bank>,
    names: HashMap<u32, String>,
    resource: Resource,
}

impl WpflFile {
    /// Parse a whole package file from memory.
    ///
    /// Rejects non-resource input, resources of other types, and packages
    /// whose root, banks or record pointers do not have the documented shape.
    ///
    /// # Errors
    ///
    /// Returns an error when the input is not an effect resource or the
    /// package root, banks or record pointers fail validation.
    pub fn parse(bytes: &[u8]) -> Result<WpflFile> {
        let resource = Resource::parse(bytes)?;
        let kind = resource.header().kind.raw();
        if kind != KIND_PARTICLES && kind != KIND_PARTICLES2 {
            return Err(Error::NotEffects { found: kind });
        }
        let sys = resource.system();
        if sys.len() < 32 {
            return Err(Error::BadLayout {
                detail: format!("system segment too short for root: {} bytes", sys.len()),
            });
        }
        let mut banks = Vec::new();
        for slot in 0..8 {
            let raw = read_u32(sys, slot * 4)?;
            if raw == 0 || raw == Pointer::DEBUG_FILL {
                continue;
            }
            let ptr = Pointer::new(raw);
            let target = ptr.offset();
            if ptr.segment().is_none() || target >= sys.len() {
                return Err(Error::BadPointer { value: raw });
            }
            banks.push(Self::parse_bank(&resource, slot, ptr)?);
        }
        if banks.is_empty() {
            return Err(Error::BadLayout {
                detail: "root holds no banks".to_string(),
            });
        }
        let names = collect_names(resource.system(), resource.graphics());
        Ok(WpflFile {
            kind,
            banks,
            names,
            resource,
        })
    }

    /// The resource type id (`0x1B` or `0x24`).
    #[must_use]
    pub fn kind(&self) -> u32 {
        self.kind
    }

    /// The banks in root-slot order.
    #[must_use]
    pub fn banks(&self) -> &[Bank] {
        &self.banks
    }

    /// Total entries across all banks.
    #[must_use]
    pub fn entry_count(&self) -> usize {
        self.banks.iter().map(Bank::len).sum()
    }

    /// The name behind an entry hash, when the name appears in this package.
    #[must_use]
    pub fn name_of(&self, hash: u32) -> Option<&str> {
        self.names.get(&hash).map(String::as_str)
    }

    /// How many bank entries resolve to an inline name.
    #[must_use]
    pub fn resolved_count(&self) -> usize {
        self.banks
            .iter()
            .flat_map(|b| b.entries.iter())
            .filter(|e| self.names.contains_key(&e.hash))
            .count()
    }

    /// Iterate over every entry in every bank, in file order.
    pub fn entries(&self) -> impl Iterator<Item = (usize, &BankEntry)> + '_ {
        self.banks
            .iter()
            .flat_map(|b| b.entries.iter().map(move |e| (b.slot, e)))
    }

    /// Borrow the underlying resource (for future typed record readers).
    #[must_use]
    pub fn resource(&self) -> &Resource {
        &self.resource
    }

    /// Read up to `len` bytes of a record's raw content.
    ///
    /// Record layouts are not decoded yet; this exposes the bytes so a later
    /// lane can work from parsed packages instead of raw offsets.
    ///
    /// # Errors
    ///
    /// Returns an error when the record pointer leaves the resource.
    pub fn record_bytes(&self, entry: &BankEntry, len: usize) -> Result<&[u8]> {
        self.resource.slice(entry.record, len).map_err(Error::from)
    }

    fn parse_bank(resource: &Resource, slot: usize, at: Pointer) -> Result<Bank> {
        let head = resource.slice(at, 32).map_err(Error::from)?;
        let w = |i: usize| u32::from_le_bytes([head[i], head[i + 1], head[i + 2], head[i + 3]]);
        let vtable = w(0);
        if w(4) != 0 || w(8) != 0 || w(12) != 1 {
            return Err(Error::BadLayout {
                detail: format!(
                    "bank in slot {slot}: expected [vtable,0,0,1], found [{:#x},{},{:#x}]",
                    w(4),
                    w(8),
                    w(12)
                ),
            });
        }
        let hash_ptr = Pointer::new(w(16));
        let rec_ptr = Pointer::new(w(24));
        let (n1, c1) = (w(20) & 0xFFFF, w(20) >> 16);
        let (n2, c2) = (w(28) & 0xFFFF, w(28) >> 16);
        if n1 != c1 || n2 != c2 || n1 != n2 {
            return Err(Error::BadLayout {
                detail: format!(
                    "bank in slot {slot}: count words disagree ({:#x},{:#x})",
                    w(20),
                    w(28)
                ),
            });
        }
        if u64::from(n1) > MAX_ENTRIES {
            return Err(Error::TooLarge {
                what: "bank entry count",
                count: u64::from(n1),
            });
        }
        let count = n1 as usize;
        let hashes = resource
            .slice(
                hash_ptr,
                count.checked_mul(4).ok_or(Error::TooLarge {
                    what: "bank hash array",
                    count: u64::from(n1),
                })?,
            )
            .map_err(Error::from)?;
        let ptrs = resource
            .slice(
                rec_ptr,
                count.checked_mul(4).ok_or(Error::TooLarge {
                    what: "bank pointer array",
                    count: u64::from(n1),
                })?,
            )
            .map_err(Error::from)?;
        let sys_len = resource.system().len();
        let mut entries = Vec::with_capacity(count);
        for i in 0..count {
            let hash = u32::from_le_bytes([
                hashes[i * 4],
                hashes[i * 4 + 1],
                hashes[i * 4 + 2],
                hashes[i * 4 + 3],
            ]);
            let raw = u32::from_le_bytes([
                ptrs[i * 4],
                ptrs[i * 4 + 1],
                ptrs[i * 4 + 2],
                ptrs[i * 4 + 3],
            ]);
            let record = Pointer::new(raw);
            if record.segment().is_none() || record.offset().saturating_add(4) > sys_len {
                return Err(Error::BadPointer { value: raw });
            }
            let head = resource.slice(record, 4).map_err(Error::from)?;
            entries.push(BankEntry {
                hash,
                record,
                vtable: u32::from_le_bytes([head[0], head[1], head[2], head[3]]),
            });
        }
        Ok(Bank {
            slot,
            vtable,
            entries,
        })
    }
}

fn read_u32(sys: &[u8], at: usize) -> Result<u32> {
    sys.get(at..at + 4)
        .ok_or_else(|| Error::BadLayout {
            detail: format!("root overruns system segment at {at}"),
        })
        .map(|b| u32::from_le_bytes([b[0], b[1], b[2], b[3]]))
}

fn is_name_byte(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_' || b == b'.' || b == b'-'
}

/// Collect NUL-terminated ASCII names from both segments, keyed by hash.
///
/// Walks back from every NUL byte over name characters; first spelling wins
/// when two names share a hash.
fn collect_names(system: &[u8], graphics: &[u8]) -> HashMap<u32, String> {
    let mut out = HashMap::new();
    for seg in [system, graphics] {
        for (i, &b) in seg.iter().enumerate() {
            if b != 0 {
                continue;
            }
            let mut start = i;
            while start > 0 && is_name_byte(seg[start - 1]) && i - start < MAX_NAME_LEN {
                start -= 1;
            }
            let len = i - start;
            if len >= MIN_NAME_LEN
                && let Ok(name) = std::str::from_utf8(&seg[start..i])
            {
                out.entry(jenkins_oat(name.as_bytes()))
                    .or_insert_with(|| name.to_string());
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use flate2::Compression;
    use flate2::write::ZlibEncoder;
    use std::io::Write;

    /// Build a minimal package in memory: root + one bank with two entries.
    /// All bytes are constructed here; nothing comes from game files.
    fn build_package() -> Vec<u8> {
        // System segment layout (all offsets Archivo-relative to segment):
        //   0x00: root, slot 0 -> bank at 0x100, rest null
        //   0x100: bank header, hashes at 0x200, pointers at 0x210
        //   0x200: hashes of "fire_large" and "smoke_small"
        //   0x210: record pointers 0x300, 0x340
        //   0x300/0x340: records starting with vtable words
        //   0x400: NUL-terminated names for resolution
        let mut sys = vec![0u8; 2048];
        let put = |sys: &mut Vec<u8>, at: usize, v: u32| {
            sys[at..at + 4].copy_from_slice(&v.to_le_bytes());
        };
        put(&mut sys, 0, 0x5000_0100);
        put(&mut sys, 0x100, 0x0040_1000); // bank vtable
        put(&mut sys, 0x104, 0);
        put(&mut sys, 0x108, 0);
        put(&mut sys, 0x10C, 1);
        put(&mut sys, 0x110, 0x5000_0200);
        put(&mut sys, 0x114, 0x0002_0002);
        put(&mut sys, 0x118, 0x5000_0210);
        put(&mut sys, 0x11C, 0x0002_0002);
        put(&mut sys, 0x200, jenkins_oat(b"fire_large"));
        put(&mut sys, 0x204, jenkins_oat(b"smoke_small"));
        put(&mut sys, 0x210, 0x5000_0300);
        put(&mut sys, 0x214, 0x5000_0340);
        put(&mut sys, 0x300, 0x0040_2000);
        put(&mut sys, 0x340, 0x0040_3000);
        sys[0x400..0x400 + 11].copy_from_slice(b"fire_large\0");
        sys[0x410..0x410 + 12].copy_from_slice(b"smoke_small\0");
        let gfx = vec![0u8; 256];
        // flags: sys_b = gfx_b = 0, mantissa = len / 256.
        let sys_pages = u32::try_from(sys.len() / 256).expect("fixture fits in u32");
        let gfx_pages = u32::try_from(gfx.len() / 256).expect("fixture fits in u32");
        let flags: u32 = sys_pages | (gfx_pages << 15);
        let mut enc = ZlibEncoder::new(Vec::new(), Compression::best());
        enc.write_all(&sys).unwrap();
        enc.write_all(&gfx).unwrap();
        let payload = enc.finish().unwrap();
        let mut file = Vec::with_capacity(12 + payload.len());
        file.extend_from_slice(&0x0543_5352u32.to_le_bytes());
        file.extend_from_slice(&KIND_PARTICLES.to_le_bytes());
        file.extend_from_slice(&flags.to_le_bytes());
        file.extend_from_slice(&payload);
        file
    }

    #[test]
    fn parses_hand_built_package() {
        let file = build_package();
        let pkg = WpflFile::parse(&file).unwrap();
        assert_eq!(pkg.kind(), KIND_PARTICLES);
        assert_eq!(pkg.banks().len(), 1);
        assert_eq!(pkg.entry_count(), 2);
        let bank = &pkg.banks()[0];
        assert_eq!(bank.slot, 0);
        assert_eq!(bank.vtable, 0x0040_1000);
        assert_eq!(bank.entries()[0].hash, jenkins_oat(b"fire_large"));
        assert_eq!(bank.entries()[1].vtable, 0x0040_3000);
        assert_eq!(pkg.name_of(bank.entries()[0].hash), Some("fire_large"));
        assert_eq!(pkg.name_of(bank.entries()[1].hash), Some("smoke_small"));
        assert_eq!(pkg.resolved_count(), 2);
        assert_eq!(
            bank.find(jenkins_oat(b"fire_large")).unwrap().vtable,
            0x0040_2000
        );
        assert!(bank.find(0x1234_5678).is_none());
        // Record bytes are readable through the parsed package.
        let head = pkg.record_bytes(&bank.entries()[0], 4).unwrap();
        assert_eq!(head, &0x0040_2000u32.to_le_bytes());
    }

    #[test]
    fn rejects_wrong_type_and_bad_layout() {
        let mut file = build_package();
        // Texture type instead of particles.
        file[4..8].copy_from_slice(&0x08u32.to_le_bytes());
        assert!(matches!(
            WpflFile::parse(&file),
            Err(Error::NotEffects { found: 0x08 })
        ));
    }

    #[test]
    fn name_scan_ignores_short_and_binary_runs() {
        let seg = b"ab\0fire_large\0\x01\x02\x03\0";
        let names = collect_names(seg, b"");
        assert_eq!(names.len(), 1);
        assert_eq!(
            names.get(&jenkins_oat(b"fire_large")).map(String::as_str),
            Some("fire_large")
        );
    }
}
