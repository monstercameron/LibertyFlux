//! Archive cipher and run-time key location.
//!
//! The table cipher is AES-256 in ECB mode applied 16 times in succession
//! over the 16-byte-aligned prefix of the buffer; trailing bytes under 16
//! pass through unchanged. The same primitive covers RPF tables, IMG
//! headers and tables, and the one RPF whose file payloads are encrypted.
//!
//! The 32-byte key is not shipped with this crate and is never stored by
//! it. It is located at run time in the owner's game executable by the
//! publicly documented method: the 32-byte window whose SHA-1 digest equals
//! the published key hash. Well-known per-version offsets are tried first;
//! when none matches, the whole file is scanned at a 32-byte stride. The
//! key is kept in memory only: this module never prints, logs or writes it.

use aes::Aes256;
use aes::cipher::{BlockDecrypt, BlockEncrypt, KeyInit};

use crate::{Error, Result};

/// A 32-byte archive key, located at run time (see [`find_key`]).
pub type Key = [u8; 32];

/// SHA-1 digest identifying the archive key.
///
/// This is the published identifier used by every open-source tool that
/// reads these archives. It reveals nothing about the key itself.
pub const KEY_SHA1: [u8; 20] = [
    0xDE, 0xA3, 0x75, 0xEF, 0x1E, 0x6E, 0xF2, 0x22, 0x3A, 0x12, 0x21, 0xC2, 0xC5, 0x75, 0xC4, 0x7B,
    0xF1, 0x7E, 0xFA, 0x5E,
];

/// Published per-version file offsets of the key within the executable.
///
/// Newest builds first. These are positions, not key material.
pub const KEY_OFFSETS: &[u32] = &[
    0xC5_B73C, // 1.2.0.59
    0xC5_B33C, // 1.2.0.32 / 1.2.0.43
    0xC9_5FD8, // 1.0.8
    0xBE_7540, // 1.0.7
    0xBE_6540, // 1.0.6
    0xBE_1370, // 1.0.4r2
    0xB7_AEF4, // 1.0.4
    0xB7_5C9C, // 1.0.3
    0xB5_6BC4, // 1.0.2
    0xB6_07C4, // 1.0.1
    0xA9_4204, // 1.0
];

/// Decrypt a table region in place: AES-256-ECB applied 16 times over the
/// 16-byte-aligned prefix. Trailing bytes under 16 are left unchanged.
pub fn decrypt_tables(data: &mut [u8], key: &Key) {
    crypt_tables(data, key, false);
}

/// Encrypt a table region in place (the inverse of [`decrypt_tables`]).
///
/// The game ships encrypted tables only; this exists so tests can build
/// encrypted fixtures from hand-written plaintext.
pub fn encrypt_tables(data: &mut [u8], key: &Key) {
    crypt_tables(data, key, true);
}

fn crypt_tables(data: &mut [u8], key: &Key, encrypt: bool) {
    let cipher = Aes256::new(key.into());
    let full = data.len() & !0x0F;
    for _ in 0..16 {
        for block in data[..full].chunks_mut(16) {
            if encrypt {
                cipher.encrypt_block(block.into());
            } else {
                cipher.decrypt_block(block.into());
            }
        }
    }
}

/// True when the 32-byte window's SHA-1 digest equals `hash`.
fn window_matches(window: &[u8], hash: &[u8; 20]) -> bool {
    use sha1::Digest;
    if window.len() != 32 {
        return false;
    }
    let digest = sha1::Sha1::digest(window);
    digest.as_slice() == hash
}

fn find_key_with_hash(executable: &[u8], hash: &[u8; 20], offsets: &[u32]) -> Option<Key> {
    for &offset in offsets {
        let start = offset as usize;
        if let Some(window) = executable.get(start..start + 32)
            && window_matches(window, hash)
        {
            let mut key = [0u8; 32];
            key.copy_from_slice(window);
            return Some(key);
        }
    }
    let windows = executable.len() / 32;
    for i in 0..windows {
        let window = &executable[i * 32..i * 32 + 32];
        if window_matches(window, hash) {
            let mut key = [0u8; 32];
            key.copy_from_slice(window);
            return Some(key);
        }
    }
    None
}

/// Locate the archive key in executable bytes.
///
/// Tries the published per-version offsets first, then scans the whole
/// buffer at a 32-byte stride. Returns the key bytes on success; reports
/// only success or failure, never key material.
#[must_use]
pub fn find_key(executable: &[u8]) -> Option<Key> {
    find_key_with_hash(executable, &KEY_SHA1, KEY_OFFSETS)
}

/// Read an executable file and locate the archive key within it.
///
/// Returns [`Error::KeyNotFound`] when no 32-byte window matches the
/// published hash. The key is returned in memory only.
///
/// # Errors
///
/// Returns an error if the input is truncated or malformed.
pub fn load_key_from_exe(path: impl AsRef<std::path::Path>) -> Result<Key> {
    let bytes = std::fs::read(path)?;
    find_key(&bytes).ok_or(Error::KeyNotFound)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A fixed key for round-trip tests. Random test bytes, not game data:
    /// they never leave the test and match no published hash.
    const TEST_KEY: Key = [
        0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE,
        0xFF, 0x10, 0x32, 0x54, 0x76, 0x98, 0xBA, 0xDC, 0xFE, 0x01, 0x23, 0x45, 0x67, 0x89, 0xAB,
        0xCD, 0xEF,
    ];

    #[test]
    fn crypt_round_trip() {
        let plain: Vec<u8> = (0..100).collect();
        let mut buf = plain.clone();
        encrypt_tables(&mut buf, &TEST_KEY);
        assert_ne!(buf, plain);
        decrypt_tables(&mut buf, &TEST_KEY);
        assert_eq!(buf, plain);
    }

    #[test]
    fn crypt_tail_passes_through() {
        // 20 bytes: 16 encrypted, 4 untouched.
        let mut buf: Vec<u8> = (0..20).collect();
        encrypt_tables(&mut buf, &TEST_KEY);
        assert_eq!(&buf[16..20], &[16, 17, 18, 19]);
        decrypt_tables(&mut buf, &TEST_KEY);
        let plain: Vec<u8> = (0..20).collect();
        assert_eq!(buf, plain);
    }

    #[test]
    fn crypt_short_buffer_is_identity() {
        let mut buf = [7u8; 9];
        decrypt_tables(&mut buf, &TEST_KEY);
        assert_eq!(buf, [7u8; 9]);
    }

    #[test]
    fn find_key_miss() {
        let exe = vec![0x90u8; 4096];
        assert!(find_key(&exe).is_none());
    }

    #[test]
    fn find_key_offset_and_scan_paths() {
        // Planted-plaintext test of both lookup paths with a synthetic
        // hash target (test bytes only, unrelated to any real key).
        use sha1::Digest;
        let planted: Vec<u8> = (0..32u8)
            .map(|b| b.wrapping_mul(7).wrapping_add(1))
            .collect();
        let digest = sha1::Sha1::digest(&planted);
        let mut want = [0u8; 20];
        want.copy_from_slice(digest.as_slice());

        // Offset path: window at a listed offset.
        let mut exe = vec![0x90u8; 512];
        exe[64..96].copy_from_slice(&planted);
        let found = find_key_with_hash(&exe, &want, &[0x40]);
        assert_eq!(found.unwrap().as_slice(), planted.as_slice());

        // Scan path: window at an unlisted 32-multiple.
        let mut exe = vec![0x90u8; 512];
        exe[7 * 32..8 * 32].copy_from_slice(&planted);
        let found = find_key_with_hash(&exe, &want, &[0x10]);
        assert_eq!(found.unwrap().as_slice(), planted.as_slice());
    }
}
