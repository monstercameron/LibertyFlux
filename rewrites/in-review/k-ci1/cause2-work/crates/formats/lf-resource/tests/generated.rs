//! Property and fuzz tests on generated resources. Every byte is generated
//! here (see `crates/formats/tests/support.rs`); no game files are needed.
//!
//! - Property: random system and graphics segments wrapped in an RSC5 file
//!   parse back to the same kind, flags and segment bytes, and every typed
//!   read through a tagged pointer returns the bytes that were written.
//! - Invariants: slices are bounds-checked, scanned pointers land inside
//!   their segment, and the inflated size always equals the flags formula.
//! - Fuzz: mutations of whole files and of the inflated segments (re-wrapped
//!   in a valid container) never panic.

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

use lf_resource::{Header, Pointer, Resource, ResourceKind, Segment};
use support::{Rng, fuzz, rsc5, split_at_clamped};

/// Resource kinds to draw from: the known ids plus arbitrary ones.
const KINDS: [u32; 6] = [0x01, 0x08, 0x20, 0x24, 0x6E, 0x70];

/// A random segment of up to `max` bytes (padded by the writer).
fn segment(rng: &mut Rng, max: usize) -> Vec<u8> {
    let len = rng.below(max + 1);
    rng.bytes(len)
}

#[test]
fn round_trip_random_segments() {
    let mut rng = Rng::for_test("resource round trip");
    for case in 0..200 {
        // Mostly small, sometimes past the 11-bit mantissa so a non-zero
        // exponent is needed.
        let max = if case % 20 == 0 { 600_000 } else { 4096 };
        let sys = segment(&mut rng, max);
        let gfx = segment(&mut rng, max / 2);
        let kind = if rng.chance(1, 4) {
            rng.next_u32()
        } else {
            *rng.pick(&KINDS)
        };
        let file = rsc5(kind, &sys, &gfx);
        let res = Resource::parse(&file).expect("generated resource parses");
        let header = res.header();
        assert_eq!(header.kind, ResourceKind::from_raw(kind));
        assert_eq!(
            header.flags,
            u32::from_le_bytes(file[8..12].try_into().unwrap())
        );
        let (sys_len, gfx_len) = header.segment_sizes().unwrap();
        assert_eq!(res.system().len(), sys_len);
        assert_eq!(res.graphics().len(), gfx_len);
        assert_eq!(header.total_size().unwrap(), sys_len + gfx_len);
        assert_eq!(&res.system()[..sys.len()], &sys[..]);
        assert_eq!(&res.graphics()[..gfx.len()], &gfx[..]);
        assert!(res.system()[sys.len()..].iter().all(|&b| b == 0));
        assert!(res.graphics()[gfx.len()..].iter().all(|&b| b == 0));
        // The reader path agrees with the slice path.
        let via_reader = Resource::parse_reader(std::io::Cursor::new(&file)).unwrap();
        assert_eq!(via_reader.system(), res.system());
        assert_eq!(via_reader.graphics(), res.graphics());
        // Header::parse on the prefix agrees too.
        assert_eq!(Header::parse(&file).unwrap(), *header);
    }
}

#[test]
fn typed_reads_follow_tagged_pointers() {
    let mut rng = Rng::for_test("resource typed reads");
    for _ in 0..100 {
        let sys = segment(&mut rng, 2048);
        let gfx = segment(&mut rng, 2048);
        let res = Resource::parse(&rsc5(1, &sys, &gfx)).unwrap();
        for (seg, bytes) in res.segments() {
            if bytes.len() < 4 {
                continue;
            }
            let off = rng.below(bytes.len() - 3);
            let ptr = Pointer::new((seg.tag() << 28) | u32::try_from(off).unwrap());
            assert_eq!(ptr.segment(), Some(seg));
            assert_eq!(ptr.offset(), off);
            let want = u32::from_le_bytes(bytes[off..off + 4].try_into().unwrap());
            assert_eq!(res.read_u32(ptr).unwrap(), want);
            assert_eq!(res.read_pointer(ptr).unwrap().raw(), want);
            let want16 = u16::from_le_bytes(bytes[off..off + 2].try_into().unwrap());
            assert_eq!(res.read_u16(ptr).unwrap(), want16);
            assert_eq!(res.slice(ptr, 4).unwrap(), &bytes[off..off + 4]);
            // One past the end is an error, never a panic.
            let end = Pointer::new((seg.tag() << 28) | u32::try_from(bytes.len()).unwrap());
            assert!(res.read_u32(end).is_err());
            assert!(res.slice(ptr, usize::MAX).is_err());
        }
        assert!(res.read_u32(Pointer::NULL).is_err());
    }
}

#[test]
fn cstring_reads_stop_at_nul() {
    let mut rng = Rng::for_test("resource cstrings");
    for _ in 0..100 {
        let name = rng.ident(1, 40);
        let mut sys = vec![0u8; 64];
        let at = rng.below(16);
        sys[at..at + name.len()].copy_from_slice(name.as_bytes());
        let res = Resource::parse(&rsc5(1, &sys, &[])).unwrap();
        let ptr = Pointer::new((Segment::System.tag() << 28) | u32::try_from(at).unwrap());
        assert_eq!(res.read_cstring(ptr, 256).unwrap(), name);
        // A window too short to reach the NUL is an error.
        assert!(res.read_cstring(ptr, name.len()).is_err());
    }
}

#[test]
fn scanned_pointers_land_inside_their_segment() {
    let mut rng = Rng::for_test("resource scan");
    for _ in 0..50 {
        let mut sys = segment(&mut rng, 1024);
        // Plant some pointers so the scan has something to find.
        for chunk in sys.chunks_exact_mut(4) {
            if rng.chance(1, 4) {
                let tag = if rng.chance(1, 2) {
                    0x5000_0000
                } else {
                    0x6000_0000
                };
                chunk.copy_from_slice(&(tag | (rng.next_u32() & 0x7FF)).to_le_bytes());
            }
        }
        let gfx = segment(&mut rng, 1024);
        let res = Resource::parse(&rsc5(1, &sys, &gfx)).unwrap();
        for (seg, bytes) in res.segments() {
            for (at, ptr) in res.scan_pointers(seg) {
                assert!(at + 4 <= bytes.len());
                assert!(at.is_multiple_of(4));
                let target = match ptr.segment().expect("scan yields tagged pointers") {
                    Segment::System => res.system(),
                    Segment::Graphics => res.graphics(),
                };
                assert!(ptr.offset() < target.len());
            }
        }
        let _ = res.pg_base();
        let _ = res.block_map();
    }
}

/// Everything the crate offers on one input; used by the fuzzers.
fn exercise(bytes: &[u8]) {
    let _ = Header::parse(bytes).map(|h| (h.segment_sizes(), h.total_size()));
    if let Ok(res) = Resource::parse(bytes) {
        let total = res
            .header()
            .total_size()
            .expect("parsed resource has a size");
        assert_eq!(res.system().len() + res.graphics().len(), total);
        let _ = res.pg_base();
        let _ = res.block_map();
        for (seg, _) in res.segments() {
            let _ = res.scan_pointers(seg).count();
        }
    }
}

#[test]
fn fuzz_whole_files() {
    let mut rng = Rng::for_test("resource fuzz seeds");
    let seeds: Vec<Vec<u8>> = (0..4)
        .map(|_| {
            let sys = segment(&mut rng, 512);
            let gfx = segment(&mut rng, 512);
            rsc5(*rng.pick(&KINDS), &sys, &gfx)
        })
        .collect();
    fuzz("resource whole file", &seeds, 3000, exercise);
}

#[test]
fn fuzz_inflated_segments() {
    // Mutate the inflated payload and re-wrap it, so the mutations reach
    // the block map and pointer readers instead of stopping at zlib.
    let mut sys = vec![0u8; 256];
    sys[0..4].copy_from_slice(&0x0069_5374u32.to_le_bytes());
    sys[4..8].copy_from_slice(&0x5000_0010u32.to_le_bytes());
    for (i, b) in sys.iter_mut().enumerate().skip(16) {
        *b = 0xCD ^ (i as u8);
    }
    let gfx = vec![0x11u8; 256];
    let mut seed = sys.clone();
    seed.extend_from_slice(&gfx);
    fuzz("resource segments", &[seed], 2000, |bytes| {
        let (s, g) = split_at_clamped(bytes, 256);
        exercise(&rsc5(1, s, g));
    });
}
