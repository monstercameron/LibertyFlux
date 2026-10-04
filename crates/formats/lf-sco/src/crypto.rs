//! AES helper for the script container.
//!
//! Each encrypted segment is transformed independently: AES-256 in ECB mode
//! with no padding, applied as 16 full passes over the largest 16-byte-aligned
//! prefix. Trailing bytes short of a block are stored as-is. Encryption and
//! decryption are the same loop with the block direction flipped.

use aes::Aes256;
use aes::cipher::{BlockDecrypt, BlockEncrypt, KeyInit};

/// Length in bytes of the script/archive AES key.
pub const KEY_LEN: usize = 32;

/// Number of ECB passes applied to a buffer.
pub const PASSES: usize = 16;

/// Direction of the block transform.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    /// Plain to cipher.
    Encrypt,
    /// Cipher to plain.
    Decrypt,
}

/// Transform `data` in place with the 16-pass ECB construction.
///
/// Only `data.len() & !15` bytes are touched; any trailing partial block is
/// left alone. Returns an error if the key is not [`KEY_LEN`] bytes.
///
/// # Errors
///
/// Returns an error if the input is truncated or malformed.
pub fn transform(data: &mut [u8], key: &[u8], direction: Direction) -> Result<(), CryptoError> {
    if key.len() != KEY_LEN {
        return Err(CryptoError::BadKeyLen(key.len()));
    }
    let cipher = Aes256::new_from_slice(key).map_err(|_| CryptoError::BadKeyLen(key.len()))?;
    let full_len = data.len() & !15;
    if full_len == 0 {
        return Ok(());
    }
    let (blocks, _) = data[..full_len].as_chunks_mut::<16>();
    for _ in 0..PASSES {
        for block in blocks.iter_mut() {
            match direction {
                Direction::Encrypt => cipher.encrypt_block(block.into()),
                Direction::Decrypt => cipher.decrypt_block(block.into()),
            }
        }
    }
    Ok(())
}

/// Errors from [`transform`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CryptoError {
    /// Key was not [`KEY_LEN`] bytes; carries the length seen.
    BadKeyLen(usize),
}

impl core::fmt::Display for CryptoError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            CryptoError::BadKeyLen(n) => write!(f, "key must be {KEY_LEN} bytes, got {n}"),
        }
    }
}

impl std::error::Error for CryptoError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trip_preserves_data() {
        let key = [0x11u8; KEY_LEN];
        // Hand-built pattern, two blocks plus a partial tail.
        let mut data: Vec<u8> = (0u32..40).map(|i| (i * 7 + 3) as u8).collect();
        let original = data.clone();
        transform(&mut data, &key, Direction::Encrypt).unwrap();
        assert_ne!(&data[..32], &original[..32]);
        // Partial tail untouched.
        assert_eq!(&data[32..], &original[32..]);
        transform(&mut data, &key, Direction::Decrypt).unwrap();
        assert_eq!(data, original);
    }

    #[test]
    fn short_buffer_is_untouched() {
        let key = [0u8; KEY_LEN];
        let mut data = vec![9u8; 7];
        transform(&mut data, &key, Direction::Decrypt).unwrap();
        assert_eq!(data, vec![9u8; 7]);
    }

    #[test]
    fn bad_key_len_errors() {
        let mut data = vec![0u8; 16];
        assert_eq!(
            transform(&mut data, &[1, 2, 3], Direction::Decrypt),
            Err(CryptoError::BadKeyLen(3))
        );
    }
}
