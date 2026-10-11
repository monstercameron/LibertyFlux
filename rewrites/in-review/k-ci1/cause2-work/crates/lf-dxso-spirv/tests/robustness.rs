//! Malformed input must give an error, never a panic, and whatever does
//! translate must pass the structural validator. Inputs are truncations,
//! single-bit flips and word substitutions of hand-built programs, plus
//! pseudo-random token streams.

mod common;

use common::*;
use lf_dxso_spirv::validate::validate;
use lf_dxso_spirv::{decode, decode_bytes, translate, translate_bytes};
use std::panic::{AssertUnwindSafe, catch_unwind};

/// xorshift32: deterministic, no dependency.
struct Rng(u32);

impl Rng {
    fn next(&mut self) -> u32 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        self.0 = x;
        x
    }
}

fn seeds() -> Vec<Vec<u32>> {
    let mut vs = Asm::vs_basic();
    vs.dcl(usage::TEXCOORD, 0, od(1));
    vs.defi(0, [3, 0, 1, 0]).def(10, [1.0, 2.0, 3.0, 4.0]);
    vs.op1(op::MOVA, d(reg::ADDR, 0).m("x"), c(0).sw("x"));
    vs.op2(op::M4X4, od(0), v(0), c(0));
    vs.flow(op::LOOP, 0, &[al(), i(0)]);
    vs.op2(op::ADD, rd(0), r(0), c(20).rel(AL));
    vs.flow(op::BREAKC, cmp::GT, &[r(0).sw("x"), c(10).sw("y")]);
    vs.flow(op::ENDLOOP, 0, &[]);
    vs.op2(op::ADD, rd(1), r(0), c(40).rel(A0X));
    vs.ins(op::SETP, cmp::LT, Some(d(reg::PRED, 0)), &[r(0), r(1)]);
    vs.pred(op::MOV, p0().sw("y"), od(1), &[r(1)]);
    vs.flow(op::CALL, 0, &[l(0)]);
    vs.flow(op::RET, 0, &[]);
    vs.flow(op::LABEL, 0, &[l(0)]);
    vs.op1(op::RCP, rd(2), r(1).sw("x"));
    vs.flow(op::RET, 0, &[]);

    let mut ps = Asm::ps_basic();
    ps.dcl_sampler(2, 0).dcl_sampler(3, 1).dcl_misc(0);
    ps.comment(&[u32::from_le_bytes(*b"CTAB"), 7, 8]);
    ps.op2(op::TEX, rd(0), v(0), sampler(0));
    ps.ins(op::TEX, 1, Some(rd(1)), &[v(0), sampler(1)]);
    ps.ins(op::TEXKILL, 0, Some(rd(0)), &[]);
    ps.flow(op::IF, 0, &[b(1)]);
    ps.op3(op::CMP, rd(2), r(0), r(1), c(3));
    ps.flow(op::ELSE, 0, &[]);
    ps.op1(op::DSX, rd(2), s(reg::MISC, 0));
    ps.flow(op::ENDIF, 0, &[]);
    ps.mov(oc(0).sat(), r(2));
    ps.mov(d(reg::DEPTHOUT, 0), r(2).sw("z"));
    vec![vs.end(), ps.end()]
}

fn run(words: &[u32]) {
    let result = catch_unwind(AssertUnwindSafe(|| {
        let _ = decode(words);
        if let Ok(m) = translate(words)
            && let Err(e) = validate(&m.words)
        {
            panic!("translated module failed validation: {e}\ninput: {words:08x?}");
        }
        let bytes: Vec<u8> = words.iter().flat_map(|w| w.to_le_bytes()).collect();
        let _ = decode_bytes(&bytes);
        let _ = translate_bytes(&bytes);
        if !bytes.is_empty() {
            let _ = translate_bytes(&bytes[..bytes.len() - 1]);
        }
    }));
    if let Err(e) = result {
        let msg = e
            .downcast_ref::<String>()
            .cloned()
            .or_else(|| e.downcast_ref::<&str>().map(|s| (*s).to_owned()))
            .unwrap_or_default();
        panic!("panic on input {words:08x?}: {msg}");
    }
}

#[test]
fn seeds_translate() {
    for s in seeds() {
        check(&s);
    }
}

#[test]
fn truncations_never_panic() {
    for s in seeds() {
        for n in 0..=s.len() {
            run(&s[..n]);
        }
    }
    run(&[]);
}

#[test]
fn bit_flips_never_panic() {
    for s in seeds() {
        for w in 0..s.len() {
            for bit in 0..32 {
                let mut m = s.clone();
                m[w] ^= 1 << bit;
                run(&m);
            }
        }
    }
}

#[test]
fn word_substitutions_never_panic() {
    let specials = [
        0,
        u32::MAX,
        0x8000_0000,
        0x0000_FFFF,
        0x0000_FFFE,
        0x7FFF_FFFE,
        0x0F00_0000,
        0x1F00_0001,
        0xFFFE_0300,
        0xFFFF_0300,
        0x8000_07FF,
        0xF000_1800,
    ];
    for s in seeds() {
        for w in 0..s.len() {
            for &x in &specials {
                let mut m = s.clone();
                m[w] = x;
                run(&m);
            }
        }
    }
}

#[test]
fn random_streams_never_panic() {
    let mut rng = Rng(0x1234_5678);
    let mut translated = 0usize;
    for round in 0..20_000 {
        let len = 1 + (rng.next() % 48) as usize;
        let mut w: Vec<u32> = (0..len).map(|_| rng.next()).collect();
        // Mostly valid headers, so the decoder gets past the version token.
        if round % 4 != 0 {
            w[0] = if round % 2 == 0 { VS30 } else { PS30 };
        }
        // Bias tokens towards plausible shapes: small opcodes and lengths.
        for t in w.iter_mut().skip(1) {
            match rng.next() % 4 {
                0 => *t = (*t & 0x0F00_0000) | (rng.next() % 97),
                1 => *t |= 0x8000_0000,
                _ => {}
            }
        }
        if round % 3 == 0 {
            w.push(END);
        }
        run(&w);
        if translate(&w).is_ok() {
            translated += 1;
        }
    }
    // Some random streams are valid programs; all of those validated above.
    eprintln!("random streams that translated: {translated}");
}
