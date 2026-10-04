//! Label hash used by the GXT text database.
//!
//! Labels are hashed with the Jenkins one-at-a-time function over the label
//! bytes, with ASCII upper-case letters (`A`–`Z`) folded to lower-case before
//! mixing. The stored key in each [`crate::GxtEntry`] is this hash; the label
//! text itself is not present in the file.

/// Hash a GXT text label such as `MO_OFF` to its 32-bit key.
///
/// The input is treated as raw bytes; only ASCII `A`–`Z` are case folded, so
/// callers may pass labels in any ASCII case. Non-ASCII input is hashed by
/// its UTF-8 bytes, which has no defined meaning in the format.
#[must_use]
pub fn label_hash(label: &str) -> u32 {
    let mut hash: u32 = 0;
    for mut byte in label.bytes() {
        if byte.is_ascii_uppercase() {
            byte += 32;
        }
        hash = hash.wrapping_add(u32::from(byte));
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
    fn case_insensitive_ascii() {
        assert_eq!(label_hash("MO_OFF"), label_hash("mo_off"));
        assert_eq!(label_hash("MO_OFF"), label_hash("Mo_Off"));
    }

    #[test]
    fn known_vectors() {
        // Vectors produced by this implementation and cross-checked against
        // the game's key tables: each of these labels resolves in american.gxt.
        assert_eq!(label_hash("MO_OFF"), 0xf1e1_a233);
        assert_eq!(label_hash("MO_ON"), 0xc75a_b6f3);
        assert_eq!(label_hash("BLIPS"), 0xe5eb_c699);
    }

    #[test]
    fn digits_and_underscore_stable() {
        // Digits and '_' must hash unchanged (no case fold side effects).
        assert_eq!(label_hash("RADIO_10"), label_hash("radio_10"));
        assert_ne!(label_hash("RADIO_10"), label_hash("RADIO_11"));
    }
}
