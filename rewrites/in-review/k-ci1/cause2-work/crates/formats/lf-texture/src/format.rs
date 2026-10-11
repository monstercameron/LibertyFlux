//! Pixel formats: Direct3D format codes, texture kinds, mip sizing.
//!
//! All shipped dictionaries use five codes: three block-compressed S3TC
//! formats and two uncompressed ones. Unknown codes are preserved, not
//! rejected, so a reader keeps working when a new code turns up; only
//! decoding reports them as errors.

use crate::Error;

/// A Direct3D 9 pixel format code as stored in a texture record.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum D3DFormat {
    /// DXT1 / BC1: 4x4 blocks of 8 bytes, with or without 1-bit alpha.
    Dxt1,
    /// DXT3 / BC2: 4x4 blocks of 16 bytes, explicit 4-bit alpha.
    Dxt3,
    /// DXT5 / BC3: 4x4 blocks of 16 bytes, interpolated alpha.
    Dxt5,
    /// Uncompressed 32-bit BGRA, one byte per channel in memory order.
    A8R8G8B8,
    /// Uncompressed 8-bit luminance.
    L8,
    /// Any other code, kept verbatim.
    Unknown(u32),
}

impl D3DFormat {
    /// The raw codes, as stored little-endian in the record.
    pub const CODE_DXT1: u32 = 0x31_54_58_44;
    /// Raw code for DXT3.
    pub const CODE_DXT3: u32 = 0x33_54_58_44;
    /// Raw code for DXT5.
    pub const CODE_DXT5: u32 = 0x35_54_58_44;
    /// Raw code for A8R8G8B8 (D3DFMT value 21).
    pub const CODE_A8R8G8B8: u32 = 0x15;
    /// Raw code for L8 (D3DFMT value 50).
    pub const CODE_L8: u32 = 0x32;

    /// Map a raw code to its format.
    #[must_use]
    pub fn from_code(code: u32) -> Self {
        match code {
            Self::CODE_DXT1 => D3DFormat::Dxt1,
            Self::CODE_DXT3 => D3DFormat::Dxt3,
            Self::CODE_DXT5 => D3DFormat::Dxt5,
            Self::CODE_A8R8G8B8 => D3DFormat::A8R8G8B8,
            Self::CODE_L8 => D3DFormat::L8,
            other => D3DFormat::Unknown(other),
        }
    }

    /// Map a format back to its raw code.
    #[must_use]
    pub fn code(self) -> u32 {
        match self {
            D3DFormat::Dxt1 => Self::CODE_DXT1,
            D3DFormat::Dxt3 => Self::CODE_DXT3,
            D3DFormat::Dxt5 => Self::CODE_DXT5,
            D3DFormat::A8R8G8B8 => Self::CODE_A8R8G8B8,
            D3DFormat::L8 => Self::CODE_L8,
            D3DFormat::Unknown(code) => code,
        }
    }

    /// True for the three block-compressed formats.
    #[must_use]
    pub fn is_compressed(self) -> bool {
        matches!(self, D3DFormat::Dxt1 | D3DFormat::Dxt3 | D3DFormat::Dxt5)
    }

    /// Short display name; unknown codes render as hex.
    #[must_use]
    pub fn name(self) -> String {
        match self {
            D3DFormat::Dxt1 => "DXT1".to_string(),
            D3DFormat::Dxt3 => "DXT3".to_string(),
            D3DFormat::Dxt5 => "DXT5".to_string(),
            D3DFormat::A8R8G8B8 => "A8R8G8B8".to_string(),
            D3DFormat::L8 => "L8".to_string(),
            D3DFormat::Unknown(code) => format!("unknown({code:#010x})"),
        }
    }
}

impl std::fmt::Display for D3DFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.name())
    }
}

/// The kind byte of a texture record.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TextureKind {
    /// Plain 2D texture; the only kind observed in shipped files.
    Flat,
    /// Cube texture (unobserved).
    Cube,
    /// Volume texture (unobserved).
    Volume,
    /// Any other kind byte, kept verbatim.
    Unknown(u8),
}

impl TextureKind {
    /// Map a raw kind byte to its kind.
    #[must_use]
    pub fn from_byte(b: u8) -> Self {
        match b {
            0 => TextureKind::Flat,
            1 => TextureKind::Cube,
            3 => TextureKind::Volume,
            other => TextureKind::Unknown(other),
        }
    }
}

/// Dimensions of one mip level: each level halves both sides to a floor of 1.
#[must_use]
pub fn level_dims(width: u16, height: u16, level: u8) -> (u32, u32) {
    let shift = u32::from(level);
    let w = if shift >= 16 {
        1
    } else {
        u32::from(width) >> shift
    };
    let h = if shift >= 16 {
        1
    } else {
        u32::from(height) >> shift
    };
    (w.max(1), h.max(1))
}

/// Byte size of one mip level of the given format and base dimensions.
///
/// Block formats round dimensions up to whole 4x4 blocks. Unknown formats
/// and overflowing sizes are errors rather than guesses.
///
/// # Errors
///
/// Returns an error if the input is truncated or malformed.
pub fn level_byte_size(
    format: D3DFormat,
    width: u16,
    height: u16,
    level: u8,
) -> Result<u64, Error> {
    let (w, h) = level_dims(width, height, level);
    let w = u64::from(w);
    let h = u64::from(h);
    match format {
        D3DFormat::Dxt1 => {
            let bw = w.div_ceil(4);
            let bh = h.div_ceil(4);
            bw.checked_mul(bh)
                .and_then(|b| b.checked_mul(8))
                .ok_or(Error::Overflow)
        }
        D3DFormat::Dxt3 | D3DFormat::Dxt5 => {
            let bw = w.div_ceil(4);
            let bh = h.div_ceil(4);
            bw.checked_mul(bh)
                .and_then(|b| b.checked_mul(16))
                .ok_or(Error::Overflow)
        }
        D3DFormat::A8R8G8B8 => w
            .checked_mul(h)
            .and_then(|p| p.checked_mul(4))
            .ok_or(Error::Overflow),
        D3DFormat::L8 => w.checked_mul(h).ok_or(Error::Overflow),
        D3DFormat::Unknown(code) => Err(Error::UnknownFormat(code)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn codes_round_trip() {
        for code in [
            0x31_54_58_44u32,
            0x33_54_58_44,
            0x35_54_58_44,
            0x15,
            0x32,
            0xDEAD,
        ] {
            assert_eq!(D3DFormat::from_code(code).code(), code);
        }
    }

    #[test]
    fn mip_dims_halve() {
        assert_eq!(level_dims(256, 128, 0), (256, 128));
        assert_eq!(level_dims(256, 128, 1), (128, 64));
        assert_eq!(level_dims(256, 128, 8), (1, 1));
        assert_eq!(level_dims(3, 5, 1), (1, 2));
    }

    #[test]
    fn block_sizes() {
        let dxt1 = D3DFormat::Dxt1;
        assert_eq!(level_byte_size(dxt1, 4, 4, 0).unwrap(), 8);
        assert_eq!(level_byte_size(dxt1, 16, 16, 0).unwrap(), 128);
        // Non-multiple dimensions round up to whole blocks.
        assert_eq!(level_byte_size(dxt1, 6, 6, 0).unwrap(), 32);
        let dxt5 = D3DFormat::Dxt5;
        assert_eq!(level_byte_size(dxt5, 4, 4, 0).unwrap(), 16);
        assert_eq!(level_byte_size(D3DFormat::A8R8G8B8, 3, 5, 0).unwrap(), 60);
        assert_eq!(level_byte_size(D3DFormat::L8, 3, 5, 0).unwrap(), 15);
        assert!(level_byte_size(D3DFormat::Unknown(7), 4, 4, 0).is_err());
    }
}
