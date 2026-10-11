//! Property, round-trip and fuzz tests on generated texture dictionaries.
//! Every byte is generated here (see `crates/formats/tests/support.rs`); no
//! game files are needed.
//!
//! - Property: a random dictionary (names, formats, sizes, mip chains,
//!   texel bytes) parses back to the same records, names and hashes, and
//!   every mip level slices to exactly the bytes that were written.
//! - Round trip: [`lf_texture::to_dds`] output read back with an
//!   independent DDS header reader gives the same dimensions, mip count,
//!   pixel format and texel bytes; uncompressed decodes match a reference.
//! - Fuzz: mutated system segments (re-wrapped in a valid container) and
//!   mutated texel data never panic in parsing, slicing, decoding or export.

// Fixture builders narrow random words and lengths into smaller fields on
// purpose, and the property checks compare floats that were written
// bit-exactly, so these pedantic lints do not apply here.
#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss,
    clippy::cast_precision_loss,
    clippy::float_cmp,
    clippy::format_push_string,
    clippy::format_collect,
    clippy::too_many_lines,
    clippy::type_complexity,
    clippy::many_single_char_names,
    clippy::similar_names
)]

#[path = "../../tests/support.rs"]
mod support;

use lf_texture::{
    D3DFormat, Dictionary, RESOURCE_TYPE_TEXTURE, decode_to_rgba8, hash_title, level_byte_size,
    level_dims, title_of, to_dds,
};
use support::{Buf, Rng, fuzz, rsc5, split_at_clamped};

/// Dictionary header length (system segment start).
const HEADER_LEN: usize = 32;
/// One texture record.
const RECORD_LEN: usize = 80;
/// System and graphics pointer tags.
const SYS: u32 = 0x5000_0000;
const GFX: u32 = 0x6000_0000;

const FORMATS: [u32; 5] = [
    D3DFormat::CODE_DXT1,
    D3DFormat::CODE_DXT3,
    D3DFormat::CODE_DXT5,
    D3DFormat::CODE_A8R8G8B8,
    D3DFormat::CODE_L8,
];

/// A generated texture: what the writer put in and the reader must return.
#[derive(Debug, Clone)]
struct Tex {
    name: String,
    width: u16,
    height: u16,
    format: u32,
    levels: u8,
    stride: u16,
    data: Vec<u8>,
}

fn random_textures(rng: &mut Rng, max: usize) -> Vec<Tex> {
    (0..rng.below(max + 1))
        .map(|i| {
            let width = 1u16 << rng.below(7);
            let height = 1u16 << rng.below(7);
            let format = *rng.pick(&FORMATS);
            let fmt = D3DFormat::from_code(format);
            let max_levels = 1 + (u32::from(width.max(height)).ilog2() as u8);
            let levels = 1 + rng.below(usize::from(max_levels)) as u8;
            let total: u64 = (0..levels)
                .map(|l| level_byte_size(fmt, width, height, l).unwrap())
                .sum();
            let stride = match fmt {
                D3DFormat::A8R8G8B8 => width * 4,
                D3DFormat::L8 => width,
                _ => 0,
            };
            Tex {
                name: format!("pack:/{}{i}.dds", rng.ident(1, 16)),
                width,
                height,
                format,
                levels,
                stride,
                data: rng.bytes(usize::try_from(total).unwrap()),
            }
        })
        .collect()
}

/// Lay out a dictionary: header, hash table, pointer list, records, names
/// in the system segment; texel data in the graphics segment.
fn write_dict(texs: &[Tex]) -> (Vec<u8>, Vec<u8>) {
    let n = texs.len();
    let hashes_at = HEADER_LEN;
    let list_at = hashes_at + n * 4;
    let records_at = (list_at + n * 4).next_multiple_of(16);
    let names_at = records_at + n * RECORD_LEN;
    let mut sys = Buf::zeroed(names_at);
    let mut gfx = Buf::new();
    sys.put_u32(0, 0x0069_5238)
        .put_u32(16, if n > 0 { SYS | hashes_at as u32 } else { 0 })
        .put_u16(20, n as u16)
        .put_u16(22, n as u16)
        .put_u32(24, if n > 0 { SYS | list_at as u32 } else { 0 })
        .put_u16(28, n as u16)
        .put_u16(30, n as u16);
    for (i, t) in texs.iter().enumerate() {
        let rec = records_at + i * RECORD_LEN;
        let name_at = sys.len();
        sys.bytes(t.name.as_bytes()).u8(0);
        gfx.align(16);
        let data_at = gfx.len();
        gfx.bytes(&t.data);
        sys.put_u32(hashes_at + i * 4, hash_title(title_of(&t.name)))
            .put_u32(list_at + i * 4, SYS | rec as u32)
            .put_u32(rec, 0x0069_5240)
            .put_u32(rec + 20, SYS | name_at as u32)
            .put_u16(rec + 28, t.width)
            .put_u16(rec + 30, t.height)
            .put_u32(rec + 32, t.format)
            .put_u16(rec + 36, t.stride)
            .put(rec + 38, &[0, t.levels])
            .put_f32(rec + 40, 1.0)
            .put_u32(rec + 72, GFX | data_at as u32);
    }
    (sys.0, gfx.0)
}

#[test]
fn dictionary_round_trip() {
    let mut rng = Rng::for_test("texture round trip");
    for _ in 0..80 {
        let texs = random_textures(&mut rng, 8);
        let (sys, gfx) = write_dict(&texs);
        let file = rsc5(RESOURCE_TYPE_TEXTURE, &sys, &gfx);
        let dict = Dictionary::parse(&file).expect("generated dictionary parses");
        assert_eq!(dict.len(), texs.len());
        for (entry, t) in dict.entries().iter().zip(&texs) {
            assert_eq!(entry.name, t.name);
            assert_eq!(entry.hash, hash_title(title_of(&t.name)));
            let r = &entry.record;
            assert_eq!((r.width, r.height, r.levels), (t.width, t.height, t.levels));
            assert_eq!(r.format.code(), t.format);
            assert_eq!(r.stride, t.stride);
            // Mip levels concatenate back to the written texel bytes.
            let mut joined = Vec::new();
            for level in 0..t.levels {
                let bytes = dict.level_data(entry, level).unwrap();
                let size = level_byte_size(r.format, r.width, r.height, level).unwrap();
                assert_eq!(bytes.len() as u64, size);
                joined.extend_from_slice(bytes);
            }
            assert_eq!(joined, t.data);
            assert!(dict.level_data(entry, t.levels).is_err());
            // Lookup by name finds this entry (names are unique).
            assert_eq!(dict.find(&t.name).map(|e| &e.name), Some(&t.name));
        }
    }
}

#[test]
fn uncompressed_decode_matches_reference() {
    let mut rng = Rng::for_test("texture decode");
    for _ in 0..40 {
        let texs = random_textures(&mut rng, 4);
        let (sys, gfx) = write_dict(&texs);
        let dict = Dictionary::parse(&rsc5(RESOURCE_TYPE_TEXTURE, &sys, &gfx)).unwrap();
        for entry in dict.entries() {
            for level in 0..entry.record.levels {
                let img = dict.decode_rgba8(entry, level).unwrap();
                let (w, h) = level_dims(entry.record.width, entry.record.height, level);
                assert_eq!((img.width, img.height), (w, h));
                assert_eq!(img.pixels.len(), (w * h * 4) as usize);
                let raw = dict.level_data(entry, level).unwrap();
                match entry.record.format {
                    D3DFormat::A8R8G8B8 => {
                        for (px, src) in img.pixels.chunks(4).zip(raw.chunks(4)) {
                            assert_eq!(px, [src[2], src[1], src[0], src[3]]);
                        }
                    }
                    D3DFormat::L8 => {
                        for (px, &l) in img.pixels.chunks(4).zip(raw) {
                            assert_eq!(px, [l, l, l, 255]);
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}

/// An independent reader for the DDS fields `to_dds` writes.
struct Dds {
    height: u32,
    width: u32,
    mips: u32,
    fourcc: u32,
    bits: u32,
    data: Vec<u8>,
}

fn read_dds(bytes: &[u8]) -> Dds {
    assert_eq!(&bytes[..4], b"DDS ");
    let word = |off: usize| u32::from_le_bytes(bytes[4 + off..8 + off].try_into().unwrap());
    assert_eq!(word(0), 124, "header size");
    assert_eq!(word(72), 32, "pixel format size");
    Dds {
        height: word(8),
        width: word(12),
        mips: word(24),
        fourcc: word(80),
        bits: word(84),
        data: bytes[128..].to_vec(),
    }
}

#[test]
fn dds_export_reads_back() {
    let mut rng = Rng::for_test("texture dds");
    for _ in 0..40 {
        let texs = random_textures(&mut rng, 4);
        let (sys, gfx) = write_dict(&texs);
        let dict = Dictionary::parse(&rsc5(RESOURCE_TYPE_TEXTURE, &sys, &gfx)).unwrap();
        for (entry, t) in dict.entries().iter().zip(&texs) {
            let dds = read_dds(&to_dds(&dict, entry).unwrap());
            assert_eq!(
                (dds.width, dds.height),
                (u32::from(t.width), u32::from(t.height))
            );
            assert_eq!(dds.mips, u32::from(t.levels));
            assert_eq!(dds.data, t.data);
            match D3DFormat::from_code(t.format) {
                D3DFormat::A8R8G8B8 => assert_eq!((dds.fourcc, dds.bits), (0, 32)),
                D3DFormat::L8 => assert_eq!((dds.fourcc, dds.bits), (0, 8)),
                _ => assert_eq!(dds.fourcc, t.format),
            }
        }
    }
}

/// Parse, then touch every level through every accessor.
fn exercise(file: &[u8]) {
    let Ok(dict) = Dictionary::parse(file) else {
        return;
    };
    for entry in dict.entries() {
        let _ = dict.find(&entry.name);
        for level in 0..entry.record.levels.min(16) {
            if let Ok(bytes) = dict.level_data(entry, level) {
                assert!(bytes.len() <= dict.resource().graphics.len());
            }
            let _ = dict.decode_rgba8(entry, level);
        }
        let _ = to_dds(&dict, entry);
    }
}

#[test]
fn fuzz_system_segment() {
    let mut rng = Rng::for_test("texture fuzz seeds");
    let seeds: Vec<(Vec<u8>, Vec<u8>)> = (0..3)
        .map(|_| {
            let mut texs = random_textures(&mut rng, 3);
            if texs.is_empty() {
                texs = random_textures(&mut rng, 3);
            }
            for t in &mut texs {
                // Keep fixtures small so mutations hit structure.
                t.width = t.width.min(8);
                t.height = t.height.min(8);
                t.levels = 1;
                let size = level_byte_size(D3DFormat::from_code(t.format), t.width, t.height, 0);
                t.data.truncate(usize::try_from(size.unwrap()).unwrap());
            }
            write_dict(&texs)
        })
        .collect();
    let gfx: Vec<Vec<u8>> = seeds.iter().map(|(_, g)| g.clone()).collect();
    let sys: Vec<Vec<u8>> = seeds.into_iter().map(|(s, _)| s).collect();
    fuzz("texture system segment", &sys, 3000, |s| {
        exercise(&rsc5(RESOURCE_TYPE_TEXTURE, s, &gfx[s.len() % gfx.len()]));
    });
    // Both segments together, split at the first seed's system length.
    let mut joined = sys[0].clone();
    let split = joined.len();
    joined.extend_from_slice(&gfx[0]);
    fuzz("texture both segments", &[joined], 1000, |b| {
        let (s, g) = split_at_clamped(b, split);
        exercise(&rsc5(RESOURCE_TYPE_TEXTURE, s, g));
    });
}

#[test]
fn fuzz_decoders_directly() {
    let mut rng = Rng::for_test("texture decoder fuzz");
    let seeds = vec![rng.bytes(64), rng.bytes(256)];
    fuzz("texture decoders", &seeds, 2000, |b| {
        // Derive the format and dimensions from the input itself.
        let pick = b.first().copied().unwrap_or(0);
        let format = D3DFormat::from_code(FORMATS[usize::from(pick) % FORMATS.len()]);
        let w = u32::from(b.get(1).copied().unwrap_or(1)) % 64;
        let h = u32::from(b.get(2).copied().unwrap_or(1)) % 64;
        if let Ok(px) = decode_to_rgba8(format, w, h, b) {
            assert_eq!(px.len(), (w * h * 4) as usize);
        }
    });
}

#[test]
fn whole_file_fuzz() {
    let mut rng = Rng::for_test("texture whole file seeds");
    let texs = random_textures(&mut rng, 2);
    let (sys, gfx) = write_dict(&texs);
    fuzz(
        "texture whole file",
        &[rsc5(RESOURCE_TYPE_TEXTURE, &sys, &gfx)],
        1000,
        exercise,
    );
}
