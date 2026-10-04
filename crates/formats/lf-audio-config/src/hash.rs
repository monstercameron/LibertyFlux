//! Jenkins one-at-a-time hash over lowercased names.
//!
//! Every cross-reference inside the audio metadata is a 32-bit hash of the
//! target's name: child sounds, curves, categories, variables, speech voices
//! and contexts. The function is Bob Jenkins' public-domain one-at-a-time
//! hash applied to the lowercased ASCII name, with no prefix or suffix.
//! Empty names hash to zero. Backslashes are hashed as-is; callers that
//! compare against display forms must normalise separators first.

/// Hash `name` the way the audio metadata cross-references it.
///
/// Lowercases ASCII letters before hashing. Non-ASCII bytes are hashed
/// unchanged.
#[must_use]
pub fn name_hash(name: &str) -> u32 {
    let mut hash: u32 = 0;
    for byte in name.bytes() {
        let lower = byte.to_ascii_lowercase();
        hash = hash.wrapping_add(u32::from(lower));
        hash = hash.wrapping_add(hash << 10);
        hash ^= hash >> 6;
    }
    hash = hash.wrapping_add(hash << 3);
    hash ^= hash >> 11;
    hash = hash.wrapping_add(hash << 15);
    hash
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn known_vectors() {
        // Hand-computed with an independent Python implementation of the
        // same public-domain algorithm; names are synthetic.
        assert_eq!(name_hash(""), 0);
        assert_eq!(name_hash("TEST_SOUND"), 0x047a4a179);
        assert_eq!(name_hash("test_sound"), 0x047a4a179);
        assert_eq!(name_hash("Hello"), 0xc8fd181b);
        assert_eq!(name_hash("a"), 0xca2e9442);
    }

    #[test]
    fn case_insensitive_ascii() {
        assert_eq!(name_hash("AbC_XyZ"), name_hash("abc_xyz"));
        assert_ne!(name_hash("abc"), name_hash("abd"));
    }
}
