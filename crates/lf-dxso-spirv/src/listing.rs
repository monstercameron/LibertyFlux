//! A human-readable listing of decoded shader model 3 programs, for
//! debugging translations.
//!
//! **A listing of the game's own shaders is a disassembly of game files.**
//! It is a debugging aid for the person who owns the game: keep its output
//! on your own machine and never commit it, paste it into the repository,
//! the devlog or an issue, or share it (`AGENTS.md`, rule 1: disassembly of
//! the original game stays under the ignored `.artifacts/` folder). The
//! hand-built programs of this crate's tests are the only listings that may
//! appear in tracked files.
//!
//! The listing is built on [`mod@crate::decode`]: it shows exactly what the
//! decoder read, one instruction per line, so an [`crate::Error`] offset
//! can be matched to the instruction it names. The syntax follows
//! Microsoft's Direct3D 9 shader assembler as documented publicly
//! (Inferred: not cross-checked against Microsoft's tools here), with a
//! few deliberate differences so that nothing in the token is hidden:
//!
//! | Element | Printed as |
//! |---|---|
//! | header | `vs_3_0` / `ps_3_0`, then a `;` comment with the instruction count, the program length in dwords and whether a `CTAB` comment was present |
//! | line | dword offset of the instruction token (the offset [`crate::Error`] reports), then the instruction, indented inside `if`/`loop`/`rep` blocks |
//! | registers | `r#`, `v#`, `c#`, `a0`, `i#`, `b#`, `s#`, `o#`, `oC#`, `oDepth`, `aL`, `p0`, `l#`, `vPos`, `vFace`; files shader model 3 lacks get their model 1-2 names (`t#`, `oPos`, `oFog`, `oPts`, `oD#`) and the extra constant banks print their absolute numbers (`c2048` and up) |
//! | write mask | `.xyzw` subset, omitted when all four components are written, `.none` when the mask is zero |
//! | swizzle | omitted when it is the identity, one letter when all four selectors are equal, otherwise all four letters (never the assembler's shortened forms, which hide the replication) |
//! | relative address | `c100[a0.x]`, `c20[aL]`, `v0[aL]`, `o1[aL]` |
//! | source modifiers | `-r0`, `r0_abs`, `-r0_abs`, `r0_bias`, `r0_bx2`, `r0_x2` (each with a `-` form), `1 - r0`, `r0_dz`, `r0_dw`, `!b0`; the swizzle follows the modifier suffix |
//! | result modifiers | appended to the mnemonic: `_sat`, `_pp`, `_centroid`, then the model 1 shift (`_x2`, `_x4`, `_x8`, `_d2`, `_d4`, `_d8`) |
//! | comparisons | `if_gt`, `break_le`, `setp_ne`, ...; codes 0 and 7 print as `_cmp0`, `_cmp7` |
//! | texture flags | `texldp`, `texldb` (`texldpb` when both are set) |
//! | `dcl` | `dcl_texcoord1 v1.xy` (the usage index is always printed, `dcl_position0` included), `dcl_2d s0`, `dcl_cube s1`, `dcl_volume s2`, `dcl vPos.xy`, `dcl vFace`; an unknown usage prints as `dcl_usage14_0`, an unknown texture type as `dcl_textype5` |
//! | `def` | `def c0, 0.5, 1.0, -0.0, inf` in Rust's shortest round-trip notation; a NaN prints with its bits, `nan(0x7fc00000)` |
//! | `defi`, `defb` | `defi i0, 4, 0, 1, 0`, `defb b0, true` |
//! | predicate | `(p0.x) mov r0, r1`, `(!p0.z) ...` |
//! | control bits | control bits the opcode does not use are kept as a trailing comment, `; control 0x04` |
//!
//! Comments other than the `CTAB` marker are not kept by the decoder, so
//! they do not appear.

use std::fmt::Write as _;

use crate::binding;
use crate::decode::{
    Dcl, DstParam, IDENTITY_SWIZZLE, Instruction, Opcode, Payload, RegType, Register, RelAddr,
    Shader, SrcMod, SrcParam, Stage, TEXLD_BIAS, TEXLD_PROJECT, TextureType,
};

/// Comparison codes of `ifc`, `breakc` and `setp` live in control bits 2:0.
const COMPARISON_MASK: u8 = 0x7;
/// Number of the first register of the `Const2` bank (`c2048`).
const CONST2_BASE: u32 = 2048;
/// Number of the first register of the `Const3` bank (`c4096`).
const CONST3_BASE: u32 = 4096;
/// Number of the first register of the `Const4` bank (`c6144`).
const CONST4_BASE: u32 = 6144;
/// `vPos` is misc register 0.
const MISC_POSITION: u16 = 0;
/// `vFace` is misc register 1.
const MISC_FACE: u16 = 1;
/// Spaces per nesting level of flow control.
const INDENT: &str = "  ";
/// Component letters, selector 0 to 3.
const COMPONENTS: [char; 4] = ['x', 'y', 'z', 'w'];

/// How [`listing_with`] lays out a program.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ListingOptions {
    /// Start every line with the dword offset of the instruction token.
    /// Turn it off to compare two programs with a line diff.
    pub offsets: bool,
    /// Indent the bodies of `if`, `loop` and `rep` blocks.
    pub indent: bool,
}

impl Default for ListingOptions {
    fn default() -> Self {
        ListingOptions {
            offsets: true,
            indent: true,
        }
    }
}

/// The listing of a whole program with default [`ListingOptions`]: a
/// header line, a comment line, then one line per instruction.
#[must_use]
pub fn listing(shader: &Shader) -> String {
    listing_with(shader, &ListingOptions::default())
}

/// The listing of a whole program; see the [module documentation](self)
/// for the syntax.
#[must_use]
pub fn listing_with(shader: &Shader, options: &ListingOptions) -> String {
    let mut out = String::new();
    let prefix = match shader.stage {
        Stage::Vertex => "vs",
        Stage::Pixel => "ps",
    };
    let _ = writeln!(out, "{prefix}_{}_{}", shader.major, shader.minor);
    let _ = writeln!(
        out,
        "; {} instructions in {} dwords; CTAB comment {}",
        shader.instructions.len(),
        shader.end_offset,
        if shader.has_ctab { "present" } else { "absent" }
    );
    let mut depth = 0usize;
    for ins in &shader.instructions {
        match ins.opcode {
            Opcode::Else | Opcode::EndIf | Opcode::EndLoop | Opcode::EndRep => {
                depth = depth.saturating_sub(1);
            }
            // A subroutine always starts outside flow control.
            Opcode::Label => depth = 0,
            _ => {}
        }
        if options.offsets {
            let _ = write!(out, "{:>5}  ", ins.offset);
        }
        if options.indent {
            out.push_str(&INDENT.repeat(depth));
        }
        out.push_str(&instruction_text(ins, shader.stage));
        out.push('\n');
        if matches!(
            ins.opcode,
            Opcode::If | Opcode::Ifc | Opcode::Else | Opcode::Loop | Opcode::Rep
        ) {
            depth += 1;
        }
    }
    out
}

/// One instruction as listing text, without offset or indentation.
#[must_use]
pub fn instruction_text(ins: &Instruction, stage: Stage) -> String {
    let mut out = String::new();
    if let Some(p) = &ins.predicate {
        let _ = write!(out, "({}) ", src_text(p, stage));
    }
    out.push_str(&mnemonic(ins));
    if let Some(d) = &ins.dst {
        out.push_str(&result_modifiers(d));
    }
    let mut operands: Vec<String> = Vec::new();
    match ins.payload {
        Payload::Dcl(dcl) => operands.push(dst_text(&dcl.dst, stage)),
        Payload::DefF(bits) => {
            operands.extend(ins.dst.iter().map(|d| dst_text(d, stage)));
            operands.extend(bits.iter().map(|&b| float_text(b)));
        }
        Payload::DefI(values) => {
            operands.extend(ins.dst.iter().map(|d| dst_text(d, stage)));
            operands.extend(values.iter().map(i32::to_string));
        }
        Payload::DefB(value) => {
            operands.extend(ins.dst.iter().map(|d| dst_text(d, stage)));
            operands.push(value.to_string());
        }
        Payload::None => {
            operands.extend(ins.dst.iter().map(|d| dst_text(d, stage)));
            operands.extend(ins.src.iter().map(|s| src_text(s, stage)));
        }
    }
    if !operands.is_empty() {
        out.push(' ');
        out.push_str(&operands.join(", "));
    }
    if unused_control_bits(ins) != 0 {
        let _ = write!(out, " ; control 0x{:02x}", ins.control);
    }
    out
}

/// The mnemonic of an instruction with its control-derived suffix
/// (`if_lt`, `setp_ge`, `texldp`) and, for `dcl`, its usage or texture
/// type (`dcl_texcoord1`, `dcl_2d`); result modifiers are not included.
#[must_use]
pub fn mnemonic(ins: &Instruction) -> String {
    match ins.opcode {
        Opcode::Ifc => format!("if{}", comparison_suffix(ins.control)),
        Opcode::BreakC => format!("break{}", comparison_suffix(ins.control)),
        Opcode::Setp => format!("setp{}", comparison_suffix(ins.control)),
        Opcode::Tex => {
            let mut m = String::from("texld");
            if ins.control & TEXLD_PROJECT != 0 {
                m.push('p');
            }
            if ins.control & TEXLD_BIAS != 0 {
                m.push('b');
            }
            m
        }
        Opcode::Dcl => match ins.payload {
            Payload::Dcl(dcl) => dcl_mnemonic(&dcl),
            _ => "dcl".to_string(),
        },
        other => other.name().to_string(),
    }
}

/// The name of a register as the listing prints it, for example `r3`,
/// `oC0`, `vFace` or `c2050` (register 2 of the `Const2` bank).
#[must_use]
pub fn register_name(reg: Register, stage: Stage) -> String {
    let n = reg.num;
    match reg.kind {
        RegType::Temp => format!("r{n}"),
        RegType::Input => format!("v{n}"),
        RegType::Const => format!("c{n}"),
        RegType::Addr => match stage {
            Stage::Vertex => format!("a{n}"),
            Stage::Pixel => format!("t{n}"),
        },
        RegType::RastOut => match n {
            0 => "oPos".to_string(),
            1 => "oFog".to_string(),
            2 => "oPts".to_string(),
            _ => format!("oRast{n}"),
        },
        RegType::AttrOut => format!("oD{n}"),
        RegType::Output => format!("o{n}"),
        RegType::ConstInt => format!("i{n}"),
        RegType::ColorOut => format!("oC{n}"),
        RegType::DepthOut if n == 0 => "oDepth".to_string(),
        RegType::DepthOut => format!("oDepth{n}"),
        RegType::Sampler => format!("s{n}"),
        RegType::Const2 => format!("c{}", CONST2_BASE + u32::from(n)),
        RegType::Const3 => format!("c{}", CONST3_BASE + u32::from(n)),
        RegType::Const4 => format!("c{}", CONST4_BASE + u32::from(n)),
        RegType::ConstBool => format!("b{n}"),
        RegType::Loop if n == 0 => "aL".to_string(),
        RegType::Loop => format!("aL{n}"),
        RegType::TempFloat16 => format!("half{n}"),
        RegType::MiscType => match n {
            MISC_POSITION => "vPos".to_string(),
            MISC_FACE => "vFace".to_string(),
            _ => format!("vMisc{n}"),
        },
        RegType::Label => format!("l{n}"),
        RegType::Predicate => format!("p{n}"),
    }
}

/// A short name for a whole register file in one stage, for tables:
/// `r#`, `c#`, `a0` (vertex) or `t#` (pixel), `vPos/vFace`, ...
#[must_use]
pub fn register_file_name(kind: RegType, stage: Stage) -> &'static str {
    match kind {
        RegType::Temp => "r#",
        RegType::Input => "v#",
        RegType::Const => "c#",
        RegType::Addr => match stage {
            Stage::Vertex => "a0",
            Stage::Pixel => "t#",
        },
        RegType::RastOut => "oPos/oFog/oPts",
        RegType::AttrOut => "oD#",
        RegType::Output => "o#",
        RegType::ConstInt => "i#",
        RegType::ColorOut => "oC#",
        RegType::DepthOut => "oDepth",
        RegType::Sampler => "s#",
        RegType::Const2 => "c# (2048-4095)",
        RegType::Const3 => "c# (4096-6143)",
        RegType::Const4 => "c# (6144-8191)",
        RegType::ConstBool => "b#",
        RegType::Loop => "aL",
        RegType::TempFloat16 => "half#",
        RegType::MiscType => "vPos/vFace",
        RegType::Label => "l#",
        RegType::Predicate => "p0",
    }
}

/// A source operand: modifier, register, relative address, swizzle.
fn src_text(s: &SrcParam, stage: Stage) -> String {
    let (prefix, suffix) = match s.modifier {
        SrcMod::None => ("", ""),
        SrcMod::Neg => ("-", ""),
        SrcMod::Bias => ("", "_bias"),
        SrcMod::BiasNeg => ("-", "_bias"),
        SrcMod::Sign => ("", "_bx2"),
        SrcMod::SignNeg => ("-", "_bx2"),
        SrcMod::Comp => ("1 - ", ""),
        SrcMod::X2 => ("", "_x2"),
        SrcMod::X2Neg => ("-", "_x2"),
        SrcMod::Dz => ("", "_dz"),
        SrcMod::Dw => ("", "_dw"),
        SrcMod::Abs => ("", "_abs"),
        SrcMod::AbsNeg => ("-", "_abs"),
        SrcMod::Not => ("!", ""),
    };
    format!(
        "{prefix}{}{suffix}{}",
        addressed(s.reg, s.relative, stage),
        swizzle_text(s.swizzle)
    )
}

/// A destination operand: register, relative address, write mask.
fn dst_text(d: &DstParam, stage: Stage) -> String {
    format!(
        "{}{}",
        addressed(d.reg, d.relative, stage),
        mask_text(d.write_mask)
    )
}

/// `c100` or `c100[a0.x]`.
fn addressed(reg: Register, relative: Option<RelAddr>, stage: Stage) -> String {
    let name = register_name(reg, stage);
    match relative {
        None => name,
        Some(rel) => {
            let base = register_name(rel.reg, stage);
            if rel.reg.kind == RegType::Loop {
                format!("{name}[{base}]")
            } else {
                let c = COMPONENTS[usize::from(rel.component & 3)];
                format!("{name}[{base}.{c}]")
            }
        }
    }
}

/// `.xz`, nothing for a full mask, `.none` for an empty one.
fn mask_text(mask: u8) -> String {
    match mask & 0xF {
        0xF => String::new(),
        0 => ".none".to_string(),
        m => {
            let mut out = String::from(".");
            out.extend((0..4).filter(|i| m & (1 << i) != 0).map(|i| COMPONENTS[i]));
            out
        }
    }
}

/// Nothing for the identity, one letter for a replicate, else four.
fn swizzle_text(sw: [u8; 4]) -> String {
    if sw == IDENTITY_SWIZZLE {
        return String::new();
    }
    let letter = |s: u8| COMPONENTS[usize::from(s & 3)];
    if sw.iter().all(|&s| s == sw[0]) {
        return format!(".{}", letter(sw[0]));
    }
    let mut out = String::from(".");
    out.extend(sw.iter().map(|&s| letter(s)));
    out
}

/// `_sat`, `_pp`, `_centroid` and the shift suffix, in that order.
fn result_modifiers(d: &DstParam) -> String {
    let mut out = String::new();
    if d.saturate {
        out.push_str("_sat");
    }
    if d.partial_precision {
        out.push_str("_pp");
    }
    if d.centroid {
        out.push_str("_centroid");
    }
    match d.shift {
        0 => {}
        1 => out.push_str("_x2"),
        2 => out.push_str("_x4"),
        3 => out.push_str("_x8"),
        -1 => out.push_str("_d2"),
        -2 => out.push_str("_d4"),
        -3 => out.push_str("_d8"),
        n => {
            let _ = write!(out, "_shift{n}");
        }
    }
    out
}

/// `_gt`, `_eq`, ... for comparison codes 1-6; `_cmpN` otherwise.
fn comparison_suffix(control: u8) -> String {
    match control & COMPARISON_MASK {
        1 => "_gt".to_string(),
        2 => "_eq".to_string(),
        3 => "_ge".to_string(),
        4 => "_lt".to_string(),
        5 => "_ne".to_string(),
        6 => "_le".to_string(),
        n => format!("_cmp{n}"),
    }
}

/// Control bits the listing does not already show in the mnemonic.
fn unused_control_bits(ins: &Instruction) -> u8 {
    match ins.opcode {
        Opcode::Ifc | Opcode::BreakC | Opcode::Setp => ins.control & !COMPARISON_MASK,
        Opcode::Tex => ins.control & !(TEXLD_PROJECT | TEXLD_BIAS),
        _ => ins.control,
    }
}

/// `dcl_texcoord1`, `dcl_2d`, `dcl` for `vPos`/`vFace`.
fn dcl_mnemonic(dcl: &Dcl) -> String {
    match dcl.dst.reg.kind {
        RegType::Sampler => match dcl.texture_type {
            TextureType::Tex2d => "dcl_2d".to_string(),
            TextureType::Cube => "dcl_cube".to_string(),
            TextureType::Volume => "dcl_volume".to_string(),
            TextureType::Unknown(t) => format!("dcl_textype{t}"),
        },
        RegType::MiscType => "dcl".to_string(),
        RegType::Input | RegType::Output => usage_mnemonic(dcl.usage, dcl.usage_index),
        // Other files carry no usage in a valid program; show one if set.
        _ if dcl.usage != 0 || dcl.usage_index != 0 => usage_mnemonic(dcl.usage, dcl.usage_index),
        _ => "dcl".to_string(),
    }
}

/// `dcl_texcoord1`; an unknown usage number prints as `dcl_usage14_1`.
fn usage_mnemonic(usage: u8, index: u8) -> String {
    match binding::usage_name(usage) {
        "unknown" => format!("dcl_usage{usage}_{index}"),
        name => format!("dcl_{name}{index}"),
    }
}

/// A `def` component: shortest round-trip decimal, or NaN with its bits.
fn float_text(bits: u32) -> String {
    let v = f32::from_bits(bits);
    if v.is_nan() {
        format!("nan(0x{bits:08x})")
    } else {
        format!("{v:?}")
    }
}
