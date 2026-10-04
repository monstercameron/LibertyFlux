//! A tiny Direct3D 9 shader model 3 assembler for tests, plus helpers that
//! translate, validate and inspect the SPIR-V. Every program the tests
//! build is written here by hand from the public token format; none comes
//! from game files.

#![allow(dead_code)]

pub mod interp;
pub mod programs;

use lf_dxso_spirv::validate::{self, Report, instructions, read_string};
use lf_dxso_spirv::{Error, Module, Options, translate_with};
use std::fmt::Write as _;

pub const VS30: u32 = 0xFFFE_0300;
pub const PS30: u32 = 0xFFFF_0300;
pub const END: u32 = 0x0000_FFFF;

/// Register file numbers.
pub mod reg {
    pub const TEMP: u32 = 0;
    pub const INPUT: u32 = 1;
    pub const CONST: u32 = 2;
    pub const ADDR: u32 = 3;
    pub const RASTOUT: u32 = 4;
    pub const OUTPUT: u32 = 6;
    pub const CONSTINT: u32 = 7;
    pub const COLOROUT: u32 = 8;
    pub const DEPTHOUT: u32 = 9;
    pub const SAMPLER: u32 = 10;
    pub const CONSTBOOL: u32 = 14;
    pub const LOOP: u32 = 15;
    pub const MISC: u32 = 17;
    pub const LABEL: u32 = 18;
    pub const PRED: u32 = 19;
}

/// Opcode numbers.
pub mod op {
    pub const NOP: u32 = 0;
    pub const MOV: u32 = 1;
    pub const ADD: u32 = 2;
    pub const SUB: u32 = 3;
    pub const MAD: u32 = 4;
    pub const MUL: u32 = 5;
    pub const RCP: u32 = 6;
    pub const RSQ: u32 = 7;
    pub const DP3: u32 = 8;
    pub const DP4: u32 = 9;
    pub const MIN: u32 = 10;
    pub const MAX: u32 = 11;
    pub const SLT: u32 = 12;
    pub const SGE: u32 = 13;
    pub const EXP: u32 = 14;
    pub const LOG: u32 = 15;
    pub const LIT: u32 = 16;
    pub const DST: u32 = 17;
    pub const LRP: u32 = 18;
    pub const FRC: u32 = 19;
    pub const M4X4: u32 = 20;
    pub const M4X3: u32 = 21;
    pub const M3X4: u32 = 22;
    pub const M3X3: u32 = 23;
    pub const M3X2: u32 = 24;
    pub const CALL: u32 = 25;
    pub const CALLNZ: u32 = 26;
    pub const LOOP: u32 = 27;
    pub const RET: u32 = 28;
    pub const ENDLOOP: u32 = 29;
    pub const LABEL: u32 = 30;
    pub const DCL: u32 = 31;
    pub const POW: u32 = 32;
    pub const CRS: u32 = 33;
    pub const SGN: u32 = 34;
    pub const ABS: u32 = 35;
    pub const NRM: u32 = 36;
    pub const SINCOS: u32 = 37;
    pub const REP: u32 = 38;
    pub const ENDREP: u32 = 39;
    pub const IF: u32 = 40;
    pub const IFC: u32 = 41;
    pub const ELSE: u32 = 42;
    pub const ENDIF: u32 = 43;
    pub const BREAK: u32 = 44;
    pub const BREAKC: u32 = 45;
    pub const MOVA: u32 = 46;
    pub const DEFB: u32 = 47;
    pub const DEFI: u32 = 48;
    pub const TEXKILL: u32 = 65;
    pub const TEX: u32 = 66;
    pub const TEXBEM: u32 = 67;
    pub const EXPP: u32 = 78;
    pub const LOGP: u32 = 79;
    pub const CND: u32 = 80;
    pub const DEF: u32 = 81;
    pub const CMP: u32 = 88;
    pub const DP2ADD: u32 = 90;
    pub const DSX: u32 = 91;
    pub const DSY: u32 = 92;
    pub const TEXLDD: u32 = 93;
    pub const SETP: u32 = 94;
    pub const TEXLDL: u32 = 95;
    pub const BREAKP: u32 = 96;
}

/// Comparison codes for ifc / breakc / setp.
pub mod cmp {
    pub const GT: u32 = 1;
    pub const EQ: u32 = 2;
    pub const GE: u32 = 3;
    pub const LT: u32 = 4;
    pub const NE: u32 = 5;
    pub const LE: u32 = 6;
}

/// Usage numbers.
pub mod usage {
    pub const POSITION: u32 = 0;
    pub const BLENDWEIGHT: u32 = 1;
    pub const BLENDINDICES: u32 = 2;
    pub const NORMAL: u32 = 3;
    pub const PSIZE: u32 = 4;
    pub const TEXCOORD: u32 = 5;
    pub const TANGENT: u32 = 6;
    pub const BINORMAL: u32 = 7;
    pub const COLOR: u32 = 10;
    pub const FOG: u32 = 11;
}

/// Source modifier numbers.
pub mod sm {
    pub const NEG: u32 = 1;
    pub const BIAS: u32 = 2;
    pub const BIASNEG: u32 = 3;
    pub const SIGN: u32 = 4;
    pub const SIGNNEG: u32 = 5;
    pub const COMP: u32 = 6;
    pub const X2: u32 = 7;
    pub const X2NEG: u32 = 8;
    pub const DZ: u32 = 9;
    pub const DW: u32 = 10;
    pub const ABS: u32 = 11;
    pub const ABSNEG: u32 = 12;
    pub const NOT: u32 = 13;
}

/// A length as a token field.
pub fn len_u32(n: usize) -> u32 {
    u32::try_from(n).expect("length fits a token")
}

pub fn regbits(t: u32, n: u32) -> u32 {
    0x8000_0000 | ((t & 7) << 28) | (((t >> 3) & 3) << 11) | (n & 0x7FF)
}

fn swz_bits(s: [u8; 4]) -> u32 {
    u32::from(s[0]) | (u32::from(s[1]) << 2) | (u32::from(s[2]) << 4) | (u32::from(s[3]) << 6)
}

/// Parse a swizzle like "xyzw", "x", "wzyx".
pub fn swz(text: &str) -> [u8; 4] {
    let mut out = [0u8; 4];
    let comps: Vec<u8> = text
        .chars()
        .map(|c| match c {
            'x' | 'r' => 0,
            'y' | 'g' => 1,
            'z' | 'b' => 2,
            'w' | 'a' => 3,
            _ => panic!("bad swizzle {text}"),
        })
        .collect();
    for (i, slot) in out.iter_mut().enumerate() {
        *slot = *comps.get(i).unwrap_or_else(|| comps.last().unwrap());
    }
    out
}

/// Parse a write mask like "xyzw", "xz".
pub fn mask(text: &str) -> u8 {
    text.chars()
        .map(|c| match c {
            'x' => 1,
            'y' => 2,
            'z' => 4,
            'w' => 8,
            _ => panic!("bad mask {text}"),
        })
        .fold(0, |a, b| a | b)
}

/// Relative address: (register file, component).
#[derive(Clone, Copy, Debug)]
pub struct Rel(pub u32, pub u8);

#[derive(Clone, Copy, Debug)]
pub struct D {
    pub t: u32,
    pub n: u32,
    pub mask: u8,
    pub mods: u32,
    pub shift: u32,
    pub rel: Option<Rel>,
}

pub fn d(t: u32, n: u32) -> D {
    D {
        t,
        n,
        mask: 0xF,
        mods: 0,
        shift: 0,
        rel: None,
    }
}

impl D {
    pub fn m(mut self, text: &str) -> D {
        self.mask = mask(text);
        self
    }
    pub fn raw_mask(mut self, m: u8) -> D {
        self.mask = m;
        self
    }
    pub fn sat(mut self) -> D {
        self.mods |= 1;
        self
    }
    pub fn pp(mut self) -> D {
        self.mods |= 2;
        self
    }
    pub fn centroid(mut self) -> D {
        self.mods |= 4;
        self
    }
    pub fn shift(mut self, s: u32) -> D {
        self.shift = s & 0xF;
        self
    }
    pub fn rel(mut self, r: Rel) -> D {
        self.rel = Some(r);
        self
    }
    pub fn tokens(&self) -> Vec<u32> {
        let mut t = regbits(self.t, self.n)
            | (u32::from(self.mask) << 16)
            | (self.mods << 20)
            | (self.shift << 24);
        if self.rel.is_some() {
            t |= 1 << 13;
        }
        let mut v = vec![t];
        if let Some(Rel(rt, c)) = self.rel {
            v.push(regbits(rt, 0) | (swz_bits([c; 4]) << 16));
        }
        v
    }
}

#[derive(Clone, Copy, Debug)]
pub struct S {
    pub t: u32,
    pub n: u32,
    pub swz: [u8; 4],
    pub m: u32,
    pub rel: Option<Rel>,
}

pub fn s(t: u32, n: u32) -> S {
    S {
        t,
        n,
        swz: [0, 1, 2, 3],
        m: 0,
        rel: None,
    }
}

impl S {
    pub fn sw(mut self, text: &str) -> S {
        self.swz = swz(text);
        self
    }
    pub fn md(mut self, m: u32) -> S {
        self.m = m;
        self
    }
    pub fn neg(self) -> S {
        self.md(sm::NEG)
    }
    pub fn not(self) -> S {
        self.md(sm::NOT)
    }
    pub fn rel(mut self, r: Rel) -> S {
        self.rel = Some(r);
        self
    }
    pub fn tokens(&self) -> Vec<u32> {
        let mut t = regbits(self.t, self.n) | (swz_bits(self.swz) << 16) | (self.m << 24);
        if self.rel.is_some() {
            t |= 1 << 13;
        }
        let mut v = vec![t];
        if let Some(Rel(rt, c)) = self.rel {
            v.push(regbits(rt, 0) | (swz_bits([c; 4]) << 16));
        }
        v
    }
}

// Shorthands for registers.
pub fn r(n: u32) -> S {
    s(reg::TEMP, n)
}
pub fn rd(n: u32) -> D {
    d(reg::TEMP, n)
}
pub fn c(n: u32) -> S {
    s(reg::CONST, n)
}
pub fn v(n: u32) -> S {
    s(reg::INPUT, n)
}
pub fn vd(n: u32) -> D {
    d(reg::INPUT, n)
}
pub fn od(n: u32) -> D {
    d(reg::OUTPUT, n)
}
pub fn sampler(n: u32) -> S {
    s(reg::SAMPLER, n)
}
pub fn i(n: u32) -> S {
    s(reg::CONSTINT, n)
}
pub fn b(n: u32) -> S {
    s(reg::CONSTBOOL, n)
}
pub fn p0() -> S {
    s(reg::PRED, 0)
}
pub fn al() -> S {
    s(reg::LOOP, 0)
}
pub fn l(n: u32) -> S {
    s(reg::LABEL, n)
}
pub fn oc(n: u32) -> D {
    d(reg::COLOROUT, n)
}
pub const A0X: Rel = Rel(reg::ADDR, 0);
pub const AL: Rel = Rel(reg::LOOP, 0);

/// Program builder.
pub struct Asm {
    pub words: Vec<u32>,
}

impl Asm {
    pub fn vs() -> Asm {
        Asm { words: vec![VS30] }
    }
    pub fn ps() -> Asm {
        Asm { words: vec![PS30] }
    }

    /// Append an instruction; the length field is computed.
    pub fn ins(&mut self, opcode: u32, control: u32, dst: Option<D>, src: &[S]) -> &mut Asm {
        self.ins_full(opcode, control, dst, None, src)
    }

    pub fn ins_full(
        &mut self,
        opcode: u32,
        control: u32,
        dst: Option<D>,
        pred: Option<S>,
        src: &[S],
    ) -> &mut Asm {
        let mut body = Vec::new();
        if let Some(d) = dst {
            body.extend(d.tokens());
        }
        if let Some(p) = pred {
            body.extend(p.tokens());
        }
        for s in src {
            body.extend(s.tokens());
        }
        let mut tok = opcode | (control << 16) | (len_u32(body.len()) << 24);
        if pred.is_some() {
            tok |= 1 << 28;
        }
        self.words.push(tok);
        self.words.extend(body);
        self
    }

    pub fn op1(&mut self, opcode: u32, dst: D, a: S) -> &mut Asm {
        self.ins(opcode, 0, Some(dst), &[a])
    }
    pub fn op2(&mut self, opcode: u32, dst: D, a: S, b: S) -> &mut Asm {
        self.ins(opcode, 0, Some(dst), &[a, b])
    }
    pub fn op3(&mut self, opcode: u32, dst: D, a: S, b: S, c: S) -> &mut Asm {
        self.ins(opcode, 0, Some(dst), &[a, b, c])
    }
    pub fn mov(&mut self, dst: D, a: S) -> &mut Asm {
        self.op1(op::MOV, dst, a)
    }
    /// `(pred) opcode dst, src...`
    pub fn pred(&mut self, opcode: u32, pred: S, dst: D, src: &[S]) -> &mut Asm {
        self.ins_full(opcode, 0, Some(dst), Some(pred), src)
    }
    pub fn flow(&mut self, opcode: u32, control: u32, src: &[S]) -> &mut Asm {
        self.ins(opcode, control, None, src)
    }

    pub fn dcl(&mut self, usage: u32, index: u32, dst: D) -> &mut Asm {
        let token = 0x8000_0000 | usage | (index << 16);
        let mut body = vec![token];
        body.extend(dst.tokens());
        self.words.push(op::DCL | (len_u32(body.len()) << 24));
        self.words.extend(body);
        self
    }
    /// `dcl_2d` (2), `dcl_cube` (3), `dcl_volume` (4).
    pub fn dcl_sampler(&mut self, texture_type: u32, n: u32) -> &mut Asm {
        let token = 0x8000_0000 | (texture_type << 27);
        self.words.push(op::DCL | (2 << 24));
        self.words.push(token);
        self.words.push(regbits(reg::SAMPLER, n) | (0xF << 16));
        self
    }
    /// `dcl vPos.xy` (0) or `dcl vFace` (1).
    pub fn dcl_misc(&mut self, n: u32) -> &mut Asm {
        self.words.push(op::DCL | (2 << 24));
        self.words.push(0x8000_0000);
        self.words.push(regbits(reg::MISC, n) | (0xF << 16));
        self
    }
    pub fn def(&mut self, n: u32, v: [f32; 4]) -> &mut Asm {
        self.words.push(op::DEF | (5 << 24));
        self.words.push(regbits(reg::CONST, n) | (0xF << 16));
        self.words.extend(v.map(f32::to_bits));
        self
    }
    pub fn defi(&mut self, n: u32, v: [i32; 4]) -> &mut Asm {
        self.words.push(op::DEFI | (5 << 24));
        self.words.push(regbits(reg::CONSTINT, n) | (0xF << 16));
        self.words.extend(v.map(i32::cast_unsigned));
        self
    }
    pub fn defb(&mut self, n: u32, v: bool) -> &mut Asm {
        self.words.push(op::DEFB | (2 << 24));
        self.words.push(regbits(reg::CONSTBOOL, n) | (0xF << 16));
        self.words.push(u32::from(v));
        self
    }
    pub fn comment(&mut self, payload: &[u32]) -> &mut Asm {
        self.words.push(0xFFFE | (len_u32(payload.len()) << 16));
        self.words.extend_from_slice(payload);
        self
    }
    pub fn raw(&mut self, w: u32) -> &mut Asm {
        self.words.push(w);
        self
    }
    pub fn end(&mut self) -> Vec<u32> {
        let mut w = self.words.clone();
        w.push(END);
        w
    }

    /// A vertex shader prologue: `dcl_position v0`, `dcl_position o0`.
    pub fn vs_basic() -> Asm {
        let mut a = Asm::vs();
        a.dcl(usage::POSITION, 0, vd(0))
            .dcl(usage::POSITION, 0, od(0));
        a
    }
    /// A pixel shader prologue: `dcl_texcoord0 v0`.
    pub fn ps_basic() -> Asm {
        let mut a = Asm::ps();
        a.dcl(usage::TEXCOORD, 0, vd(0));
        a
    }
}

/// Translate with default options, validate, and return the module and report.
pub fn check(words: &[u32]) -> (Module, Report) {
    check_with(words, Options::default())
}

pub fn check_with(words: &[u32], opts: Options) -> (Module, Report) {
    let m = translate_with(words, &opts).unwrap_or_else(|e| panic!("translation failed: {e}"));
    let rep = validate::validate(&m.words)
        .unwrap_or_else(|e| panic!("structural validation failed: {e}\n{}", dump(&m.words)));
    (m, rep)
}

/// Translate and expect an error.
pub fn fail(words: &[u32]) -> Error {
    match translate_with(words, &Options::default()) {
        Ok(_) => panic!("translation unexpectedly succeeded"),
        Err(e) => e,
    }
}

/// Count instructions with an opcode.
pub fn count(m: &Module, opcode: u16) -> usize {
    instructions(&m.words)
        .unwrap()
        .iter()
        .filter(|i| i.opcode == opcode)
        .count()
}

pub fn has(m: &Module, opcode: u16) -> bool {
    count(m, opcode) > 0
}

/// Count `GLSL.std.450` instructions of a number.
pub fn count_ext(m: &Module, inst: u32) -> usize {
    instructions(&m.words)
        .unwrap()
        .iter()
        .filter(|i| i.opcode == lf_dxso_spirv::spirv::op::EXT_INST && i.operands[3] == inst)
        .count()
}

/// The id named `name` by `OpName`.
pub fn named(m: &Module, name: &str) -> Option<u32> {
    instructions(&m.words).unwrap().iter().find_map(|i| {
        if i.opcode == lf_dxso_spirv::spirv::op::NAME {
            let (s, _) = read_string(&i.operands[1..])?;
            (s == name).then_some(i.operands[0])
        } else {
            None
        }
    })
}

/// Decorations `(decoration, literals)` on an id.
pub fn decorations(m: &Module, id: u32) -> Vec<(u32, Vec<u32>)> {
    instructions(&m.words)
        .unwrap()
        .iter()
        .filter(|i| i.opcode == lf_dxso_spirv::spirv::op::DECORATE && i.operands[0] == id)
        .map(|i| (i.operands[1], i.operands[2..].to_vec()))
        .collect()
}

/// The value of decoration `d` on the variable named `name`.
pub fn decoration_of(m: &Module, name: &str, d: u32) -> Option<Vec<u32>> {
    let id = named(m, name)?;
    decorations(m, id)
        .into_iter()
        .find(|(k, _)| *k == d)
        .map(|(_, v)| v)
}

/// Execution modes present.
pub fn execution_modes(m: &Module) -> Vec<u32> {
    instructions(&m.words)
        .unwrap()
        .iter()
        .filter(|i| i.opcode == lf_dxso_spirv::spirv::op::EXECUTION_MODE)
        .map(|i| i.operands[1])
        .collect()
}

/// A crude listing for failure messages.
pub fn dump(words: &[u32]) -> String {
    let mut out = String::new();
    if let Ok(insts) = instructions(words) {
        for i in insts {
            let _ = writeln!(out, "{:5}: op {:3} {:?}", i.offset, i.opcode, i.operands);
        }
    }
    out
}

/// A corpus covering every instruction family, for external validation
/// (`spirv-val`). Each entry: name, tokens, options.
#[allow(clippy::too_many_lines)]
pub fn corpus() -> Vec<(&'static str, Vec<u32>, Options)> {
    let def = Options::default();
    let mut out = vec![
        ("vs_skinned_lit", programs::skinned_lit_vertex_shader(), def),
        ("ps_textured", programs::textured_pixel_shader(), def),
    ];
    let mut a = Asm::vs_basic();
    a.mov(rd(0), v(0)).mov(rd(1), c(1));
    for (o, n) in [
        (op::ADD, 2),
        (op::SUB, 2),
        (op::MUL, 2),
        (op::MAD, 3),
        (op::MIN, 2),
        (op::MAX, 2),
        (op::SLT, 2),
        (op::SGE, 2),
        (op::DP3, 2),
        (op::DP4, 2),
        (op::DST, 2),
        (op::LRP, 3),
        (op::CRS, 2),
        (op::POW, 2),
        (op::M4X4, 2),
        (op::M4X3, 2),
        (op::M3X4, 2),
        (op::M3X3, 2),
        (op::M3X2, 2),
        (op::CMP, 3),
        (op::CND, 3),
        (op::DP2ADD, 3),
    ] {
        let srcs = [r(0), c(4), r(1)];
        let dst = match o {
            op::CRS | op::M4X3 | op::M3X3 => rd(2).m("xyz"),
            op::M3X2 => rd(2).m("xy"),
            _ => rd(2),
        };
        a.ins(o, 0, Some(dst), &srcs[..n]);
    }
    for o in [
        op::RCP,
        op::RSQ,
        op::EXP,
        op::LOG,
        op::EXPP,
        op::LOGP,
        op::FRC,
        op::ABS,
        op::NRM,
        op::LIT,
        op::SGN,
    ] {
        a.op1(o, rd(3), r(2).sw("x"));
    }
    a.op1(op::SINCOS, rd(3).m("xy"), r(2).sw("x"));
    a.mov(rd(3).sat(), r(2).md(sm::ABSNEG));
    a.op1(op::MOVA, d(reg::ADDR, 0), r(3));
    a.mov(rd(4), c(30).rel(Rel(reg::ADDR, 1)));
    a.mov(od(0), r(4));
    out.push(("vs_arithmetic", a.end(), def));

    let mut a = Asm::ps_basic();
    a.dcl(usage::COLOR, 0, vd(1).centroid());
    a.dcl_sampler(2, 0)
        .dcl_sampler(3, 1)
        .dcl_sampler(4, 2)
        .dcl_misc(0)
        .dcl_misc(1);
    a.defi(0, [3, 1, 2, 0])
        .defb(0, true)
        .def(0, [0.0, 0.5, 1.0, -1.0]);
    a.op2(op::TEX, rd(0), v(0), sampler(0));
    a.ins(op::TEX, 1, Some(rd(1)), &[v(0), sampler(1)]);
    a.ins(op::TEX, 2, Some(rd(1)), &[v(0), sampler(2)]);
    a.op2(op::TEXLDL, rd(1), v(0), sampler(0));
    a.ins(op::TEXLDD, 0, Some(rd(1)), &[v(0), sampler(1), c(1), c(2)]);
    a.op1(op::DSX, rd(2), s(reg::MISC, 0));
    a.op1(op::DSY, rd(2), v(1));
    a.ins(op::TEXKILL, 0, Some(rd(0)), &[]);
    a.ins(op::SETP, cmp::NE, Some(d(reg::PRED, 0)), &[r(0), c(0)]);
    a.pred(op::MOV, p0().sw("z").not(), rd(2), &[r(1)]);
    a.flow(op::IF, 0, &[b(0)]);
    a.flow(op::REP, 0, &[i(0)]);
    a.flow(op::BREAKP, 0, &[p0().sw("x")]);
    a.flow(op::LOOP, 0, &[al(), i(1)]);
    a.op2(op::ADD, rd(2), r(2), v(0).rel(AL));
    a.flow(op::BREAKC, cmp::GE, &[r(2).sw("x"), c(0).sw("z")]);
    a.flow(op::ENDLOOP, 0, &[]);
    a.flow(op::ENDREP, 0, &[]);
    a.flow(op::ELSE, 0, &[]);
    a.flow(op::IFC, cmp::LT, &[s(reg::MISC, 1), c(0).sw("x")]);
    a.op3(op::LRP, rd(2), r(0), r(1), c(3));
    a.flow(op::ENDIF, 0, &[]);
    a.flow(op::REP, 0, &[i(0)]);
    a.flow(op::IF, 0, &[b(2)]);
    a.flow(op::BREAK, 0, &[]);
    a.flow(op::ENDIF, 0, &[]);
    a.flow(op::ENDREP, 0, &[]);
    a.flow(op::ENDIF, 0, &[]);
    a.flow(op::CALLNZ, 0, &[l(3), b(1).not()]);
    a.mov(oc(0), r(2))
        .mov(oc(1), r(0))
        .mov(d(reg::DEPTHOUT, 0), r(1).sw("w"));
    a.flow(op::RET, 0, &[]);
    a.flow(op::LABEL, 0, &[l(3)]);
    a.op2(op::MUL, rd(2), r(2), c(5));
    a.flow(op::RET, 0, &[]);
    out.push(("ps_flow_texture", a.end(), def));

    let mut a = Asm::ps_basic();
    a.dcl_sampler(2, 0);
    a.op2(op::TEX, rd(0), v(0), sampler(0));
    a.ins(op::TEX, 1, Some(rd(0)), &[v(0), sampler(0)]);
    a.mov(oc(0), r(0));
    out.push((
        "ps_shadow",
        a.end(),
        Options {
            depth_compare_samplers: 1,
            ..def
        },
    ));

    let mut a = Asm::vs_basic();
    a.dcl(usage::TEXCOORD, 0, od(1).m("xy"))
        .dcl(usage::TEXCOORD, 1, od(1).m("zw"))
        .dcl(usage::PSIZE, 0, od(2).m("x"))
        .dcl(usage::TEXCOORD, 2, od(3))
        .dcl_sampler(2, 0);
    a.defi(0, [2, 0, 2, 0]);
    a.op2(op::TEXLDL, rd(0), v(0), sampler(0));
    a.mov(od(0), v(0)).mov(od(1), r(0)).mov(od(2).m("x"), c(0));
    a.flow(op::LOOP, 0, &[al(), i(0)]);
    a.mov(od(1).rel(AL), c(1));
    a.flow(op::ENDLOOP, 0, &[]);
    out.push(("vs_outputs", a.end(), def));
    out
}
