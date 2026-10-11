//! Versioned audio configuration files, parsed structurally.
//!
//! `effects.dat11`, `curves.dat12`, `categories.dat15`, `sounds.dat15` and
//! `game.dat16` share one outer shape: a 16-byte header (`version`,
//! `names_off`, two unknown words), an opaque object region, and a name
//! region holding length-prefixed names with per-name metadata. The object
//! records and the per-name metadata split are still unknown, so both
//! regions are exposed raw alongside a heuristic name scan. `speech.dat`
//! has no version suffix and a different, unknown layout.

use crate::{Cursor, Error, ErrorKind};

/// Size of the config header, in bytes.
pub const HEADER_LEN: u64 = 16;

/// A name-like run found by the heuristic scan.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NameRun<'a> {
    /// Absolute byte offset where the run starts.
    pub offset: u32,
    /// The run text.
    pub text: &'a str,
}

/// A parsed config file borrowing its input.
#[derive(Debug)]
pub struct DatConfig<'a> {
    buf: &'a [u8],
    version: u32,
    names_off: u32,
    field_y: u32,
    field_z: u32,
}

impl<'a> DatConfig<'a> {
    /// Parse the outer structure: header, object region, name region.
    ///
    /// A `names_off` of 0 means the file carries no name region (seen in a
    /// tiny episode curve file): the whole body counts as objects.
    ///
    /// # Errors
    ///
    /// Returns an error when the input is shorter than the 16-byte header
    /// or the name offset points outside the input.
    pub fn parse(buf: &'a [u8]) -> Result<DatConfig<'a>, Error> {
        let mut cur = Cursor::new(buf);
        let version = cur.u32()?;
        let names_off = cur.u32()?;
        let field_y = cur.u32()?;
        let field_z = cur.u32()?;
        if names_off != 0
            && (u64::from(names_off) < HEADER_LEN || u64::from(names_off) > buf.len() as u64)
        {
            return Err(Error::new(
                4,
                ErrorKind::Invalid,
                format!("name offset {names_off} outside input of {}", buf.len()),
            ));
        }
        Ok(DatConfig {
            buf,
            version,
            names_off,
            field_y,
            field_z,
        })
    }

    /// Format version; equals the number in the file extension (11, 12, 15
    /// or 16 in files seen).
    #[must_use]
    pub fn version(&self) -> u32 {
        self.version
    }

    /// Byte offset where the name region starts (0 when there is none).
    #[must_use]
    pub fn names_off(&self) -> u32 {
        self.names_off
    }

    /// Unknown header word.
    #[must_use]
    pub fn field_y(&self) -> u32 {
        self.field_y
    }

    /// Unknown header word.
    #[must_use]
    pub fn field_z(&self) -> u32 {
        self.field_z
    }

    /// The object region: bytes between the header and the name region.
    /// Record layout unknown.
    #[must_use]
    pub fn objects(&self) -> &'a [u8] {
        let end = if self.names_off == 0 {
            self.buf.len()
        } else {
            self.names_off as usize
        };
        // `parse` validated `HEADER_LEN <= end <= input length`, so this
        // range is always in bounds; the fallback only satisfies the types.
        let start = usize::try_from(HEADER_LEN).unwrap_or(0);
        &self.buf[start..end]
    }

    /// The name region: bytes from `names_off` to the end of the file.
    #[must_use]
    pub fn names_region(&self) -> &'a [u8] {
        let start = if self.names_off == 0 {
            self.buf.len()
        } else {
            self.names_off as usize
        };
        &self.buf[start..]
    }

    /// Heuristic scan of the name region for printable runs: maximal runs of
    /// at least `min_len` bytes from letters, digits and `_:/{}. -`, starting
    /// with an alphanumeric. Real names are length-prefixed (`u8` length +
    /// bytes) with per-name metadata words that are still unknown, so this
    /// is a census aid, not a record parser.
    #[must_use]
    pub fn name_runs(&self, min_len: usize) -> Vec<NameRun<'a>> {
        fn is_body(b: u8) -> bool {
            b.is_ascii_alphanumeric()
                || matches!(b, b'_' | b':' | b'/' | b'{' | b'}' | b'.' | b' ' | b'-')
        }
        let region = self.names_region();
        let base = if self.names_off == 0 {
            u32::try_from(self.buf.len()).unwrap_or(u32::MAX)
        } else {
            self.names_off
        };
        let mut out = Vec::new();
        let mut i = 0usize;
        while i < region.len() {
            if region[i].is_ascii_alphanumeric() {
                let mut j = i + 1;
                while j < region.len() && is_body(region[j]) {
                    j += 1;
                }
                // Trim trailing blanks that glued two fields together.
                let mut end = j;
                while end > i + 1 && region[end - 1] == b' ' {
                    end -= 1;
                }
                if end - i >= min_len.max(1)
                    && let Ok(text) = std::str::from_utf8(&region[i..end])
                {
                    let at = u32::try_from(i).unwrap_or(u32::MAX);
                    out.push(NameRun {
                        offset: base.saturating_add(at),
                        text,
                    });
                }
                i = j.max(i + 1);
            } else {
                i += 1;
            }
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tiny_dat() -> Vec<u8> {
        // version 11, names at 24, objects 16..24, names "FILT" + "MASTER".
        let mut b = vec![0u8; 40];
        b[0..4].copy_from_slice(&11u32.to_le_bytes());
        b[4..8].copy_from_slice(&24u32.to_le_bytes());
        b[24] = 4;
        b[25..29].copy_from_slice(b"FILT");
        b[29..33].copy_from_slice(&7u32.to_le_bytes());
        b[33] = 6;
        b[34..40].copy_from_slice(b"MASTER");
        b
    }

    #[test]
    fn splits_objects_from_names() {
        let b = tiny_dat();
        let d = DatConfig::parse(&b).unwrap();
        assert_eq!(d.version(), 11);
        assert_eq!(d.names_off(), 24);
        assert_eq!(d.objects(), &[0u8; 8]);
        assert_eq!(d.names_region().len(), 16);
        let runs = d.name_runs(4);
        let texts: Vec<&str> = runs.iter().map(|r| r.text).collect();
        assert_eq!(texts, vec!["FILT", "MASTER"]);
        assert_eq!(runs[0].offset, 25);
    }

    #[test]
    fn zero_names_off_means_no_names() {
        let mut b = tiny_dat();
        b[4..8].copy_from_slice(&0u32.to_le_bytes());
        let d = DatConfig::parse(&b).unwrap();
        assert!(d.names_region().is_empty());
        assert!(d.name_runs(4).is_empty());
        assert_eq!(d.objects().len(), b.len() - 16);
    }

    #[test]
    fn rejects_bad_offsets() {
        assert_eq!(
            DatConfig::parse(&[0u8; 15]).unwrap_err().kind(),
            ErrorKind::Truncated
        );
        let mut b = tiny_dat();
        b[4..8].copy_from_slice(&400u32.to_le_bytes());
        assert_eq!(DatConfig::parse(&b).unwrap_err().kind(), ErrorKind::Invalid);
    }
}
