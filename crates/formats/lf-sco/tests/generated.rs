//! Round-trip, property and fuzz tests on generated scripts. Every byte is
//! generated here (see `crates/formats/tests/support.rs`); keys are random
//! test keys. No game files are needed.
//!
//! - Round trip: the 16-pass cipher's encrypt then decrypt is the identity;
//!   random code, statics and globals written as plain, encrypted and
//!   encrypted-zlib containers load back unchanged.
//! - Property: random instruction streams, encoded from the operand table in
//!   the crate documentation, decode back to the same opcodes, operands,
//!   offsets and sizes; every instruction formats without panicking, and a
//!   native name table resolves the hashes it was built from.
//! - Fuzz: mutated containers, code segments and JSON name tables never
//!   panic, and decoded instructions tile the code segment exactly.

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

use std::io::Write;

use flate2::Compression;
use flate2::write::ZlibEncoder;
use lf_sco::container::{self, PayloadKind};
use lf_sco::crypto::{self, Direction, KEY_LEN};
use lf_sco::isa::{self, Opcode, Operand};
use lf_sco::{disasm, natives::NativeDb};
use support::{Buf, Rng, fuzz};

/// Opcode bytes by operand shape, from the crate's ISA table.
const NO_OPERAND: [u8; 4] = [0x01, 0x2B, 0x31, 0x44];
const U32_OPERAND: [u8; 5] = [0x22, 0x23, 0x24, 0x2E, 0x29];
const OP_PUSH_U16: u8 = 0x28;
const OP_PUSH_F: u8 = 0x2A;
const OP_NATIVE: u8 = 0x2D;
const OP_ENTER: u8 = 0x2F;
const OP_LEAVE: u8 = 0x30;
const OP_SWITCH: u8 = 0x42;
const OP_STRING: u8 = 0x43;
const TEXT_LABEL: [u8; 4] = [0x45, 0x46, 0x47, 0x48];

/// One generated instruction: the bytes and the operand it must decode to.
fn random_instruction(rng: &mut Rng, natives: &[u32]) -> (Vec<u8>, Operand) {
    let mut b = Buf::new();
    let operand = match rng.below(11) {
        0 => {
            b.u8(*rng.pick(&NO_OPERAND));
            Operand::None
        }
        1 => {
            // Small integer push: 0x50..=0xFF.
            b.u8(0x50 + rng.below(0xB0) as u8);
            Operand::None
        }
        2 => {
            let v = rng.next_u32();
            b.u8(*rng.pick(&U32_OPERAND)).u32(v);
            Operand::U32(v)
        }
        3 => {
            let v = rng.next_u32() as u16;
            b.u8(OP_PUSH_U16).u16(v);
            Operand::U16(v)
        }
        4 => {
            let v = rng.f32_in(-1000.0, 1000.0);
            b.u8(OP_PUSH_F).f32(v);
            Operand::F32(v)
        }
        5 => {
            let (argc, retc, hash) = (rng.below(8) as u8, rng.below(2) as u8, *rng.pick(natives));
            b.u8(OP_NATIVE).u8(argc).u8(retc).u32(hash);
            Operand::Native { argc, retc, hash }
        }
        6 => {
            let (argc, frame_size) = (rng.below(8) as u8, rng.below(300) as u16);
            b.u8(OP_ENTER).u8(argc).u16(frame_size);
            Operand::Enter { argc, frame_size }
        }
        7 => {
            let (argc, retc) = (rng.below(8) as u8, rng.below(3) as u8);
            b.u8(OP_LEAVE).u8(argc).u8(retc);
            Operand::Leave { argc, retc }
        }
        8 => {
            let n = rng.below(6);
            b.u8(OP_SWITCH).u8(n as u8);
            let cases: Vec<isa::SwitchCase> = (0..n)
                .map(|_| isa::SwitchCase {
                    value: rng.next_u32(),
                    target: rng.next_u32(),
                })
                .collect();
            for c in &cases {
                b.u32(c.value).u32(c.target);
            }
            Operand::Switch(cases)
        }
        9 => {
            let s = rng.ident(0, 30).into_bytes();
            b.u8(OP_STRING).u8((s.len() + 1) as u8).bytes(&s).u8(0);
            Operand::String(s)
        }
        _ => {
            let len = rng.range(1, 64) as u8;
            b.u8(*rng.pick(&TEXT_LABEL)).u8(len);
            Operand::Len(len)
        }
    };
    (b.0, operand)
}

fn random_code(rng: &mut Rng, natives: &[u32]) -> (Vec<u8>, Vec<(u8, Operand, usize)>) {
    let mut code = Vec::new();
    let mut want = Vec::new();
    for _ in 0..rng.below(60) {
        let (bytes, operand) = random_instruction(rng, natives);
        want.push((bytes[0], operand, bytes.len()));
        code.extend_from_slice(&bytes);
    }
    (code, want)
}

#[test]
fn instruction_streams_decode_back() {
    let mut rng = Rng::for_test("sco isa");
    let natives: Vec<u32> = (0..8).map(|_| rng.next_u32()).collect();
    let json = format!(
        "[{}]",
        natives
            .iter()
            .enumerate()
            .map(|(i, h)| format!(
                "{{\"hash_int\":{h},\"name\":\"NATIVE_{i}\",\"alias_names\":[]}}"
            ))
            .collect::<Vec<_>>()
            .join(",")
    );
    let db = NativeDb::from_p0_natives_json(&json).expect("generated name table parses");
    assert_eq!(db.len(), natives.len());
    for _ in 0..200 {
        let (code, want) = random_code(&mut rng, &natives);
        let insts = isa::decode_all(&code).expect("generated code decodes");
        assert_eq!(insts.len(), want.len());
        let mut offset = 0usize;
        for (inst, (byte, operand, size)) in insts.iter().zip(&want) {
            assert_eq!(inst.offset as usize, offset);
            assert_eq!(inst.size as usize, *size);
            assert_eq!(Some(inst.opcode), Opcode::from_byte(*byte));
            assert_eq!(&inst.operand, operand);
            let text = disasm::format_instruction(inst, Some(&db));
            if let Operand::Native { hash, .. } = inst.operand {
                let name = db.lookup(hash).expect("hash from the table");
                assert!(text.contains(name), "{text} lacks {name}");
            }
            offset += size;
        }
        assert_eq!(offset, code.len());
        assert_eq!(disasm::disassemble(&code).unwrap(), insts);
    }
}

fn random_key(rng: &mut Rng) -> [u8; KEY_LEN] {
    let mut k = [0u8; KEY_LEN];
    k.copy_from_slice(&rng.bytes(KEY_LEN));
    k
}

#[test]
fn cipher_round_trips() {
    let mut rng = Rng::for_test("sco cipher");
    for _ in 0..50 {
        let key = random_key(&mut rng);
        let len = rng.below(100);
        let plain = rng.bytes(len);
        let mut data = plain.clone();
        crypto::transform(&mut data, &key, Direction::Encrypt).unwrap();
        crypto::transform(&mut data, &key, Direction::Decrypt).unwrap();
        assert_eq!(data, plain);
        assert!(crypto::transform(&mut data, &key[..31], Direction::Decrypt).is_err());
    }
}

/// Write a container of the given kind around code, statics and globals.
fn write_script(
    kind: PayloadKind,
    code: &[u8],
    statics: &[u32],
    globals: &[u32],
    args: u32,
    key: &[u8; KEY_LEN],
) -> Vec<u8> {
    let magic = match kind {
        PayloadKind::Plain => container::MAGIC_PLAIN,
        PayloadKind::V13Plain => container::MAGIC_V13_PLAIN,
        PayloadKind::Encrypted => container::MAGIC_ENCRYPTED,
        PayloadKind::EncryptedZlib => container::MAGIC_ENCRYPTED_ZLIB,
    };
    let mut b = Buf::new();
    b.u32(magic)
        .u32(code.len() as u32)
        .u32(statics.len() as u32)
        .u32(globals.len() as u32)
        .u32(args)
        .u32(0x1234_5678);
    let words = |v: &[u32]| v.iter().flat_map(|w| w.to_le_bytes()).collect::<Vec<u8>>();
    let (mut c, mut s, mut g) = (code.to_vec(), words(statics), words(globals));
    match kind {
        PayloadKind::Plain | PayloadKind::V13Plain => {
            b.bytes(&c).bytes(&s).bytes(&g);
        }
        PayloadKind::Encrypted => {
            for part in [&mut c, &mut s, &mut g] {
                crypto::transform(part, key, Direction::Encrypt).unwrap();
            }
            b.bytes(&c).bytes(&s).bytes(&g);
        }
        PayloadKind::EncryptedZlib => {
            let mut enc = ZlibEncoder::new(Vec::new(), Compression::default());
            enc.write_all(&[c, s, g].concat()).unwrap();
            let mut z = enc.finish().unwrap();
            crypto::transform(&mut z, key, Direction::Encrypt).unwrap();
            b.u32(z.len() as u32).bytes(&z);
        }
    }
    b.0
}

const KINDS: [PayloadKind; 4] = [
    PayloadKind::Plain,
    PayloadKind::V13Plain,
    PayloadKind::Encrypted,
    PayloadKind::EncryptedZlib,
];

#[test]
fn containers_round_trip() {
    let mut rng = Rng::for_test("sco container");
    for case in 0..80 {
        let kind = KINDS[case % KINDS.len()];
        let key = random_key(&mut rng);
        let (code, _) = random_code(&mut rng, &[1, 2, 3]);
        let statics: Vec<u32> = (0..rng.below(20)).map(|_| rng.next_u32()).collect();
        let globals: Vec<u32> = (0..rng.below(20)).map(|_| rng.next_u32()).collect();
        let args = rng.below(statics.len() + 1) as u32;
        let file = write_script(kind, &code, &statics, &globals, args, &key);
        let header = container::parse_header(&file).unwrap();
        assert_eq!(header.kind, kind);
        assert_eq!(header.globals_signature, 0x1234_5678);
        let needs_key = matches!(kind, PayloadKind::Encrypted | PayloadKind::EncryptedZlib);
        if needs_key {
            assert_eq!(
                container::load(&file, None),
                Err(container::LoadError::MissingKey)
            );
        }
        let script = container::load(&file, Some(&key)).expect("generated script loads");
        assert_eq!(script.code, code);
        assert_eq!(
            script
                .statics
                .iter()
                .map(|v| v.as_u32())
                .collect::<Vec<_>>(),
            statics
        );
        assert_eq!(
            script
                .globals
                .iter()
                .map(|v| v.as_u32())
                .collect::<Vec<_>>(),
            globals
        );
        assert_eq!(script.args().len(), args as usize);
        assert_eq!(
            script.plain_statics().len() + script.args().len(),
            statics.len()
        );
    }
}

#[test]
fn fuzz_containers() {
    let mut rng = Rng::for_test("sco container fuzz");
    let key = random_key(&mut rng);
    let seeds: Vec<Vec<u8>> = KINDS
        .iter()
        .map(|&kind| {
            let (code, _) = random_code(&mut rng, &[7]);
            write_script(kind, &code, &[1, 2, 3], &[4, 5], 1, &key)
        })
        .collect();
    fuzz("sco container", &seeds, 3000, |b| {
        let _ = container::parse_header(b);
        for k in [None, Some(&key)] {
            if let Ok(s) = container::load(b, k) {
                let _ = (s.plain_statics().len(), s.args().len());
                let _ = isa::decode_all(&s.code);
            }
        }
    });
}

#[test]
fn fuzz_code_segments() {
    let mut rng = Rng::for_test("sco code fuzz");
    let db = NativeDb::from_p0_natives_json("[{\"hash_int\":7,\"name\":\"N\"}]").unwrap();
    let seeds: Vec<Vec<u8>> = (0..4).map(|_| random_code(&mut rng, &[7]).0).collect();
    fuzz("sco code", &seeds, 4000, |code| {
        if let Ok(insts) = isa::decode_all(code) {
            let total: u32 = insts.iter().map(|i| i.size).sum();
            assert_eq!(total as usize, code.len());
            for i in &insts {
                let _ = disasm::format_instruction(i, Some(&db));
                let _ = (i.opcode.mnemonic(), i.opcode.is_control_flow());
            }
        }
        for at in [0, code.len() / 2, code.len()] {
            let _ = isa::decode_one(code, at);
        }
        let _ = disasm::escape(code);
    });
}

#[test]
fn fuzz_native_tables() {
    let seeds = vec![
        b"[{\"hash_int\":1,\"name\":\"A\",\"alias_names\":[\"B\"]},{\"hash_int\":4294967295,\"name\":\"C\"}]".to_vec(),
        b"[]".to_vec(),
    ];
    fuzz("sco natives json", &seeds, 2000, |b| {
        if let Ok(db) = NativeDb::from_p0_natives_json(&String::from_utf8_lossy(b)) {
            let _ = db.lookup(1);
        }
    });
}
