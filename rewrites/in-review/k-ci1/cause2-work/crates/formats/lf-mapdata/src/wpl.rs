//! Binary item placement (`.wpl`) files.
//!
//! Flat little-endian binary: a 68-byte header (version word plus sixteen
//! per-section counts) followed by the record arrays. The arrays are stored in
//! a fixed order that is **not** numeric section order: `inst` (0), `grge`
//! (2), `cars` (3), `tcyc` (4), `mlop` (8), `blok` (15), `lodm` (9), `slow`
//! (10). Sections 1, 5, 6, 7 and 11-14 are always empty. The positions of
//! `cars` and `mlop` within that order are assumed, not observed: both are
//! always empty in shipped files, so any position parses them identically.
//!
//! All reads are explicit little-endian conversions from the byte slice; the
//! parser is pointer-width and platform independent.

use crate::Error;

/// Expected `.wpl` version word; the only value in shipped files.
pub const VERSION: u32 = 3;

/// Size of the file header in bytes.
pub const HEADER_SIZE: usize = 68;

/// Record sizes in bytes, in on-disk section order.
pub const INST_SIZE: usize = 48;
/// Garage record size in bytes.
pub const GRGE_SIZE: usize = 48;
/// Parked-car record size in bytes (documented, never observed).
pub const CARS_SIZE: usize = 56;
/// Time-cycle modifier record size in bytes.
///
/// Public documentation says 56, but its own field table sums to 44 and every
/// shipped file agrees with 44.
pub const TCYC_SIZE: usize = 44;
/// Interior placement record size in bytes (documented, never observed).
pub const MLOP_SIZE: usize = 64;
/// Block stamp record size in bytes.
pub const BLOK_SIZE: usize = 132;
/// LOD record size in bytes.
pub const LODM_SIZE: usize = 388;
/// Slow-zone record size in bytes.
pub const SLOW_SIZE: usize = 24;

/// A parsed `.wpl` file: every record array plus the raw header counts.
#[derive(Debug, Clone, Default)]
pub struct WplFile {
    /// The sixteen header counts in section-index order.
    pub counts: [u32; 16],
    /// Instance placements (section 0).
    pub inst: Vec<Inst>,
    /// Garages (section 2).
    pub grge: Vec<Grge>,
    /// Parked cars (section 3); always empty in shipped files.
    pub cars: Vec<Car>,
    /// Time-cycle modifiers (section 4).
    pub tcyc: Vec<Tcyc>,
    /// Interior placements (section 8); always empty in shipped files.
    pub mlop: Vec<Mlop>,
    /// Block stamps (section 15).
    pub blok: Vec<Blok>,
    /// LOD entries (section 9).
    pub lodm: Vec<Lodm>,
    /// Slow zones (section 10).
    pub slow: Vec<Slow>,
    /// Bytes past the last record. One shipped file carries zero padding
    /// here; anything else is preserved for inspection.
    pub trailing_bytes: Vec<u8>,
}

/// Instance placement (48 bytes).
#[derive(Debug, Clone, PartialEq)]
pub struct Inst {
    /// Position (x, y, z).
    pub pos: [f32; 3],
    /// Rotation quaternion (x, y, z, w).
    pub rot: [f32; 4],
    /// Model name hash.
    pub model_hash: u32,
    /// Placement flags.
    pub flags: u32,
    /// LOD instance index; meaning partly uncertain.
    pub lod_index: i32,
    /// Unknown word.
    pub unknown: u32,
    /// Unknown float (often -1.0).
    pub unknown_float: f32,
}

/// Garage (48 bytes).
#[derive(Debug, Clone, PartialEq)]
pub struct Grge {
    /// Lower-left corner (x, y, z).
    pub corner_a: [f32; 3],
    /// Front direction (x, y).
    pub front: [f32; 2],
    /// Upper-rear corner (x, y, z).
    pub corner_b: [f32; 3],
    /// Door type id.
    pub door_type: u32,
    /// Garage type id.
    pub garage_type: u32,
    /// Garage name (up to 8 bytes, NUL-padded).
    pub name: String,
}

/// Parked car (56 bytes).
///
/// Layout follows the public documentation; no shipped file holds one, so
/// every field below is unverified against real data.
#[derive(Debug, Clone, PartialEq)]
pub struct Car {
    /// Position (x, y, z).
    pub pos: [f32; 3],
    /// Unknown float.
    pub unknown_a: f32,
    /// Two rotation floats; meanings uncertain.
    pub rot: [f32; 2],
    /// Model name hash.
    pub model_hash: u32,
    /// First paint colour.
    pub color_a: i32,
    /// Second paint colour.
    pub color_b: i32,
    /// Third paint colour.
    pub color_c: i32,
    /// Specular colour.
    pub color_spec: i32,
    /// Flags.
    pub flags: u32,
    /// Alarm id.
    pub alarm: i32,
    /// Unknown integer.
    pub unknown_b: i32,
}

/// Time-cycle modifier box (44 bytes).
#[derive(Debug, Clone, PartialEq)]
pub struct Tcyc {
    /// Lower corner (x, y, z).
    pub corner_a: [f32; 3],
    /// Upper corner (x, y, z).
    pub corner_b: [f32; 3],
    /// Four unknown words (read as floats in some shipped rows, e.g. 23.5 and
    /// 6.0; kept raw).
    pub unknown: [u32; 4],
    /// Box name hash.
    pub hash: u32,
}

/// Interior placement (64 bytes).
///
/// Layout follows the public documentation; no shipped file holds one, so
/// every field below is unverified against real data.
#[derive(Debug, Clone, PartialEq)]
pub struct Mlop {
    /// Model name (up to 24 bytes, NUL-padded).
    pub model: String,
    /// Flags.
    pub flags: u32,
    /// Interior instance index.
    pub interior_index: u32,
    /// Unknown word.
    pub unknown: u32,
    /// Position (x, y, z).
    pub pos: [f32; 3],
    /// Rotation quaternion (x, y, z, w).
    pub rot: [f32; 4],
}

/// Block stamp (132 bytes): a source-file line plus eight floats.
///
/// The game ignores this section. The string repeats the area name, author and
/// date also found in text `blok` rows. Several shipped records pad the string
/// field with `0xCD` (uninitialised debug-heap memory from the tool that wrote
/// the files) rather than zeros.
#[derive(Debug, Clone, PartialEq)]
pub struct Blok {
    /// Leading word, always 1 in shipped files; meaning uncertain.
    pub unknown_a: u32,
    /// Source line (`name,unknown,author,date`), cut at the first NUL.
    pub text: String,
    /// Word after the string, always 128; meaning uncertain.
    pub unknown_b: u32,
    /// Eight trailing floats; meanings uncertain (often `0xCD` fill).
    pub floats: [f32; 8],
}

/// LOD entry (388 bytes): a bounding box with model hashes and names.
#[derive(Debug, Clone, PartialEq)]
pub struct Lodm {
    /// Lower corner (x, y, z).
    pub corner_a: [f32; 3],
    /// Upper corner (x, y, z).
    pub corner_b: [f32; 3],
    /// Entry count, always 10; how many of the hash/name slots are live.
    pub count: u32,
    /// Ten model hashes.
    pub hashes: [u32; 10],
    /// Ten model names (32 bytes each, NUL-padded).
    pub names: [String; 10],
}

/// Slow zone (24 bytes): a bounding box in world coordinates.
#[derive(Debug, Clone, PartialEq)]
pub struct Slow {
    /// Lower corner (x, y, z).
    pub corner_a: [f32; 3],
    /// Upper corner (x, y, z).
    pub corner_b: [f32; 3],
}

struct Cursor<'a> {
    data: &'a [u8],
    off: usize,
}

impl<'a> Cursor<'a> {
    fn new(data: &'a [u8]) -> Cursor<'a> {
        Cursor { data, off: 0 }
    }

    fn take(&mut self, n: usize, what: &str) -> Result<&'a [u8], Error> {
        if self.off + n > self.data.len() {
            return Err(Error::Truncated {
                offset: self.data.len(),
                what: what.to_string(),
            });
        }
        let s = &self.data[self.off..self.off + n];
        self.off += n;
        Ok(s)
    }

    fn u32(&mut self, what: &str) -> Result<u32, Error> {
        let b = self.take(4, what)?;
        Ok(u32::from_le_bytes([b[0], b[1], b[2], b[3]]))
    }

    fn i32(&mut self, what: &str) -> Result<i32, Error> {
        let b = self.take(4, what)?;
        Ok(i32::from_le_bytes([b[0], b[1], b[2], b[3]]))
    }

    fn f32(&mut self, what: &str) -> Result<f32, Error> {
        let b = self.take(4, what)?;
        Ok(f32::from_le_bytes([b[0], b[1], b[2], b[3]]))
    }

    fn array_f32<const N: usize>(&mut self, what: &str) -> Result<[f32; N], Error> {
        let mut out = [0f32; N];
        for v in &mut out {
            *v = self.f32(what)?;
        }
        Ok(out)
    }

    fn array_u32<const N: usize>(&mut self, what: &str) -> Result<[u32; N], Error> {
        let mut out = [0u32; N];
        for v in &mut out {
            *v = self.u32(what)?;
        }
        Ok(out)
    }

    fn fixed_string(&mut self, n: usize, what: &str) -> Result<String, Error> {
        let b = self.take(n, what)?;
        let end = b.iter().position(|&c| c == 0).unwrap_or(n);
        Ok(String::from_utf8_lossy(&b[..end]).into_owned())
    }
}

impl WplFile {
    /// Parse a `.wpl` file from a byte slice.
    ///
    /// Fails on a short header, an unknown version, or a record array that
    /// runs past the end of the input. Trailing bytes after the last record
    /// are kept in [`WplFile::trailing_bytes`], not treated as an error.
    ///
    /// # Errors
    ///
    /// Returns an error if the input is truncated or malformed.
    // One section reader per record kind, in on-disk order; splitting would
    // scatter the order the format requires.
    #[allow(clippy::too_many_lines)]
    pub fn parse(bytes: &[u8]) -> Result<WplFile, Error> {
        if bytes.len() < HEADER_SIZE {
            return Err(Error::Truncated {
                offset: bytes.len(),
                what: "wpl header".to_string(),
            });
        }
        let mut cur = Cursor::new(bytes);
        let version = cur.u32("wpl version")?;
        if version != VERSION {
            return Err(Error::BadVersion { found: version });
        }
        let mut counts = [0u32; 16];
        for c in &mut counts {
            *c = cur.u32("wpl header counts")?;
        }
        let mut out = WplFile {
            counts,
            ..WplFile::default()
        };
        // On-disk order: 0, 2, 3, 4, 8, 15, 9, 10. Verified against all
        // shipped files (see crate docs).
        for _ in 0..counts[0] {
            out.inst.push(Inst {
                pos: cur.array_f32("inst pos")?,
                rot: cur.array_f32("inst rot")?,
                model_hash: cur.u32("inst hash")?,
                flags: cur.u32("inst flags")?,
                lod_index: cur.i32("inst lod")?,
                unknown: cur.u32("inst unknown")?,
                unknown_float: cur.f32("inst unknown float")?,
            });
        }
        for _ in 0..counts[2] {
            let corner_a: [f32; 3] = cur.array_f32("grge a")?;
            let front: [f32; 2] = [cur.f32("grge front")?, cur.f32("grge front")?];
            out.grge.push(Grge {
                corner_a,
                front,
                corner_b: cur.array_f32("grge b")?,
                door_type: cur.u32("grge door")?,
                garage_type: cur.u32("grge type")?,
                name: cur.fixed_string(8, "grge name")?,
            });
        }
        for _ in 0..counts[3] {
            out.cars.push(Car {
                pos: cur.array_f32("cars pos")?,
                unknown_a: cur.f32("cars unknown")?,
                rot: [cur.f32("cars rot")?, cur.f32("cars rot")?],
                model_hash: cur.u32("cars hash")?,
                color_a: cur.i32("cars color")?,
                color_b: cur.i32("cars color")?,
                color_c: cur.i32("cars color")?,
                color_spec: cur.i32("cars specular")?,
                flags: cur.u32("cars flags")?,
                alarm: cur.i32("cars alarm")?,
                unknown_b: cur.i32("cars unknown")?,
            });
        }
        for _ in 0..counts[4] {
            out.tcyc.push(Tcyc {
                corner_a: cur.array_f32("tcyc a")?,
                corner_b: cur.array_f32("tcyc b")?,
                unknown: cur.array_u32("tcyc unknown")?,
                hash: cur.u32("tcyc hash")?,
            });
        }
        for _ in 0..counts[8] {
            out.mlop.push(Mlop {
                model: cur.fixed_string(24, "mlop model")?,
                flags: cur.u32("mlop flags")?,
                interior_index: cur.u32("mlop interior")?,
                unknown: cur.u32("mlop unknown")?,
                pos: cur.array_f32("mlop pos")?,
                rot: cur.array_f32("mlop rot")?,
            });
        }
        for _ in 0..counts[15] {
            out.blok.push(Blok {
                unknown_a: cur.u32("blok unknown")?,
                text: cur.fixed_string(92, "blok text")?,
                unknown_b: cur.u32("blok unknown")?,
                floats: cur.array_f32("blok floats")?,
            });
        }
        for _ in 0..counts[9] {
            let mut names: [String; 10] = Default::default();
            let corner_a = cur.array_f32("lodm a")?;
            let corner_b = cur.array_f32("lodm b")?;
            let count = cur.u32("lodm count")?;
            let hashes = cur.array_u32("lodm hashes")?;
            for n in &mut names {
                *n = cur.fixed_string(32, "lodm name")?;
            }
            out.lodm.push(Lodm {
                corner_a,
                corner_b,
                count,
                hashes,
                names,
            });
        }
        for _ in 0..counts[10] {
            out.slow.push(Slow {
                corner_a: cur.array_f32("slow a")?,
                corner_b: cur.array_f32("slow b")?,
            });
        }
        out.trailing_bytes = bytes[cur.off..].to_vec();
        Ok(out)
    }

    /// Total records across all sections.
    #[must_use]
    pub fn len(&self) -> usize {
        self.inst.len()
            + self.grge.len()
            + self.cars.len()
            + self.tcyc.len()
            + self.mlop.len()
            + self.blok.len()
            + self.lodm.len()
            + self.slow.len()
    }

    /// True when no records were parsed.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// True when the trailing bytes are all zero (padding, as in the one
    /// shipped file that carries any).
    #[must_use]
    pub fn trailing_is_zero_padding(&self) -> bool {
        self.trailing_bytes.iter().all(|&b| b == 0)
    }
}

#[cfg(test)]
#[allow(clippy::float_cmp)] // parsed values are compared bit for bit on purpose
mod tests {
    use super::*;

    fn header(counts: [u32; 16]) -> Vec<u8> {
        let mut v = Vec::new();
        v.extend_from_slice(&VERSION.to_le_bytes());
        for c in counts {
            v.extend_from_slice(&c.to_le_bytes());
        }
        v
    }

    #[test]
    fn parses_inst_grge_tcyc() {
        let mut counts = [0u32; 16];
        counts[0] = 1;
        counts[2] = 1;
        counts[4] = 1;
        let mut data = header(counts);
        // inst: pos + quat + hash + flags + lod + unk + unkf
        for f in [1.0f32, 2.0, 3.0, 0.0, 0.0, 0.0, 1.0] {
            data.extend_from_slice(&f.to_le_bytes());
        }
        data.extend_from_slice(&0xAABBCCDDu32.to_le_bytes());
        data.extend_from_slice(&7u32.to_le_bytes());
        data.extend_from_slice(&(-1i32).to_le_bytes());
        data.extend_from_slice(&11u32.to_le_bytes());
        data.extend_from_slice(&(-1.0f32).to_le_bytes());
        // grge: 8 floats + 2 u32 + 8 name bytes
        for f in [0.0f32, 0.0, 0.0, 1.0, 1.0, 2.0, 2.0, 2.0] {
            data.extend_from_slice(&f.to_le_bytes());
        }
        data.extend_from_slice(&1u32.to_le_bytes());
        data.extend_from_slice(&4u32.to_le_bytes());
        data.extend_from_slice(b"G1\0\0\0\0\0\0");
        // tcyc: 6 floats + 4 u32 + hash
        for f in [0.0f32, 0.0, 0.0, 9.0, 9.0, 9.0] {
            data.extend_from_slice(&f.to_le_bytes());
        }
        for u in [1u32, 2, 3, 4] {
            data.extend_from_slice(&u.to_le_bytes());
        }
        data.extend_from_slice(&0x1234u32.to_le_bytes());

        let f = WplFile::parse(&data).unwrap();
        assert_eq!(f.inst.len(), 1);
        assert_eq!(f.inst[0].pos, [1.0, 2.0, 3.0]);
        assert_eq!(f.inst[0].model_hash, 0xAABB_CCDD);
        assert_eq!(f.grge.len(), 1);
        assert_eq!(f.grge[0].name, "G1");
        assert_eq!(f.grge[0].garage_type, 4);
        assert_eq!(f.tcyc.len(), 1);
        assert_eq!(f.tcyc[0].hash, 0x1234);
        assert!(f.trailing_bytes.is_empty());
        assert_eq!(f.len(), 3);
    }

    #[test]
    fn blok_comes_before_lodm_and_slow() {
        // The on-disk order is blok(15), lodm(9), slow(10): build bytes in
        // that order and check each lands in the right array.
        let mut counts = [0u32; 16];
        counts[15] = 1;
        counts[9] = 1;
        counts[10] = 1;
        let mut data = header(counts);
        // blok: u32 + 92 str + u32 + 8 floats
        data.extend_from_slice(&1u32.to_le_bytes());
        let mut s = [0u8; 92];
        s[..4].copy_from_slice(b"Area");
        data.extend_from_slice(&s);
        data.extend_from_slice(&128u32.to_le_bytes());
        for _ in 0..8 {
            data.extend_from_slice(&0.0f32.to_le_bytes());
        }
        // lodm: 6 floats + count + 10 hashes + 10x32 names
        for f in [0.0f32, 0.0, 0.0, 5.0, 5.0, 5.0] {
            data.extend_from_slice(&f.to_le_bytes());
        }
        data.extend_from_slice(&10u32.to_le_bytes());
        for i in 0..10u32 {
            data.extend_from_slice(&i.to_le_bytes());
        }
        for i in 0..10 {
            let mut n = [0u8; 32];
            n[0] = b'm';
            n[1] = b'0' + i;
            data.extend_from_slice(&n);
        }
        // slow: 6 floats
        for f in [1.0f32, 1.0, 1.0, 2.0, 2.0, 2.0] {
            data.extend_from_slice(&f.to_le_bytes());
        }
        let f = WplFile::parse(&data).unwrap();
        assert_eq!(f.blok[0].text, "Area");
        assert_eq!(f.lodm[0].names[0], "m0");
        assert_eq!(f.slow[0].corner_b, [2.0, 2.0, 2.0]);
    }

    #[test]
    fn trailing_bytes_kept() {
        let data = [header([0u32; 16]), vec![0, 0, 1]].concat();
        let f = WplFile::parse(&data).unwrap();
        assert_eq!(f.trailing_bytes, vec![0, 0, 1]);
        assert!(!f.trailing_is_zero_padding());
    }

    #[test]
    fn bad_version_rejected() {
        let mut data = header([0u32; 16]);
        data[0] = 99;
        assert!(matches!(
            WplFile::parse(&data),
            Err(Error::BadVersion { found: 99 })
        ));
    }

    #[test]
    fn truncated_record_errors() {
        let mut counts = [0u32; 16];
        counts[0] = 2;
        let mut data = header(counts);
        data.extend_from_slice(&[0u8; INST_SIZE]); // only one of two
        assert!(matches!(
            WplFile::parse(&data),
            Err(Error::Truncated { .. })
        ));
    }
}
