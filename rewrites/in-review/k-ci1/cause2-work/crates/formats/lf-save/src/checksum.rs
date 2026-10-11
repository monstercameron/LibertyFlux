//! The trailing checksum dword.
//!
//! Public documentation says the checksum is the wrapping sum of every byte
//! before it, computed with the header's size field temporarily holding a
//! fixed Games-for-Windows-Live-era value, and that the game ignores the
//! value on load. Neither the normalisation nor the summing rule could be
//! verified here (no save files exist on this machine), so verification sums
//! the stored bytes exactly as they stand. A mismatch therefore means "the
//! file differs from the documented rule", not "the file is corrupt".

/// Compute the documented checksum over the bytes preceding it: the
/// wrapping u32 sum of every byte.
#[must_use]
pub fn compute_checksum(data_before: &[u8]) -> u32 {
    data_before
        .iter()
        .fold(0u32, |sum, &b| sum.wrapping_add(u32::from(b)))
}

/// A stored checksum dword and where it sits in the file.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Checksum {
    /// File offset of the checksum dword.
    pub offset: u64,
    /// The stored value.
    pub stored: u32,
}

impl Checksum {
    /// Recompute the checksum over `data_before` (all file bytes preceding
    /// [`Checksum::offset`]) and compare with the stored value.
    #[must_use]
    pub fn verify(&self, data_before: &[u8]) -> bool {
        compute_checksum(data_before) == self.stored
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sums_and_wraps() {
        assert_eq!(compute_checksum(&[]), 0);
        assert_eq!(compute_checksum(&[1, 2, 3]), 6);
        assert_eq!(compute_checksum(&[0xFF; 4]), 0x3FC);
        // Wrapping: 2^24 copies of 0xFF sum to 0 mod 2^32.
        let many = vec![0xFFu8; 1 << 20];
        let expected = (0xFFu64 * (1 << 20)) as u32;
        assert_eq!(compute_checksum(&many), expected);
    }

    #[test]
    fn verify_compares() {
        let c = Checksum {
            offset: 3,
            stored: 6,
        };
        assert!(c.verify(&[1, 2, 3]));
        assert!(!c.verify(&[1, 2, 4]));
    }
}
