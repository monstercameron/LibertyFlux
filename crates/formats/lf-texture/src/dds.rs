//! In-memory DDS export for inspection in tests.
//!
//! [`to_dds`] packs one texture, with all its mip levels, into a standard
//! DDS byte buffer following the public Microsoft container layout, so test
//! tooling (image viewers, `ffmpeg`) can read it back independently. It is a
//! debugging aid, not a converter: only flat 2D textures are supported.

use crate::Error;
use crate::format::D3DFormat;
use crate::texture::{Dictionary, Entry};

/// Pack one texture and all its mip levels into a DDS file in memory.
///
/// # Errors
///
/// Returns an error if the input is truncated or malformed.
pub fn to_dds(dict: &Dictionary, entry: &Entry) -> Result<Vec<u8>, Error> {
    // Flag words of the public DDS header.
    const CAPS: u32 = 0x1;
    const HEIGHT: u32 = 0x2;
    const WIDTH: u32 = 0x4;
    const PITCH: u32 = 0x8;
    const PIXELFORMAT: u32 = 0x1000;
    const MIPMAPCOUNT: u32 = 0x20000;
    const LINEARSIZE: u32 = 0x80000;
    const FOURCC: u32 = 0x4;
    const RGB: u32 = 0x40;
    const ALPHA: u32 = 0x1;
    const LUMINANCE: u32 = 0x20000;
    const TEXTURE: u32 = 0x1000;
    const COMPLEX: u32 = 0x8;
    const MIPMAP: u32 = 0x40_0000;
    let rec = &entry.record;
    if rec.kind != crate::format::TextureKind::Flat {
        return Err(Error::DdsUnsupported("non-2D textures"));
    }
    let levels = rec.levels.max(1);
    let mut data = Vec::new();
    for level in 0..levels {
        data.extend_from_slice(dict.level_data(entry, level)?);
    }

    let mut header = [0u8; 124];
    let put = |h: &mut [u8; 124], off: usize, v: u32| {
        h[off..off + 4].copy_from_slice(&v.to_le_bytes());
    };

    let top = dict.level_data(entry, 0)?;
    let mut flags = CAPS | HEIGHT | WIDTH | PIXELFORMAT;
    if levels > 1 {
        flags |= MIPMAPCOUNT;
    }
    let pitch_or_linear: u32 = if rec.format.is_compressed() {
        flags |= LINEARSIZE;
        u32::try_from(top.len()).unwrap_or(u32::MAX)
    } else {
        flags |= PITCH;
        let bpp = match rec.format {
            D3DFormat::A8R8G8B8 => 4u32,
            _ => 1u32,
        };
        u32::from(rec.width) * bpp
    };
    put(&mut header, 0, 124);
    put(&mut header, 4, flags);
    put(&mut header, 8, u32::from(rec.height));
    put(&mut header, 12, u32::from(rec.width));
    put(&mut header, 16, pitch_or_linear);
    put(&mut header, 20, 0); // depth: 2D only
    put(&mut header, 24, u32::from(levels));
    // Pixel format sub-structure at offset 72.
    put(&mut header, 72, 32);
    match rec.format {
        D3DFormat::Dxt1 | D3DFormat::Dxt3 | D3DFormat::Dxt5 => {
            put(&mut header, 76, FOURCC);
            put(&mut header, 80, rec.format.code());
            put(&mut header, 84, 0);
        }
        D3DFormat::A8R8G8B8 => {
            put(&mut header, 76, RGB | ALPHA);
            put(&mut header, 80, 0);
            put(&mut header, 84, 32);
            put(&mut header, 88, 0x00FF_0000);
            put(&mut header, 92, 0x0000_FF00);
            put(&mut header, 96, 0x0000_00FF);
            put(&mut header, 100, 0xFF00_0000);
        }
        D3DFormat::L8 => {
            put(&mut header, 76, LUMINANCE);
            put(&mut header, 80, 0);
            put(&mut header, 84, 8);
            put(&mut header, 88, 0xFF);
            put(&mut header, 92, 0);
            put(&mut header, 96, 0);
            put(&mut header, 100, 0);
        }
        D3DFormat::Unknown(code) => return Err(Error::UnknownFormat(code)),
    }
    let mut caps = TEXTURE;
    if levels > 1 {
        caps |= COMPLEX | MIPMAP;
    }
    put(&mut header, 104, caps);

    let mut out = Vec::with_capacity(4 + 124 + data.len());
    out.extend_from_slice(b"DDS ");
    out.extend_from_slice(&header);
    out.extend_from_slice(&data);
    Ok(out)
}
