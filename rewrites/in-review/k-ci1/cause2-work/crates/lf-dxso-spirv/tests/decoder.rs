//! Decoder field tests: each token field the decoder reads, set by the
//! test assembler and read back.

mod common;

use common::*;
use lf_dxso_spirv::decode::{Opcode, Payload, SrcMod, TextureType};
use lf_dxso_spirv::{Error, RegType, Stage, decode};

#[test]
fn version_tokens() {
    let vs = decode(&[VS30, END]).unwrap();
    assert_eq!((vs.stage, vs.major, vs.minor), (Stage::Vertex, 3, 0));
    let ps = decode(&[PS30, END]).unwrap();
    assert_eq!(ps.stage, Stage::Pixel);
    assert_eq!(ps.end_offset, 2);
    assert!(matches!(
        decode(&[0x1234_0300, END]),
        Err(Error::BadVersion(_))
    ));
    assert!(matches!(decode(&[]), Err(Error::BadLength(0))));
    assert_eq!(decode(&[PS30]), Err(Error::MissingEnd));
}

#[test]
fn register_types_split_across_two_fields() {
    // Types >= 8 need the high bits 12:11.
    for (t, kind) in [
        (reg::TEMP, RegType::Temp),
        (reg::CONSTINT, RegType::ConstInt),
        (reg::COLOROUT, RegType::ColorOut),
        (reg::SAMPLER, RegType::Sampler),
        (reg::CONSTBOOL, RegType::ConstBool),
        (reg::LOOP, RegType::Loop),
        (reg::MISC, RegType::MiscType),
        (reg::LABEL, RegType::Label),
        (reg::PRED, RegType::Predicate),
    ] {
        let w = Asm::ps().mov(rd(0), s(t, 5)).end();
        let sh = decode(&w).unwrap();
        assert_eq!(sh.instructions[0].src[0].reg.kind, kind, "type {t}");
        assert_eq!(sh.instructions[0].src[0].reg.num, 5);
        assert_eq!(kind.raw(), t);
    }
    // Register type 20+ does not exist.
    let w = Asm::ps().mov(rd(0), s(20, 0)).end();
    assert!(matches!(decode(&w), Err(Error::BadInstruction { .. })));
}

#[test]
fn destination_fields() {
    let w = Asm::ps()
        .mov(rd(7).m("xw").sat().pp().centroid(), r(1))
        .mov(rd(0).shift(0xF), r(1))
        .end();
    let sh = decode(&w).unwrap();
    let d0 = sh.instructions[0].dst.unwrap();
    assert_eq!(d0.reg.num, 7);
    assert_eq!(d0.write_mask, 0b1001);
    assert!(d0.saturate && d0.partial_precision && d0.centroid);
    assert_eq!(d0.shift, 0);
    assert_eq!(sh.instructions[1].dst.unwrap().shift, -1);
}

#[test]
fn source_fields_and_modifiers() {
    let mods = [
        (0, SrcMod::None),
        (sm::NEG, SrcMod::Neg),
        (sm::BIAS, SrcMod::Bias),
        (sm::BIASNEG, SrcMod::BiasNeg),
        (sm::SIGN, SrcMod::Sign),
        (sm::SIGNNEG, SrcMod::SignNeg),
        (sm::COMP, SrcMod::Comp),
        (sm::X2, SrcMod::X2),
        (sm::X2NEG, SrcMod::X2Neg),
        (sm::DZ, SrcMod::Dz),
        (sm::DW, SrcMod::Dw),
        (sm::ABS, SrcMod::Abs),
        (sm::ABSNEG, SrcMod::AbsNeg),
        (sm::NOT, SrcMod::Not),
    ];
    for (raw, m) in mods {
        let w = Asm::ps().mov(rd(0), r(1).sw("wzyx").md(raw)).end();
        let sh = decode(&w).unwrap();
        let s0 = sh.instructions[0].src[0];
        assert_eq!(s0.modifier, m);
        assert_eq!(s0.swizzle, [3, 2, 1, 0]);
    }
    let w = Asm::ps().mov(rd(0), r(1).md(14)).end();
    assert!(matches!(decode(&w), Err(Error::BadInstruction { .. })));
}

#[test]
fn relative_addressing_tokens() {
    let w = Asm::vs()
        .mov(rd(0), c(10).rel(Rel(reg::ADDR, 2)))
        .mov(od(1).rel(AL), c(3).rel(AL))
        .end();
    let sh = decode(&w).unwrap();
    let s0 = sh.instructions[0].src[0];
    let rel = s0.relative.unwrap();
    assert_eq!((rel.reg.kind, rel.component), (RegType::Addr, 2));
    let i1 = &sh.instructions[1];
    assert_eq!(i1.dst.unwrap().relative.unwrap().reg.kind, RegType::Loop);
    assert_eq!(i1.src[0].relative.unwrap().reg.kind, RegType::Loop);
    // A relative token that names a temp is rejected.
    let w = Asm::vs().mov(rd(0), c(10).rel(Rel(reg::TEMP, 0))).end();
    assert!(matches!(decode(&w), Err(Error::BadInstruction { .. })));
}

#[test]
fn predicate_token_follows_the_destination() {
    let w = Asm::ps()
        .pred(op::ADD, p0().sw("y").not(), rd(2), &[r(0), r(1)])
        .end();
    let sh = decode(&w).unwrap();
    let ins = &sh.instructions[0];
    let p = ins.predicate.unwrap();
    assert_eq!(p.reg.kind, RegType::Predicate);
    assert_eq!(p.modifier, SrcMod::Not);
    assert_eq!(p.swizzle, [1, 1, 1, 1]);
    assert_eq!(ins.dst.unwrap().reg.num, 2);
    assert_eq!(ins.src.len(), 2);
}

#[test]
fn declarations() {
    let w = Asm::ps()
        .dcl(usage::TEXCOORD, 7, vd(3).m("xy").centroid())
        .dcl_sampler(3, 4)
        .dcl_sampler(4, 5)
        .dcl_misc(1)
        .def(2, [1.0, -2.0, 0.5, 0.0])
        .defi(1, [-1, 2, 3, 4])
        .defb(3, true)
        .end();
    let sh = decode(&w).unwrap();
    let Payload::Dcl(d0) = sh.instructions[0].payload else {
        panic!()
    };
    assert_eq!((d0.usage, d0.usage_index), (5, 7));
    assert_eq!(d0.dst.write_mask, 0b0011);
    assert!(d0.dst.centroid);
    let Payload::Dcl(d1) = sh.instructions[1].payload else {
        panic!()
    };
    assert_eq!(d1.texture_type, TextureType::Cube);
    let Payload::Dcl(d2) = sh.instructions[2].payload else {
        panic!()
    };
    assert_eq!(d2.texture_type, TextureType::Volume);
    assert_eq!(sh.instructions[3].dst.unwrap().reg.kind, RegType::MiscType);
    assert_eq!(
        sh.instructions[4].payload,
        Payload::DefF([1.0f32, -2.0, 0.5, 0.0].map(f32::to_bits))
    );
    assert_eq!(sh.instructions[5].payload, Payload::DefI([-1, 2, 3, 4]));
    assert_eq!(sh.instructions[6].payload, Payload::DefB(true));
}

#[test]
fn comparison_and_texld_control_bits() {
    let w = Asm::ps()
        .flow(op::IFC, cmp::LE, &[r(0), r(1)])
        .flow(op::ENDIF, 0, &[])
        .ins(op::TEX, 2, Some(rd(0)), &[r(0), sampler(0)])
        .end();
    let sh = decode(&w).unwrap();
    assert_eq!(sh.instructions[0].opcode, Opcode::Ifc);
    assert_eq!(sh.instructions[0].control, 6);
    assert_eq!(sh.instructions[2].opcode, Opcode::Tex);
    assert_eq!(sh.instructions[2].control, 2);
}

#[test]
fn length_and_operand_count_errors() {
    // Declared length longer than the parameters (one stray word).
    let mut a = Asm::ps();
    a.raw(op::MOV | (3 << 24))
        .raw(regbits(reg::TEMP, 0) | (0xF << 16))
        .raw(regbits(reg::TEMP, 1) | (0xE4 << 16))
        .raw(regbits(reg::TEMP, 2) | (0xE4 << 16));
    assert!(matches!(
        decode(&a.end()),
        Err(Error::BadInstruction { .. })
    ));
    // Too few sources for add.
    let mut a = Asm::ps();
    a.raw(op::ADD | (2 << 24))
        .raw(regbits(reg::TEMP, 0) | (0xF << 16))
        .raw(regbits(reg::TEMP, 1) | (0xE4 << 16));
    assert!(matches!(
        decode(&a.end()),
        Err(Error::BadInstruction { .. })
    ));
    // Truncated: length runs past the end of the input.
    assert!(matches!(
        decode(&[PS30, op::MOV | (5 << 24), 0x8000_0000]),
        Err(Error::Truncated { offset: 1 })
    ));
    // Truncated comment.
    assert!(matches!(
        decode(&[PS30, 0xFFFE | (9 << 16), END]),
        Err(Error::Truncated { offset: 1 })
    ));
    // Unknown opcode.
    assert!(matches!(
        decode(&[PS30, 49, END]),
        Err(Error::UnknownOpcode { opcode: 49, .. })
    ));
    // Parameter token without bit 31.
    assert!(matches!(
        decode(&[PS30, op::MOV | (2 << 24), 0x000F_0000, 0x00E4_0001, END]),
        Err(Error::BadInstruction { .. })
    ));
}

#[test]
fn errors_display() {
    let msgs = [
        Error::BadLength(3).to_string(),
        Error::MissingEnd.to_string(),
        Error::UnsupportedVersion { major: 2, minor: 0 }.to_string(),
        Error::Unsupported {
            offset: 4,
            what: "x".into(),
        }
        .to_string(),
    ];
    assert!(msgs.iter().all(|m| !m.is_empty()));
}
