//! The dictionary name hash: Jenkins one-at-a-time over the texture title.
//!
//! Each dictionary stores a hash beside every texture pointer. The hash is
//! computed over the title: the stored name without its `pack:/` prefix and
//! `.dds` suffix. The algorithm lowercases ASCII letters and treats
//! backslashes as slashes before mixing, and clamps results below 2 up to 2.
//! Verified against every texture of every reachable dictionary: each stored
//! hash equals this function applied to the stored name's title.

/// Strip the `pack:/` prefix and `.dds` suffix from a stored texture name.
///
/// Names that lack either part are returned unchanged, so the function is
/// total over whatever a file contains.
#[must_use]
pub fn title_of(name: &str) -> &str {
    let mut title = name;
    if let Some(rest) = title.strip_prefix("pack:/") {
        title = rest;
    }
    if let Some(rest) = title.strip_suffix(".dds") {
        title = rest;
    }
    title
}

/// Hash a texture title the way the dictionary hash table does.
#[must_use]
pub fn hash_title(title: &str) -> u32 {
    let mut value: u32 = 0;
    for mut byte in title.bytes() {
        if byte == b'\\' {
            byte = b'/';
        }
        if byte.is_ascii_uppercase() {
            byte = byte.to_ascii_lowercase();
        }
        // Wrapping: the algorithm is defined on 32-bit words.
        let mut temp = u32::from(byte).wrapping_add(value);
        value = temp << 10;
        temp = temp.wrapping_add(value);
        value = temp >> 6;
        value ^= temp;
    }
    let mut temp = (value << 3).wrapping_add(value);
    temp ^= temp >> 11;
    value = (temp << 15).wrapping_add(temp);
    if value < 2 {
        value += 2;
    }
    value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn titles_strip() {
        assert_eq!(title_of("pack:/radar_police.dds"), "radar_police");
        assert_eq!(title_of("plain"), "plain");
        assert_eq!(title_of("pack:/noext"), "noext");
    }

    #[test]
    fn hash_is_case_and_slash_insensitive() {
        assert_eq!(hash_title("AbC"), hash_title("abc"));
        assert_eq!(hash_title("a\\b"), hash_title("a/b"));
    }

    #[test]
    fn hash_fixed_vector() {
        // Fixed by this implementation and cross-checked with an
        // independent Python implementation during development.
        assert_eq!(hash_title("radar_police"), 0x0758_7f5e);
    }
}
