//! Pixel decoders: every shipped format to RGBA8, in pure Rust.
//!
//! The block formats follow the public S3TC specification: each 4x4 texel
//! block stores two RGB565 endpoints plus per-texel interpolation indices,
//! with DXT3 adding explicit 4-bit alpha and DXT5 interpolated alpha.
//! No lookup tables, no unsafe, no heap beyond the output image.

use crate::Error;
use crate::format::D3DFormat;

/// A decoded image: row-major RGBA8 bytes, top row first.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RgbaImage {
    /// Width in texels.
    pub width: u32,
    /// Height in texels.
    pub height: u32,
    /// `width * height * 4` bytes.
    pub pixels: Vec<u8>,
}

/// Decode one mip level's raw bytes to RGBA8.
///
/// `width` and `height` are the level's dimensions; `data` must hold at
/// least the level's byte size (see [`crate::level_byte_size`]).
/// Dimensions need not be multiples of 4: edge blocks are cropped.
///
/// # Errors
///
/// Returns an error if the input is truncated or malformed.
pub fn decode_to_rgba8(
    format: D3DFormat,
    width: u32,
    height: u32,
    data: &[u8],
) -> Result<Vec<u8>, Error> {
    // Guard the allocation against corrupt dimensions.
    let count = u64::from(width)
        .checked_mul(u64::from(height))
        .ok_or(Error::Overflow)?;
    let len = count.checked_mul(4).ok_or(Error::Overflow)?;
    if len > 1024 * 1024 * 1024 {
        return Err(Error::SegmentTooLarge {
            what: "decoded image",
            size: len,
        });
    }
    let count_len = usize::try_from(count).map_err(|_| Error::SegmentTooLarge {
        what: "decoded image",
        size: count,
    })?;
    let out_len = usize::try_from(len).map_err(|_| Error::SegmentTooLarge {
        what: "decoded image",
        size: len,
    })?;
    match format {
        D3DFormat::Dxt1 => decode_blocks(data, width, height, 8, decode_dxt1_block),
        D3DFormat::Dxt3 => decode_blocks(data, width, height, 16, decode_dxt3_block),
        D3DFormat::Dxt5 => decode_blocks(data, width, height, 16, decode_dxt5_block),
        D3DFormat::A8R8G8B8 => {
            let need = out_len;
            if data.len() < need {
                return Err(Error::TooShort {
                    what: "A8R8G8B8 data",
                });
            }
            let mut out = vec![0u8; need];
            for (dst, src) in out.chunks_exact_mut(4).zip(data.chunks_exact(4)) {
                // Memory order is B, G, R, A.
                dst[0] = src[2];
                dst[1] = src[1];
                dst[2] = src[0];
                dst[3] = src[3];
            }
            Ok(out)
        }
        D3DFormat::L8 => {
            let need = count_len;
            if data.len() < need {
                return Err(Error::TooShort { what: "L8 data" });
            }
            let mut out = vec![0u8; out_len];
            for (dst, &l) in out.chunks_exact_mut(4).zip(data.iter()) {
                dst[0] = l;
                dst[1] = l;
                dst[2] = l;
                dst[3] = 255;
            }
            Ok(out)
        }
        D3DFormat::Unknown(code) => Err(Error::UnknownFormat(code)),
    }
}

/// Expand a 5-bit channel to 8 bits.
fn expand5(v: u16) -> u8 {
    (((v << 3) | (v >> 2)) & 0xFF) as u8
}

/// Expand a 6-bit channel to 8 bits.
fn expand6(v: u16) -> u8 {
    (((v << 2) | (v >> 4)) & 0xFF) as u8
}

/// Split an RGB565 endpoint into 8-bit channels.
fn rgb565(c: u16) -> [u8; 3] {
    [
        expand5((c >> 11) & 0x1F),
        expand6((c >> 5) & 0x3F),
        expand5(c & 0x1F),
    ]
}

/// Walk the block grid, decoding each block and cropping edge blocks.
fn decode_blocks(
    data: &[u8],
    width: u32,
    height: u32,
    block_len: usize,
    decode: fn(&[u8], &mut [[u8; 4]; 16]),
) -> Result<Vec<u8>, Error> {
    let blocks_x = width.div_ceil(4) as usize;
    let blocks_y = height.div_ceil(4) as usize;
    let need = blocks_x * blocks_y * block_len;
    if data.len() < need {
        return Err(Error::TooShort {
            what: "block-compressed data",
        });
    }
    let mut out = vec![0u8; (width * height * 4) as usize];
    let mut texels = [[0u8; 4]; 16];
    for by in 0..blocks_y {
        for bx in 0..blocks_x {
            let block = &data[(by * blocks_x + bx) * block_len..][..block_len];
            decode(block, &mut texels);
            for ty in 0..4 {
                for tx in 0..4 {
                    let x = u32::try_from(bx)
                        .unwrap_or(u32::MAX)
                        .saturating_mul(4)
                        .saturating_add(tx);
                    let y = u32::try_from(by)
                        .unwrap_or(u32::MAX)
                        .saturating_mul(4)
                        .saturating_add(ty);
                    if x < width && y < height {
                        let dst = ((y * width + x) * 4) as usize;
                        out[dst..dst + 4].copy_from_slice(&texels[(ty * 4 + tx) as usize]);
                    }
                }
            }
        }
    }
    Ok(out)
}

/// Decode the 8-byte color part shared by all three block formats.
///
/// With `opaque` (DXT3/DXT5) the fourth palette entry of the
/// `c0 <= c1` case is opaque black; with DXT1 it is transparent black.
fn decode_color(block: &[u8], opaque: bool, texels: &mut [[u8; 4]; 16]) {
    let c0 = u16::from_le_bytes([block[0], block[1]]);
    let c1 = u16::from_le_bytes([block[2], block[3]]);
    let bits = u32::from_le_bytes([block[4], block[5], block[6], block[7]]);
    let p0 = rgb565(c0);
    let p1 = rgb565(c1);
    let mut pal = [[0u8; 4]; 4];
    pal[0] = [p0[0], p0[1], p0[2], 255];
    pal[1] = [p1[0], p1[1], p1[2], 255];
    if c0 > c1 {
        for c in 0..3 {
            let a = u16::from(p0[c]);
            let b = u16::from(p1[c]);
            pal[2][c] = u8::try_from((2 * a + b) / 3).unwrap_or(u8::MAX);
            pal[3][c] = u8::try_from((a + 2 * b) / 3).unwrap_or(u8::MAX);
        }
        pal[2][3] = 255;
        pal[3][3] = 255;
    } else {
        for c in 0..3 {
            pal[2][c] =
                u8::try_from(u16::midpoint(u16::from(p0[c]), u16::from(p1[c]))).unwrap_or(u8::MAX);
        }
        pal[2][3] = 255;
        pal[3] = if opaque { [0, 0, 0, 255] } else { [0, 0, 0, 0] };
    }
    for i in 0..16 {
        texels[i] = pal[((bits >> (2 * i)) & 3) as usize];
    }
}

fn decode_dxt1_block(block: &[u8], texels: &mut [[u8; 4]; 16]) {
    decode_color(block, false, texels);
}

fn decode_dxt3_block(block: &[u8], texels: &mut [[u8; 4]; 16]) {
    // 8 bytes of explicit alpha: one nibble per texel, low nibble first.
    let mut alpha = [0u8; 16];
    for i in 0..16 {
        let byte = block[i / 2];
        let nibble = if i % 2 == 0 { byte & 0xF } else { byte >> 4 };
        alpha[i] = nibble * 17;
    }
    decode_color(&block[8..16], true, texels);
    for i in 0..16 {
        texels[i][3] = alpha[i];
    }
}

fn decode_dxt5_block(block: &[u8], texels: &mut [[u8; 4]; 16]) {
    let a0 = block[0];
    let a1 = block[1];
    let mut packed: u64 = 0;
    for i in 0..6 {
        packed |= u64::from(block[2 + i]) << (8 * i);
    }
    let mut alpha = [0u8; 8];
    alpha[0] = a0;
    alpha[1] = a1;
    if a0 > a1 {
        for (slot, k) in alpha[2..8].iter_mut().zip(1u16..) {
            *slot =
                u8::try_from(((7 - k) * u16::from(a0) + k * u16::from(a1)) / 7).unwrap_or(u8::MAX);
        }
    } else {
        for (slot, k) in alpha[2..6].iter_mut().zip(1u16..) {
            *slot =
                u8::try_from(((5 - k) * u16::from(a0) + k * u16::from(a1)) / 5).unwrap_or(u8::MAX);
        }
        alpha[6] = 0;
        alpha[7] = 255;
    }
    decode_color(&block[8..16], true, texels);
    for i in 0..16 {
        texels[i][3] = alpha[((packed >> (3 * i)) & 7) as usize];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn solid_dxt1(c0: u16, c1: u16, index: u32) -> Vec<u8> {
        let mut b = Vec::new();
        b.extend_from_slice(&c0.to_le_bytes());
        b.extend_from_slice(&c1.to_le_bytes());
        let mut bits = 0u32;
        for i in 0..16 {
            bits |= index << (2 * i);
        }
        b.extend_from_slice(&bits.to_le_bytes());
        b
    }

    #[test]
    fn dxt1_solid_endpoints() {
        // Pure red vs pure blue endpoints, all texels index 0.
        let px = decode_to_rgba8(D3DFormat::Dxt1, 4, 4, &solid_dxt1(0xF800, 0x001F, 0)).unwrap();
        assert_eq!(px.len(), 64);
        assert!(px.chunks_exact(4).all(|p| p == [255, 0, 0, 255]));
        let px = decode_to_rgba8(D3DFormat::Dxt1, 4, 4, &solid_dxt1(0xF800, 0x001F, 1)).unwrap();
        assert!(px.chunks_exact(4).all(|p| p == [0, 0, 255, 255]));
    }

    #[test]
    fn dxt1_interpolation_and_transparency() {
        // c0 > c1: index 2 is (2*c0 + c1) / 3.
        let px = decode_to_rgba8(D3DFormat::Dxt1, 4, 4, &solid_dxt1(0xF800, 0x001F, 2)).unwrap();
        assert_eq!(&px[0..4], &[170, 0, 85, 255]);
        // c0 <= c1: index 3 is transparent black.
        let px = decode_to_rgba8(D3DFormat::Dxt1, 4, 4, &solid_dxt1(0x001F, 0xF800, 3)).unwrap();
        assert!(px.chunks_exact(4).all(|p| p == [0, 0, 0, 0]));
    }

    #[test]
    fn dxt3_explicit_alpha() {
        let mut block = vec![0u8; 16];
        // First texel alpha nibble 0xF, rest 0x0.
        block[0] = 0x0F;
        block[8..10].copy_from_slice(&0xFFFFu16.to_le_bytes());
        block[10..12].copy_from_slice(&0x0000u16.to_le_bytes());
        let px = decode_to_rgba8(D3DFormat::Dxt3, 4, 4, &block).unwrap();
        assert_eq!(&px[0..4], &[255, 255, 255, 255]);
        assert_eq!(&px[4..8], &[255, 255, 255, 0]);
        // Opaque rule: c0 <= c1 index 3 is opaque black, alpha still applies.
        let mut block = vec![0xFFu8; 8];
        block.extend_from_slice(&solid_dxt1(0x001F, 0xF800, 3));
        let px = decode_to_rgba8(D3DFormat::Dxt3, 4, 4, &block).unwrap();
        assert!(px.chunks_exact(4).all(|p| p == [0, 0, 0, 255]));
    }

    #[test]
    fn dxt5_interpolated_alpha() {
        // a0 = 255, a1 = 0, all indices 0 -> alpha 255 everywhere.
        let mut block = vec![255u8, 0];
        block.extend_from_slice(&[0u8; 6]);
        block.extend_from_slice(&solid_dxt1(0xFFFF, 0x0000, 0));
        let px = decode_to_rgba8(D3DFormat::Dxt5, 4, 4, &block).unwrap();
        assert!(px.chunks_exact(4).all(|p| p == [255, 255, 255, 255]));
        // a0 <= a1 path: index 6 -> 0, index 7 -> 255.
        let mut block = vec![10u8, 20];
        // First texel index 6 (0b110), second 7 (0b111).
        let packed: u64 = 0b111_110;
        for i in 0..6 {
            block.push(((packed >> (8 * i)) & 0xFF) as u8);
        }
        block.extend_from_slice(&solid_dxt1(0xFFFF, 0x0000, 0));
        let px = decode_to_rgba8(D3DFormat::Dxt5, 4, 4, &block).unwrap();
        assert_eq!(px[3], 0);
        assert_eq!(px[7], 255);
    }

    #[test]
    fn uncompressed_swizzle() {
        // Memory order B, G, R, A.
        let px = decode_to_rgba8(D3DFormat::A8R8G8B8, 2, 1, &[1, 2, 3, 4, 5, 6, 7, 8]).unwrap();
        assert_eq!(px, vec![3, 2, 1, 4, 7, 6, 5, 8]);
        let px = decode_to_rgba8(D3DFormat::L8, 2, 1, &[0, 200]).unwrap();
        assert_eq!(px, vec![0, 0, 0, 255, 200, 200, 200, 255]);
    }

    #[test]
    fn edge_blocks_crop() {
        // 6x6 needs 2x2 blocks; only 36 texels come out.
        let one = solid_dxt1(0xF800, 0x001F, 0);
        let data = [one.clone(), one.clone(), one.clone(), one].concat();
        let px = decode_to_rgba8(D3DFormat::Dxt1, 6, 6, &data).unwrap();
        assert_eq!(px.len(), 6 * 6 * 4);
        assert!(px.chunks_exact(4).all(|p| p == [255, 0, 0, 255]));
    }

    #[test]
    fn short_data_errors() {
        assert!(decode_to_rgba8(D3DFormat::Dxt1, 4, 4, &[0u8; 7]).is_err());
        assert!(decode_to_rgba8(D3DFormat::A8R8G8B8, 1, 1, &[0u8; 3]).is_err());
        assert!(decode_to_rgba8(D3DFormat::Unknown(9), 1, 1, &[0u8; 4]).is_err());
    }
}
