//! Locating the PC AES key inside the owner's executable at run time.
//!
//! The 32-byte key is not stored anywhere by this crate. Instead, [`find_key`]
//! slides a 32-byte window over bytes the caller supplies (normally the
//! executable the user bought) and returns the first window whose SHA-1
//! digest equals the publicly documented key hash. The caller keeps the key
//! in memory only and passes it to [`crate::container::load`].
//!
//! The hash value below is public information: it appears in the key-search
//! routines of several open-source tools (rage-sc-tools `KeyStore`, MIT;
//! `RageLib` `KeyUtilGTAIV`, GPL). Only the hash is embedded, never the key.

use sha1::{Digest, Sha1};

use crate::crypto::KEY_LEN;

/// SHA-1 digest of the 32-byte GTA IV PC AES key, as published in
/// open-source key-search routines.
pub const KEY_SHA1: [u8; 20] = [
    0xDE, 0xA3, 0x75, 0xEF, 0x1E, 0x6E, 0xF2, 0x22, 0x3A, 0x12, 0x21, 0xC2, 0xC5, 0x75, 0xC4, 0x7B,
    0xF1, 0x7E, 0xFA, 0x5E,
];

/// Search `haystack` for the 32-byte window hashing to [`KEY_SHA1`].
///
/// Returns the key bytes on the first match, or `None` when no window
/// matches. The scan advances one byte at a time and reads only.
#[must_use]
pub fn find_key(haystack: &[u8]) -> Option<[u8; KEY_LEN]> {
    if haystack.len() < KEY_LEN {
        return None;
    }
    let mut digest = [0u8; 20];
    for window in haystack.windows(KEY_LEN) {
        let mut hasher = Sha1::new();
        hasher.update(window);
        let out = hasher.finalize();
        digest.copy_from_slice(&out);
        if digest == KEY_SHA1 {
            let mut key = [0u8; KEY_LEN];
            key.copy_from_slice(window);
            return Some(key);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_and_short_buffers_find_nothing() {
        assert_eq!(find_key(&[]), None);
        assert_eq!(find_key(&[0u8; KEY_LEN - 1]), None);
    }

    #[test]
    fn random_buffer_finds_nothing() {
        // Hand-built deterministic filler; overwhelmingly unlikely to
        // contain a window hashing to the documented digest.
        let filler: Vec<u8> = (0u32..4096)
            .map(|i| (i.wrapping_mul(2654435761u32) >> 11) as u8)
            .collect();
        assert_eq!(find_key(&filler), None);
    }
}
