//! Property and fuzz tests on generated save files. Every byte is generated
//! here (see `crates/formats/tests/support.rs`); no save or game files are
//! needed.
//!
//! - Property: random saves (header with a UTF-16 mission name, any number
//!   of `BLOCK` records with random payloads, optional checksum, optional
//!   end block, optional trailing bytes) parse back to the same header,
//!   block offsets, kinds and payloads; the checksum verifies; payloads
//!   holding an RSC5 or RPF header are classified as such.
//! - Invariants: blocks tile the file from the header onwards, and header,
//!   blocks, checksum, end block and trailing bytes account for every byte.
//! - Fuzz: mutated saves never panic in parsing or in any accessor.

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

use lf_save::{BLOCK_HEADER_LEN, BLOCK_MAGIC, BlockKind, Payload, SaveFile};
use support::{Buf, Rng, fuzz};

const HEADER_LEN: usize = 0x110;
const END_LEN: usize = 0x16C;
const MISSION_UNITS: usize = 128;

#[derive(Debug, Clone)]
struct SaveSpec {
    version: u32,
    mission: String,
    payloads: Vec<Vec<u8>>,
    checksum: bool,
    end_word: Option<u32>,
    trailing: Vec<u8>,
}

fn random_save(rng: &mut Rng) -> SaveSpec {
    SaveSpec {
        version: rng.below(100) as u32,
        mission: rng.ident(0, 40),
        payloads: (0..rng.below(40))
            .map(|_| {
                let n = rng.below(300);
                let mut p = rng.bytes(n);
                // Keep payloads from starting with the end magic by chance.
                if p.starts_with(b"END") {
                    p[0] = 0;
                }
                p
            })
            .collect(),
        checksum: rng.chance(3, 4),
        end_word: rng.chance(3, 4).then(|| rng.next_u32()),
        trailing: if rng.chance(1, 4) {
            vec![0; rng.below(16) + 1]
        } else {
            Vec::new()
        },
    }
}

fn write_save(s: &SaveSpec) -> Vec<u8> {
    let mut b = Buf::new();
    b.u32(s.version).u32(0).u32(0).bytes(b"SAVE");
    let units: Vec<u16> = s.mission.encode_utf16().collect();
    for i in 0..MISSION_UNITS {
        b.u16(units.get(i).copied().unwrap_or(0));
    }
    assert_eq!(b.len(), HEADER_LEN);
    for p in &s.payloads {
        b.bytes(BLOCK_MAGIC)
            .u32(p.len() as u32 + BLOCK_HEADER_LEN)
            .bytes(p);
    }
    if s.checksum {
        let sum = lf_save::compute_checksum(&b.0);
        // A checksum whose first bytes spell the end magic would be read
        // as the end block; such a sum is vanishingly rare but avoided.
        assert_ne!(&sum.to_le_bytes()[..], b"END\0");
        b.u32(sum);
    }
    if let Some(word) = s.end_word {
        let mut end = vec![0u8; END_LEN];
        end[..4].copy_from_slice(b"END\0");
        end[4..8].copy_from_slice(&word.to_le_bytes());
        b.bytes(&end);
    }
    b.bytes(&s.trailing);
    b.0
}

#[test]
fn save_round_trip() {
    let mut rng = Rng::for_test("save round trip");
    for _ in 0..100 {
        let mut spec = random_save(&mut rng);
        // Trailing bytes after a missing end block would be read as one.
        if spec.end_word.is_none() && !spec.checksum {
            spec.trailing.clear();
        }
        let bytes = write_save(&spec);
        let save = SaveFile::parse(&bytes).expect("generated save parses");
        assert_eq!(save.len(), bytes.len());
        assert_eq!(save.header().version, spec.version);
        assert_eq!(save.header().mission(), spec.mission);
        assert_eq!(save.blocks().len(), spec.payloads.len());
        let mut at = HEADER_LEN as u64;
        for (i, (block, payload)) in save.blocks().iter().zip(&spec.payloads).enumerate() {
            assert_eq!(block.index, i);
            assert_eq!(block.kind, BlockKind::from_index(i));
            assert_eq!(block.offset, at);
            assert_eq!(save.payload(block), &payload[..]);
            assert_eq!(block.payload_len(), payload.len());
            at += u64::from(block.total_len);
        }
        assert_eq!(save.checksum().is_some(), spec.checksum);
        if spec.checksum {
            assert_eq!(save.verify_checksum(), Some(true));
        }
        assert_eq!(save.end().map(|e| e.word), spec.end_word);
        if spec.end_word.is_some() {
            assert_eq!(save.end_payload().unwrap().len(), END_LEN - 8);
        }
        assert_eq!(save.trailing_bytes(), &spec.trailing[..]);
        if let Some(first) = save.blocks().first() {
            assert_eq!(save.block(BlockKind::SimpleVars), Some(first));
        }
    }
}

#[test]
fn embedded_containers_are_classified() {
    let mut rng = Rng::for_test("save classify");
    let mut spec = random_save(&mut rng);
    let mut rsc = vec![0u8; 32];
    rsc[..4].copy_from_slice(b"RSC\x05");
    rsc[12..14].copy_from_slice(&0xDA78u16.to_le_bytes());
    let mut rpf = vec![0u8; 32];
    rpf[..4].copy_from_slice(b"RPF2");
    spec.payloads = vec![rsc, rpf, vec![1, 2, 3]];
    let bytes = write_save(&spec);
    let save = SaveFile::parse(&bytes).unwrap();
    let kinds: Vec<Payload> = save.blocks().iter().map(|b| save.classify(b)).collect();
    assert!(matches!(kinds[0], Payload::Resource { codec: 0xDA78, .. }));
    assert!(matches!(kinds[1], Payload::Rpf));
    assert!(matches!(kinds[2], Payload::Unknown));
}

/// Every accessor; the parts must account for the whole file.
fn exercise(bytes: &[u8]) {
    let Ok(save) = SaveFile::parse(bytes) else {
        return;
    };
    let mut used = HEADER_LEN;
    for b in save.blocks() {
        assert_eq!(b.offset as usize, used);
        used += b.total_len as usize;
        let _ = (save.payload(b).len(), save.classify(b), b.kind.name());
    }
    if save.checksum().is_some() {
        used += 4;
        let _ = save.verify_checksum();
    }
    if save.end().is_some() {
        used += END_LEN;
        let _ = save.end_payload();
    }
    assert_eq!(used + save.trailing_bytes().len(), bytes.len());
    let _ = save.header().mission();
}

#[test]
fn fuzz_saves() {
    let mut rng = Rng::for_test("save fuzz");
    let seeds: Vec<Vec<u8>> = (0..4)
        .map(|_| {
            let mut s = random_save(&mut rng);
            s.payloads.truncate(6);
            write_save(&s)
        })
        .collect();
    fuzz("save", &seeds, 3000, exercise);
    let _ = SaveFile::read_from(std::io::Cursor::new(&seeds[0])).unwrap();
}
