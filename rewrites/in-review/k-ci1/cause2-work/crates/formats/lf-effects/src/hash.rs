//! Jenkins one-at-a-time hash: the key function for effect bank entries.
//!
//! Bank keys in [`crate::wpfl`] are the Jenkins hash of the lower-case entry
//! name, the same function the RPF3 container uses for entry names (see
//! `lf-archive`). Implemented here from the public-domain algorithm
//! description so this crate needs no hash dependency.

/// Compute the Jenkins one-at-a-time hash of `data`.
///
/// The hash runs over the raw bytes; effect names are ASCII so no
/// normalisation is applied.
#[must_use]
pub fn jenkins_oat(data: &[u8]) -> u32 {
    let mut h: u32 = 0;
    for &b in data {
        h = h.wrapping_add(u32::from(b));
        h = h.wrapping_add(h << 10);
        h ^= h >> 6;
    }
    h = h.wrapping_add(h << 3);
    h ^= h >> 11;
    h = h.wrapping_add(h << 15);
    h
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_hash_is_zero() {
        assert_eq!(jenkins_oat(b""), 0);
    }

    #[test]
    fn known_vectors() {
        // "a" is the published reference value for this algorithm.
        assert_eq!(jenkins_oat(b"a"), 0xca2e_9442);
        assert_eq!(jenkins_oat(b"exp_grenade"), 0x33e8_ee36);
        // Determinism and case sensitivity on effect-like names.
        assert_eq!(jenkins_oat(b"exp_grenade"), jenkins_oat(b"exp_grenade"));
        assert_ne!(jenkins_oat(b"exp_grenade"), jenkins_oat(b"EXP_GRENADE"));
    }
}
