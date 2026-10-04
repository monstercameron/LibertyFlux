//! The version-14 script container: header, code, statics and globals.
//!
//! [`load`] parses the 24-byte header, then reads the payload according to
//! the magic: plain bytes, AES-decrypted segments, or one AES-decrypted
//! zlib stream holding all three segments concatenated.
//!
//! Sizes are checked with overflow-safe arithmetic before any slicing, so
//! corrupt length fields produce [`LoadError`] instead of panics.

use flate2::read::ZlibDecoder;
use std::io::Read;

use crate::crypto::{self, Direction, KEY_LEN};

/// Magic of an unencrypted script (`SCR\x0E` as little-endian `u32`).
pub const MAGIC_PLAIN: u32 = 0x0E52_4353;
/// Magic of an AES-encrypted script (`scr\x0E`).
pub const MAGIC_ENCRYPTED: u32 = 0x0E72_6373;
/// Magic of an AES-encrypted, zlib-compressed script (`Scr\x0E`).
pub const MAGIC_ENCRYPTED_ZLIB: u32 = 0x0E72_6353;
/// Magic of the version-13 leftover script (`SCR\r`).
///
/// Only one file in the game uses this: the unencrypted navgen script. Its
/// header and segments match version 14 field for field; only the magic
/// differs, and no encrypted v13 variant is known.
pub const MAGIC_V13_PLAIN: u32 = 0x0D52_4353;

/// Size of the fixed header in bytes.
pub const HEADER_LEN: usize = 24;

/// Cap on zlib output to bound memory on corrupt input (64 MiB).
pub const MAX_DECOMPRESSED_LEN: u64 = 64 * 1024 * 1024;

/// Cap on a single count field (16 Mi slots) to reject corrupt headers early.
pub const MAX_COUNT: u64 = 16 * 1024 * 1024;

/// How the payload after the header is stored.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PayloadKind {
    /// Plain code + statics + globals.
    Plain,
    /// Version-13 plain code + statics + globals (same layout, older magic).
    V13Plain,
    /// Each segment encrypted independently.
    Encrypted,
    /// One encrypted zlib stream holding all segments concatenated.
    EncryptedZlib,
}

/// The parsed 24-byte header.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Header {
    /// Raw magic word.
    pub magic: u32,
    /// How the payload is stored.
    pub kind: PayloadKind,
    /// Length of the code segment in bytes.
    pub code_len: u32,
    /// Number of 32-bit statics slots.
    pub statics_count: u32,
    /// Number of 32-bit globals slots.
    pub globals_count: u32,
    /// Trailing statics slots that are launch arguments.
    pub args_count: u32,
    /// Globals signature, or 0 when the script declares no globals.
    pub globals_signature: u32,
}

/// One 32-bit script value; the VM reinterprets the bits as int or float.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct ScriptValue(pub u32);

impl ScriptValue {
    /// The raw bits.
    #[must_use]
    pub fn as_u32(self) -> u32 {
        self.0
    }
    /// The bits as a signed integer.
    #[must_use]
    pub fn as_i32(self) -> i32 {
        i32::from_ne_bytes(self.0.to_ne_bytes())
    }
    /// The bits as an IEEE-754 float.
    #[must_use]
    pub fn as_f32(self) -> f32 {
        f32::from_le_bytes(self.0.to_le_bytes())
    }
}

/// A fully loaded script: header plus the three segments.
#[derive(Clone, Debug, PartialEq)]
pub struct Script {
    /// The parsed header.
    pub header: Header,
    /// Decoded bytecode; `header.code_len` bytes.
    pub code: Vec<u8>,
    /// Statics segment; the last `args_count` entries are launch arguments.
    pub statics: Vec<ScriptValue>,
    /// Globals segment.
    pub globals: Vec<ScriptValue>,
}

impl Script {
    /// Statics that are not launch arguments.
    #[must_use]
    pub fn plain_statics(&self) -> &[ScriptValue] {
        let args = (self.header.args_count as usize).min(self.statics.len());
        &self.statics[..self.statics.len() - args]
    }

    /// The launch-argument tail of the statics segment.
    #[must_use]
    pub fn args(&self) -> &[ScriptValue] {
        let args = (self.header.args_count as usize).min(self.statics.len());
        &self.statics[self.statics.len() - args..]
    }
}

/// Errors from [`parse_header`] and [`load`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LoadError {
    /// Fewer than [`HEADER_LEN`] bytes available.
    TooShort(usize),
    /// Unknown magic word.
    BadMagic(u32),
    /// The payload is encrypted but no key was supplied.
    MissingKey,
    /// A count field exceeds [`MAX_COUNT`].
    CountTooLarge {
        /// Which field (`code_len`, `statics_count` or `globals_count`).
        field: &'static str,
        /// The offending value.
        value: u64,
    },
    /// The declared segments do not fit in the input.
    TruncatedPayload {
        /// Bytes the header declares past the header.
        needed: u64,
        /// Bytes actually available past the header.
        available: u64,
    },
    /// The zlib stream failed to decode.
    Decompress(String),
    /// The zlib stream decoded to fewer bytes than the segments need.
    DecompressedTooShort {
        /// Bytes the segments need.
        needed: usize,
        /// Bytes the stream produced.
        got: usize,
    },
}

impl core::fmt::Display for LoadError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            LoadError::TooShort(n) => write!(f, "input too short for a header: {n} bytes"),
            LoadError::BadMagic(m) => write!(f, "unknown script magic 0x{m:08X}"),
            LoadError::MissingKey => write!(f, "script is encrypted but no key was given"),
            LoadError::CountTooLarge { field, value } => {
                write!(f, "{field} of {value} exceeds the sanity cap")
            }
            LoadError::TruncatedPayload { needed, available } => write!(
                f,
                "payload needs {needed} bytes past the header, only {available} available"
            ),
            LoadError::Decompress(msg) => write!(f, "zlib decode failed: {msg}"),
            LoadError::DecompressedTooShort { needed, got } => write!(
                f,
                "zlib stream gave {got} bytes but the segments need {needed}"
            ),
        }
    }
}

impl std::error::Error for LoadError {}

fn read_u32(bytes: &[u8], at: usize) -> u32 {
    u32::from_le_bytes([bytes[at], bytes[at + 1], bytes[at + 2], bytes[at + 3]])
}

/// Parse just the 24-byte header.
///
/// # Errors
///
/// Returns an error if the input is truncated or malformed.
pub fn parse_header(bytes: &[u8]) -> Result<Header, LoadError> {
    if bytes.len() < HEADER_LEN {
        return Err(LoadError::TooShort(bytes.len()));
    }
    let magic = read_u32(bytes, 0);
    let kind = match magic {
        MAGIC_PLAIN => PayloadKind::Plain,
        MAGIC_V13_PLAIN => PayloadKind::V13Plain,
        MAGIC_ENCRYPTED => PayloadKind::Encrypted,
        MAGIC_ENCRYPTED_ZLIB => PayloadKind::EncryptedZlib,
        _ => return Err(LoadError::BadMagic(magic)),
    };
    Ok(Header {
        magic,
        kind,
        code_len: read_u32(bytes, 4),
        statics_count: read_u32(bytes, 8),
        globals_count: read_u32(bytes, 12),
        args_count: read_u32(bytes, 16),
        globals_signature: read_u32(bytes, 20),
    })
}

fn check_counts(header: &Header) -> Result<(), LoadError> {
    for (field, value) in [
        ("code_len", u64::from(header.code_len)),
        ("statics_count", u64::from(header.statics_count)),
        ("globals_count", u64::from(header.globals_count)),
    ] {
        if value > MAX_COUNT {
            return Err(LoadError::CountTooLarge { field, value });
        }
    }
    Ok(())
}

/// Load a whole script file.
///
/// `key` must hold the 32-byte AES key when the magic calls for decryption,
/// and is ignored for plain files. Returns the header plus the decoded
/// segments, or a [`LoadError`]. Never panics on any input.
///
/// # Errors
///
/// Returns an error if the input is truncated or malformed.
pub fn load(bytes: &[u8], key: Option<&[u8; KEY_LEN]>) -> Result<Script, LoadError> {
    let header = parse_header(bytes)?;
    check_counts(&header)?;
    let code_len = header.code_len as usize;
    let statics_len = header.statics_count as usize * 4;
    let globals_len = header.globals_count as usize * 4;

    match header.kind {
        PayloadKind::Plain | PayloadKind::V13Plain | PayloadKind::Encrypted => {
            let total = code_len + statics_len + globals_len;
            let available = bytes.len() - HEADER_LEN;
            if total > available {
                return Err(LoadError::TruncatedPayload {
                    needed: total as u64,
                    available: available as u64,
                });
            }
            let mut code = bytes[HEADER_LEN..HEADER_LEN + code_len].to_vec();
            let mut statics_raw =
                bytes[HEADER_LEN + code_len..HEADER_LEN + code_len + statics_len].to_vec();
            let mut globals_raw =
                bytes[HEADER_LEN + code_len + statics_len..HEADER_LEN + total].to_vec();
            if header.kind == PayloadKind::Encrypted {
                let Some(key) = key else {
                    return Err(LoadError::MissingKey);
                };
                // Key length is fixed by the type, so this cannot fail.
                let _ = crypto::transform(&mut code, key, Direction::Decrypt);
                let _ = crypto::transform(&mut statics_raw, key, Direction::Decrypt);
                let _ = crypto::transform(&mut globals_raw, key, Direction::Decrypt);
            }
            Ok(Script {
                header,
                code,
                statics: words_to_values(&statics_raw),
                globals: words_to_values(&globals_raw),
            })
        }
        PayloadKind::EncryptedZlib => {
            let Some(key) = key else {
                return Err(LoadError::MissingKey);
            };
            if bytes.len() < HEADER_LEN + 4 {
                return Err(LoadError::TruncatedPayload {
                    needed: 4,
                    available: (bytes.len() - HEADER_LEN) as u64,
                });
            }
            let compressed_len = read_u32(bytes, HEADER_LEN) as usize;
            let start = HEADER_LEN + 4;
            if bytes.len() - start < compressed_len {
                return Err(LoadError::TruncatedPayload {
                    needed: compressed_len as u64,
                    available: (bytes.len() - start) as u64,
                });
            }
            let mut compressed = bytes[start..start + compressed_len].to_vec();
            let _ = crypto::transform(&mut compressed, key, Direction::Decrypt);
            let decoder = ZlibDecoder::new(&compressed[..]);
            let mut inflated = Vec::new();
            decoder
                .take(MAX_DECOMPRESSED_LEN)
                .read_to_end(&mut inflated)
                .map_err(|e| LoadError::Decompress(e.to_string()))?;
            let needed = code_len + statics_len + globals_len;
            if inflated.len() < needed {
                return Err(LoadError::DecompressedTooShort {
                    needed,
                    got: inflated.len(),
                });
            }
            Ok(Script {
                header,
                code: inflated[..code_len].to_vec(),
                statics: words_to_values(&inflated[code_len..code_len + statics_len]),
                globals: words_to_values(
                    &inflated[code_len + statics_len..code_len + statics_len + globals_len],
                ),
            })
        }
    }
}

fn words_to_values(raw: &[u8]) -> Vec<ScriptValue> {
    raw.chunks_exact(4)
        .map(|w| ScriptValue(u32::from_le_bytes([w[0], w[1], w[2], w[3]])))
        .collect()
}

#[cfg(test)]
#[allow(clippy::float_cmp)] // parsed values are compared bit for bit on purpose
mod tests {
    use super::*;
    use flate2::Compression;
    use flate2::write::ZlibEncoder;
    use std::io::Write;

    /// Hand-built plain script: 2-byte code, 3 statics, 1 global, 1 arg.
    fn plain_bytes() -> Vec<u8> {
        let mut v = Vec::new();
        v.extend_from_slice(&MAGIC_PLAIN.to_le_bytes());
        v.extend_from_slice(&2u32.to_le_bytes());
        v.extend_from_slice(&3u32.to_le_bytes());
        v.extend_from_slice(&1u32.to_le_bytes());
        v.extend_from_slice(&1u32.to_le_bytes());
        v.extend_from_slice(&0u32.to_le_bytes());
        v.extend_from_slice(&[0x60, 0x2C]); // PUSH_CONST_0, DROP
        v.extend_from_slice(&10u32.to_le_bytes());
        v.extend_from_slice(&20u32.to_le_bytes());
        v.extend_from_slice(&30u32.to_le_bytes());
        v.extend_from_slice(&40u32.to_le_bytes());
        v
    }

    #[test]
    fn plain_round_trip() {
        let script = load(&plain_bytes(), None).unwrap();
        assert_eq!(script.header.kind, PayloadKind::Plain);
        assert_eq!(script.code, vec![0x60, 0x2C]);
        assert_eq!(
            script.statics,
            vec![ScriptValue(10), ScriptValue(20), ScriptValue(30)]
        );
        assert_eq!(script.globals, vec![ScriptValue(40)]);
        assert_eq!(script.args(), &[ScriptValue(30)]);
        assert_eq!(script.plain_statics(), &[ScriptValue(10), ScriptValue(20)]);
        assert_eq!(ScriptValue(0x40490FDBu32).as_f32(), core::f32::consts::PI);
        assert_eq!(ScriptValue(0xFFFF_FFFFu32).as_i32(), -1);
    }

    #[test]
    fn v13_leftover_parses_as_plain() {
        let mut v13 = plain_bytes();
        v13[0..4].copy_from_slice(&MAGIC_V13_PLAIN.to_le_bytes());
        let script = load(&v13, None).unwrap();
        assert_eq!(script.header.kind, PayloadKind::V13Plain);
        assert_eq!(script.code, vec![0x60, 0x2C]);
        assert_eq!(script.statics.len(), 3);
    }

    #[test]
    fn header_errors() {
        assert_eq!(load(&[0u8; 10], None), Err(LoadError::TooShort(10)));
        let mut bad = plain_bytes();
        bad[0] = 0xAA;
        assert!(matches!(load(&bad, None), Err(LoadError::BadMagic(_))));
        let mut cut = plain_bytes();
        cut.truncate(cut.len() - 2);
        assert!(matches!(
            load(&cut, None),
            Err(LoadError::TruncatedPayload { .. })
        ));
        let mut huge = plain_bytes();
        huge[4..8].copy_from_slice(&u32::MAX.to_le_bytes());
        assert!(matches!(
            load(&huge, None),
            Err(LoadError::CountTooLarge { .. })
        ));
    }

    #[test]
    fn encrypted_needs_key_and_round_trips() {
        let key = [0x2Au8; KEY_LEN];
        let plain = plain_bytes();
        // Encrypt the three segments with the crate's own transform.
        let mut code = plain[24..26].to_vec();
        let mut statics = plain[26..38].to_vec();
        let mut globals = plain[38..42].to_vec();
        crypto::transform(&mut code, &key, Direction::Encrypt).unwrap();
        crypto::transform(&mut statics, &key, Direction::Encrypt).unwrap();
        crypto::transform(&mut globals, &key, Direction::Encrypt).unwrap();
        let mut enc = plain[..24].to_vec();
        enc[0..4].copy_from_slice(&MAGIC_ENCRYPTED.to_le_bytes());
        enc.extend_from_slice(&code);
        enc.extend_from_slice(&statics);
        enc.extend_from_slice(&globals);
        assert_eq!(load(&enc, None), Err(LoadError::MissingKey));
        let script = load(&enc, Some(&key)).unwrap();
        assert_eq!(script.code, vec![0x60, 0x2C]);
        assert_eq!(script.statics.len(), 3);
        assert_eq!(script.globals, vec![ScriptValue(40)]);
    }

    #[test]
    fn zlib_variant_round_trips() {
        let key = [0x7Fu8; KEY_LEN];
        let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
        // Same hand-built segments as plain_bytes().
        encoder.write_all(&[0x60u8, 0x2C]).unwrap();
        for w in [10u32, 20, 30, 40] {
            encoder.write_all(&w.to_le_bytes()).unwrap();
        }
        let mut compressed = encoder.finish().unwrap();
        crypto::transform(&mut compressed, &key, Direction::Encrypt).unwrap();
        let mut file = plain_bytes()[..24].to_vec();
        file[0..4].copy_from_slice(&MAGIC_ENCRYPTED_ZLIB.to_le_bytes());
        file.extend_from_slice(&(compressed.len() as u32).to_le_bytes());
        file.extend_from_slice(&compressed);
        let script = load(&file, Some(&key)).unwrap();
        assert_eq!(script.header.kind, PayloadKind::EncryptedZlib);
        assert_eq!(script.code, vec![0x60, 0x2C]);
        assert_eq!(script.statics.len(), 3);
        assert_eq!(script.globals.len(), 1);
    }
}
