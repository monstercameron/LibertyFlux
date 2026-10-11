//! Clip-name hashing: Bob Jenkins' public-domain one-at-a-time hash.
//!
//! Dictionary hashes are the raw 32-bit hash of the clip's short name (the
//! file stem of its `pack:/<name>.anim` path). Names in shipped files are
//! already lowercase ASCII, so unlike the archive name hash there is no
//! case folding and no minimum-value adjustment.

/// Hash a clip short name the way dictionary hashes are computed.
///
/// Verified: `clip_hash("yawn")` is the hash stored beside the yawn clip,
/// and the rule holds for every shipped clip.
#[must_use]
pub fn clip_hash(name: &str) -> u32 {
    let mut value: u32 = 0;
    for byte in name.bytes() {
        let temp = u32::from(byte).wrapping_add(value);
        value = temp.wrapping_add(temp << 10);
        value ^= value >> 6;
    }
    let mut temp = value.wrapping_add(value << 3);
    temp ^= temp >> 11;
    value = temp.wrapping_add(temp << 15);
    value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verified_vector() {
        // Observed in a shipped single-clip dictionary: the stored hash of
        // the clip whose path is "pack:/yawn.anim".
        assert_eq!(clip_hash("yawn"), 0x8d6c_146d);
    }

    #[test]
    fn further_vectors() {
        // Observed in a shipped three-clip dictionary.
        assert_eq!(clip_hash("play_pinball"), 0x210c_f395);
        assert_eq!(clip_hash("use_vendmac"), 0x2bb0_8140);
        assert_eq!(clip_hash("play_videogame"), 0x870e_eed7);
    }

    #[test]
    fn case_matters() {
        assert_ne!(clip_hash("YAWN"), clip_hash("yawn"));
    }
}
