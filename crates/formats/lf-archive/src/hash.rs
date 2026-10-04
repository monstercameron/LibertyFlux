//! RPF3 content-hash function.
//!
//! RPF version 3 stores a 32-bit hash where RPF2 stores a name offset. The
//! function is Bob Jenkins' public-domain one-at-a-time hash over the
//! lowercased name with `\` normalised to `/`. A hash below 2 is raised by
//! 2 (values 0 and 1 never appear as content hashes).

/// Hash an entry name the way RPF3 content hashes are computed.
///
/// The name is lowercased (Unicode-aware) and backslashes become slashes
/// before hashing.
#[must_use]
pub fn name_hash(name: &str) -> u32 {
    let mut value: u32 = 0;
    for ch in name.chars() {
        let mut c = ch.to_lowercase().next().unwrap_or(ch) as u32;
        if c == '\\' as u32 {
            c = '/' as u32;
        }
        // One-at-a-time mix. Widen to u64 would change nothing: the
        // algorithm is defined on 32-bit wraparound.
        let temp = c.wrapping_add(value);
        value = temp.wrapping_add(temp << 10);
        value ^= value >> 6;
    }
    let mut temp = value.wrapping_add(value << 3);
    temp ^= temp >> 11;
    value = temp.wrapping_add(temp << 15);
    if value < 2 {
        value += 2;
    }
    value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn known_vectors() {
        // Verified against the public pyrpfiv hash-to-name table (MIT):
        // each name hashes to the key it is listed under.
        assert_eq!(name_hash("E1S4_MA_03"), 25024);
        assert_eq!(name_hash("MISSION_FAIL_RAGE_01"), 35310);
        assert_eq!(name_hash("E2BR1_ATH_01"), 55576);
        assert_eq!(name_hash("R12_A_AA_01"), 109148);
        assert_eq!(name_hash("GYM_BAG_HIT_1"), 176282);
        assert_eq!(name_hash("GCK_ACT_PKA_NOGLASS_BAD_01"), 285927);
    }

    #[test]
    fn normalisation() {
        assert_eq!(name_hash("AbC"), name_hash("abc"));
        assert_eq!(name_hash("a\\b"), name_hash("a/b"));
    }

    #[test]
    fn min_value() {
        assert!(name_hash("") >= 2);
    }
}
