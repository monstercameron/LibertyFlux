//! Decoder for the Direct3D 9 shader token stream ("DXSO").
//!
//! Turns a `vs_3_0` / `ps_3_0` token stream into a list of typed
//! [`Instruction`]s. The decoder checks the token grammar only (lengths,
//! parameter shapes, operand counts); register ranges and the meaning of
//! each instruction are checked by the translator. It never panics on
//! malformed input: every failure is an [`Error`].
//!
//! Token layout (public Microsoft documentation, `d3d9types.h`; written
//! from that documentation, not cross-checked against a copy in this
//! environment, so the layout is labelled Inferred and is pinned by the
//! hand-built tests):
//!
//! | Token | Bits | Meaning |
//! |---|---|---|
//! | version | 31:16 | `0xFFFE` vertex, `0xFFFF` pixel |
//! | version | 15:8, 7:0 | major, minor |
//! | instruction | 15:0 | opcode |
//! | instruction | 23:16 | opcode-specific control (comparison, texld flags) |
//! | instruction | 27:24 | parameter dwords that follow |
//! | instruction | 28 | predicated: a predicate token follows the destination |
//! | instruction | 30 | co-issue (pixel shader 1.x only; rejected here) |
//! | comment | 15:0 = `0xFFFE`, 30:16 | comment length in dwords |
//! | end | all | `0x0000FFFF` |
//! | parameter | 31 | always 1 |
//! | parameter | 10:0 | register number |
//! | parameter | 30:28 and 12:11 | register type, low three bits and high two bits |
//! | parameter | 13 | relative addressing: an address token follows |
//! | destination | 19:16 | write mask (x, y, z, w) |
//! | destination | 23:20 | result modifier: saturate, partial precision, centroid |
//! | destination | 27:24 | shift scale (signed, pixel shader 1.x only) |
//! | source | 23:16 | swizzle, two bits per component, x first |
//! | source | 27:24 | source modifier |
//! | `dcl` token | 3:0, 19:16 | usage and usage index (inputs and outputs) |
//! | `dcl` token | 30:27 | texture type (samplers) |

use crate::Error;

/// The end-of-shader token.
pub const END_TOKEN: u32 = 0x0000_FFFF;
/// Low word of a comment token.
pub const COMMENT_OPCODE: u32 = 0xFFFE;
/// Version token high word for vertex shaders.
pub const VERTEX_VERSION_PREFIX: u32 = 0xFFFE_0000;
/// Version token high word for pixel shaders.
pub const PIXEL_VERSION_PREFIX: u32 = 0xFFFF_0000;

const INSTR_LENGTH_SHIFT: u32 = 24;
const INSTR_LENGTH_MASK: u32 = 0xF;
const INSTR_CONTROL_SHIFT: u32 = 16;
const INSTR_PREDICATED: u32 = 1 << 28;
const INSTR_COISSUE: u32 = 1 << 30;
const COMMENT_LENGTH_SHIFT: u32 = 16;
const COMMENT_LENGTH_MASK: u32 = 0x7FFF;
const PARAM_MARKER: u32 = 1 << 31;
const REGNUM_MASK: u32 = 0x7FF;
const REGTYPE_LOW_SHIFT: u32 = 28;
const REGTYPE_LOW_MASK: u32 = 0x7;
const REGTYPE_HIGH_SHIFT: u32 = 11;
const REGTYPE_HIGH_MASK: u32 = 0x3;
const RELATIVE_BIT: u32 = 1 << 13;
const WRITEMASK_SHIFT: u32 = 16;
const RESULT_MOD_SHIFT: u32 = 20;
const RESULT_SATURATE: u32 = 1;
const RESULT_PARTIAL_PRECISION: u32 = 2;
const RESULT_CENTROID: u32 = 4;
const DST_SHIFT_SHIFT: u32 = 24;
const SWIZZLE_SHIFT: u32 = 16;
const SRC_MOD_SHIFT: u32 = 24;
const DCL_USAGE_MASK: u32 = 0xF;
const DCL_USAGE_INDEX_SHIFT: u32 = 16;
const DCL_USAGE_INDEX_MASK: u32 = 0xF;
const DCL_TEXTURE_TYPE_SHIFT: u32 = 27;
const DCL_TEXTURE_TYPE_MASK: u32 = 0xF;
/// `texld` control flag: projected (`texldp`).
pub const TEXLD_PROJECT: u8 = 0x01;
/// `texld` control flag: bias from `.w` (`texldb`).
pub const TEXLD_BIAS: u8 = 0x02;

/// The identity swizzle `.xyzw`.
pub const IDENTITY_SWIZZLE: [u8; 4] = [0, 1, 2, 3];

/// Shader stage, from the version token.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Stage {
    /// `vs_*`.
    Vertex,
    /// `ps_*`.
    Pixel,
}

/// A decoded shader program.
#[derive(Debug, Clone, PartialEq)]
pub struct Shader {
    /// Vertex or pixel.
    pub stage: Stage,
    /// Shader model major version.
    pub major: u8,
    /// Shader model minor version.
    pub minor: u8,
    /// Instructions in program order (comments removed).
    pub instructions: Vec<Instruction>,
    /// Whether a comment holding a `CTAB` constant table was seen.
    pub has_ctab: bool,
    /// Dword index just past the end token.
    pub end_offset: usize,
}

/// Every Direct3D 9 opcode, including the pixel shader 1.x ones that the
/// translator rejects, so that rejections can name the instruction.
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Opcode {
    Nop,
    Mov,
    Add,
    Sub,
    Mad,
    Mul,
    Rcp,
    Rsq,
    Dp3,
    Dp4,
    Min,
    Max,
    Slt,
    Sge,
    Exp,
    Log,
    Lit,
    Dst,
    Lrp,
    Frc,
    M4x4,
    M4x3,
    M3x4,
    M3x3,
    M3x2,
    Call,
    CallNz,
    Loop,
    Ret,
    EndLoop,
    Label,
    Dcl,
    Pow,
    Crs,
    Sgn,
    Abs,
    Nrm,
    SinCos,
    Rep,
    EndRep,
    If,
    Ifc,
    Else,
    EndIf,
    Break,
    BreakC,
    Mova,
    DefB,
    DefI,
    TexCoord,
    TexKill,
    /// `texld` in shader model 2 and later (`tex` in 1.x).
    Tex,
    TexBem,
    TexBemL,
    TexReg2Ar,
    TexReg2Gb,
    TexM3x2Pad,
    TexM3x2Tex,
    TexM3x3Pad,
    TexM3x3Tex,
    TexM3x3Spec,
    TexM3x3VSpec,
    ExpP,
    LogP,
    Cnd,
    Def,
    TexReg2Rgb,
    TexDp3Tex,
    TexM3x2Depth,
    TexDp3,
    TexM3x3,
    TexDepth,
    Cmp,
    Bem,
    Dp2Add,
    Dsx,
    Dsy,
    TexLdd,
    Setp,
    TexLdl,
    BreakP,
    Phase,
}

impl Opcode {
    /// Map a raw opcode number to an [`Opcode`].
    #[must_use]
    pub fn from_raw(raw: u16) -> Option<Opcode> {
        use Opcode as O;
        Some(match raw {
            0 => O::Nop,
            1 => O::Mov,
            2 => O::Add,
            3 => O::Sub,
            4 => O::Mad,
            5 => O::Mul,
            6 => O::Rcp,
            7 => O::Rsq,
            8 => O::Dp3,
            9 => O::Dp4,
            10 => O::Min,
            11 => O::Max,
            12 => O::Slt,
            13 => O::Sge,
            14 => O::Exp,
            15 => O::Log,
            16 => O::Lit,
            17 => O::Dst,
            18 => O::Lrp,
            19 => O::Frc,
            20 => O::M4x4,
            21 => O::M4x3,
            22 => O::M3x4,
            23 => O::M3x3,
            24 => O::M3x2,
            25 => O::Call,
            26 => O::CallNz,
            27 => O::Loop,
            28 => O::Ret,
            29 => O::EndLoop,
            30 => O::Label,
            31 => O::Dcl,
            32 => O::Pow,
            33 => O::Crs,
            34 => O::Sgn,
            35 => O::Abs,
            36 => O::Nrm,
            37 => O::SinCos,
            38 => O::Rep,
            39 => O::EndRep,
            40 => O::If,
            41 => O::Ifc,
            42 => O::Else,
            43 => O::EndIf,
            44 => O::Break,
            45 => O::BreakC,
            46 => O::Mova,
            47 => O::DefB,
            48 => O::DefI,
            64 => O::TexCoord,
            65 => O::TexKill,
            66 => O::Tex,
            67 => O::TexBem,
            68 => O::TexBemL,
            69 => O::TexReg2Ar,
            70 => O::TexReg2Gb,
            71 => O::TexM3x2Pad,
            72 => O::TexM3x2Tex,
            73 => O::TexM3x3Pad,
            74 => O::TexM3x3Tex,
            76 => O::TexM3x3Spec,
            77 => O::TexM3x3VSpec,
            78 => O::ExpP,
            79 => O::LogP,
            80 => O::Cnd,
            81 => O::Def,
            82 => O::TexReg2Rgb,
            83 => O::TexDp3Tex,
            84 => O::TexM3x2Depth,
            85 => O::TexDp3,
            86 => O::TexM3x3,
            87 => O::TexDepth,
            88 => O::Cmp,
            89 => O::Bem,
            90 => O::Dp2Add,
            91 => O::Dsx,
            92 => O::Dsy,
            93 => O::TexLdd,
            94 => O::Setp,
            95 => O::TexLdl,
            96 => O::BreakP,
            0xFFFD => O::Phase,
            _ => return None,
        })
    }

    /// Assembler mnemonic, for error messages.
    #[must_use]
    pub fn name(self) -> &'static str {
        use Opcode as O;
        match self {
            O::Nop => "nop",
            O::Mov => "mov",
            O::Add => "add",
            O::Sub => "sub",
            O::Mad => "mad",
            O::Mul => "mul",
            O::Rcp => "rcp",
            O::Rsq => "rsq",
            O::Dp3 => "dp3",
            O::Dp4 => "dp4",
            O::Min => "min",
            O::Max => "max",
            O::Slt => "slt",
            O::Sge => "sge",
            O::Exp => "exp",
            O::Log => "log",
            O::Lit => "lit",
            O::Dst => "dst",
            O::Lrp => "lrp",
            O::Frc => "frc",
            O::M4x4 => "m4x4",
            O::M4x3 => "m4x3",
            O::M3x4 => "m3x4",
            O::M3x3 => "m3x3",
            O::M3x2 => "m3x2",
            O::Call => "call",
            O::CallNz => "callnz",
            O::Loop => "loop",
            O::Ret => "ret",
            O::EndLoop => "endloop",
            O::Label => "label",
            O::Dcl => "dcl",
            O::Pow => "pow",
            O::Crs => "crs",
            O::Sgn => "sgn",
            O::Abs => "abs",
            O::Nrm => "nrm",
            O::SinCos => "sincos",
            O::Rep => "rep",
            O::EndRep => "endrep",
            O::If => "if",
            O::Ifc => "ifc",
            O::Else => "else",
            O::EndIf => "endif",
            O::Break => "break",
            O::BreakC => "breakc",
            O::Mova => "mova",
            O::DefB => "defb",
            O::DefI => "defi",
            O::TexCoord => "texcoord",
            O::TexKill => "texkill",
            O::Tex => "texld",
            O::TexBem => "texbem",
            O::TexBemL => "texbeml",
            O::TexReg2Ar => "texreg2ar",
            O::TexReg2Gb => "texreg2gb",
            O::TexM3x2Pad => "texm3x2pad",
            O::TexM3x2Tex => "texm3x2tex",
            O::TexM3x3Pad => "texm3x3pad",
            O::TexM3x3Tex => "texm3x3tex",
            O::TexM3x3Spec => "texm3x3spec",
            O::TexM3x3VSpec => "texm3x3vspec",
            O::ExpP => "expp",
            O::LogP => "logp",
            O::Cnd => "cnd",
            O::Def => "def",
            O::TexReg2Rgb => "texreg2rgb",
            O::TexDp3Tex => "texdp3tex",
            O::TexM3x2Depth => "texm3x2depth",
            O::TexDp3 => "texdp3",
            O::TexM3x3 => "texm3x3",
            O::TexDepth => "texdepth",
            O::Cmp => "cmp",
            O::Bem => "bem",
            O::Dp2Add => "dp2add",
            O::Dsx => "dsx",
            O::Dsy => "dsy",
            O::TexLdd => "texldd",
            O::Setp => "setp",
            O::TexLdl => "texldl",
            O::BreakP => "breakp",
            O::Phase => "phase",
        }
    }

    /// Whether the instruction has a destination parameter, and the
    /// allowed number of source parameters (excluding the predicate).
    /// `None` for the pixel shader 1.x instructions, whose operand shape
    /// differs by version and is not decoded.
    fn operand_shape(self) -> Option<(bool, u8, u8)> {
        use Opcode as O;
        Some(match self {
            O::Nop | O::Ret | O::EndLoop | O::EndRep | O::Else | O::EndIf | O::Break => {
                (false, 0, 0)
            }
            O::Mov
            | O::Rcp
            | O::Rsq
            | O::Exp
            | O::Log
            | O::Lit
            | O::Frc
            | O::Abs
            | O::Nrm
            | O::Mova
            | O::ExpP
            | O::LogP
            | O::Dsx
            | O::Dsy => (true, 1, 1),
            O::Add
            | O::Sub
            | O::Mul
            | O::Dp3
            | O::Dp4
            | O::Min
            | O::Max
            | O::Slt
            | O::Sge
            | O::Dst
            | O::M4x4
            | O::M4x3
            | O::M3x4
            | O::M3x3
            | O::M3x2
            | O::Pow
            | O::Crs
            | O::Tex
            | O::Setp
            | O::TexLdl => (true, 2, 2),
            O::Mad | O::Lrp | O::Cnd | O::Cmp | O::Dp2Add => (true, 3, 3),
            // Shader model 2 forms carry two scratch operands; model 3 has one source.
            O::SinCos | O::Sgn => (true, 1, 3),
            O::TexLdd => (true, 4, 4),
            O::TexKill => (true, 0, 0),
            O::Call | O::Label | O::Rep | O::If | O::BreakP => (false, 1, 1),
            O::CallNz | O::Loop | O::Ifc | O::BreakC => (false, 2, 2),
            O::Dcl
            | O::Def
            | O::DefI
            | O::DefB
            | O::TexCoord
            | O::TexBem
            | O::TexBemL
            | O::TexReg2Ar
            | O::TexReg2Gb
            | O::TexM3x2Pad
            | O::TexM3x2Tex
            | O::TexM3x3Pad
            | O::TexM3x3Tex
            | O::TexM3x3Spec
            | O::TexM3x3VSpec
            | O::TexReg2Rgb
            | O::TexDp3Tex
            | O::TexM3x2Depth
            | O::TexDp3
            | O::TexM3x3
            | O::TexDepth
            | O::Bem
            | O::Phase => return None,
        })
    }
}

/// Register files (`D3DSPR_*`). Some numbers are shared between the two
/// stages under different names; the stage-specific name is noted.
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum RegType {
    /// `r#`
    Temp,
    /// `v#`
    Input,
    /// `c#`
    Const,
    /// `a0` in vertex shaders, `t#` in pixel shaders 1.x/2.x.
    Addr,
    /// `oPos`, `oFog`, `oPts` (vertex shaders before 3.0).
    RastOut,
    /// `oD#` (vertex shaders before 3.0).
    AttrOut,
    /// `o#` in `vs_3_0`, `oT#` before.
    Output,
    /// `i#`
    ConstInt,
    /// `oC#`
    ColorOut,
    /// `oDepth`
    DepthOut,
    /// `s#`
    Sampler,
    Const2,
    Const3,
    Const4,
    /// `b#`
    ConstBool,
    /// `aL`
    Loop,
    TempFloat16,
    /// `vPos` (0) and `vFace` (1).
    MiscType,
    /// `l#`
    Label,
    /// `p0`
    Predicate,
}

impl RegType {
    fn from_raw(raw: u32) -> Option<RegType> {
        use RegType as R;
        Some(match raw {
            0 => R::Temp,
            1 => R::Input,
            2 => R::Const,
            3 => R::Addr,
            4 => R::RastOut,
            5 => R::AttrOut,
            6 => R::Output,
            7 => R::ConstInt,
            8 => R::ColorOut,
            9 => R::DepthOut,
            10 => R::Sampler,
            11 => R::Const2,
            12 => R::Const3,
            13 => R::Const4,
            14 => R::ConstBool,
            15 => R::Loop,
            16 => R::TempFloat16,
            17 => R::MiscType,
            18 => R::Label,
            19 => R::Predicate,
            _ => return None,
        })
    }

    /// The raw register type number.
    #[must_use]
    pub fn raw(self) -> u32 {
        use RegType as R;
        match self {
            R::Temp => 0,
            R::Input => 1,
            R::Const => 2,
            R::Addr => 3,
            R::RastOut => 4,
            R::AttrOut => 5,
            R::Output => 6,
            R::ConstInt => 7,
            R::ColorOut => 8,
            R::DepthOut => 9,
            R::Sampler => 10,
            R::Const2 => 11,
            R::Const3 => 12,
            R::Const4 => 13,
            R::ConstBool => 14,
            R::Loop => 15,
            R::TempFloat16 => 16,
            R::MiscType => 17,
            R::Label => 18,
            R::Predicate => 19,
        }
    }
}

/// A register: file and number.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Register {
    /// Register file.
    pub kind: RegType,
    /// Register number within the file.
    pub num: u16,
}

/// A relative address: `[a0.c]` or `[aL]` added to a register number.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RelAddr {
    /// `a0` or `aL`.
    pub reg: Register,
    /// Component of the address register (0 = x).
    pub component: u8,
}

/// Source modifiers (`D3DSPSM_*`).
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SrcMod {
    None,
    Neg,
    /// `_bias`: x - 0.5
    Bias,
    BiasNeg,
    /// `_bx2`: 2x - 1
    Sign,
    SignNeg,
    /// `1 - x`
    Comp,
    /// `_x2`
    X2,
    X2Neg,
    /// `_dz` (pixel shader 1.4 texld only)
    Dz,
    /// `_dw` (pixel shader 1.4 texld only)
    Dw,
    Abs,
    AbsNeg,
    /// `!` on a boolean or predicate
    Not,
}

impl SrcMod {
    fn from_raw(raw: u32) -> Option<SrcMod> {
        use SrcMod as M;
        Some(match raw {
            0 => M::None,
            1 => M::Neg,
            2 => M::Bias,
            3 => M::BiasNeg,
            4 => M::Sign,
            5 => M::SignNeg,
            6 => M::Comp,
            7 => M::X2,
            8 => M::X2Neg,
            9 => M::Dz,
            10 => M::Dw,
            11 => M::Abs,
            12 => M::AbsNeg,
            13 => M::Not,
            _ => return None,
        })
    }
}

/// A destination parameter.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DstParam {
    /// Register written.
    pub reg: Register,
    /// Relative address (`o[aL + n]` in `vs_3_0`).
    pub relative: Option<RelAddr>,
    /// Write mask, bit 0 = x.
    pub write_mask: u8,
    /// `_sat`
    pub saturate: bool,
    /// `_pp`
    pub partial_precision: bool,
    /// `_centroid`
    pub centroid: bool,
    /// Shift scale (pixel shader 1.x); non-zero is rejected by the translator.
    pub shift: i8,
}

/// A source parameter.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SrcParam {
    /// Register read.
    pub reg: Register,
    /// Relative address (`c[a0.x + n]`, `c[aL + n]`, `v[aL + n]`).
    pub relative: Option<RelAddr>,
    /// Component selectors, x first (0 = x ... 3 = w).
    pub swizzle: [u8; 4],
    /// Source modifier.
    pub modifier: SrcMod,
}

/// Sampler texture type from `dcl_2d`, `dcl_cube`, `dcl_volume`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TextureType {
    /// `D3DSTT_UNKNOWN` (0) or another value; carries the raw value.
    Unknown(u8),
    /// `dcl_2d` (2)
    Tex2d,
    /// `dcl_cube` (3)
    Cube,
    /// `dcl_volume` (4)
    Volume,
}

/// The payload of a `dcl` instruction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Dcl {
    /// Raw `dcl` token.
    pub raw: u32,
    /// Usage (`D3DDECLUSAGE_*`) for inputs and outputs.
    pub usage: u8,
    /// Usage index.
    pub usage_index: u8,
    /// Texture type for samplers.
    pub texture_type: TextureType,
    /// The declared register, write mask and modifiers.
    pub dst: DstParam,
}

/// Extra payloads that are not ordinary parameters.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Payload {
    /// Ordinary instruction.
    None,
    /// `dcl`
    Dcl(Dcl),
    /// `def c#, x, y, z, w` as raw float bits.
    DefF([u32; 4]),
    /// `defi i#, x, y, z, w`.
    DefI([i32; 4]),
    /// `defb b#, bool`.
    DefB(bool),
}

/// One decoded instruction.
#[derive(Debug, Clone, PartialEq)]
pub struct Instruction {
    /// Dword offset of the instruction token from the start of the program.
    pub offset: usize,
    /// Decoded opcode.
    pub opcode: Opcode,
    /// Opcode-specific control bits 23:16.
    pub control: u8,
    /// Destination parameter, or the defined register for `def*`.
    pub dst: Option<DstParam>,
    /// Predicate source (`(p0.x) mov ...`).
    pub predicate: Option<SrcParam>,
    /// Source parameters.
    pub src: Vec<SrcParam>,
    /// `dcl` / `def*` payload.
    pub payload: Payload,
}

/// Decode a little-endian byte blob.
///
/// # Errors
///
/// [`Error::BadLength`] if the length is not a multiple of four, otherwise
/// whatever [`decode`] returns.
pub fn decode_bytes(bytes: &[u8]) -> Result<Shader, Error> {
    if !bytes.len().is_multiple_of(4) {
        return Err(Error::BadLength(bytes.len()));
    }
    let words: Vec<u32> = bytes
        .chunks_exact(4)
        .map(|c| u32::from_le_bytes([c[0], c[1], c[2], c[3]]))
        .collect();
    decode(&words)
}

/// Decode a token stream.
///
/// Accepts `vs_3_0` and `ps_3_0` only.
///
/// # Errors
///
/// A typed [`Error`] for any malformed or unsupported token.
pub fn decode(words: &[u32]) -> Result<Shader, Error> {
    let &version = words.first().ok_or(Error::BadLength(0))?;
    let stage = match version & 0xFFFF_0000 {
        VERTEX_VERSION_PREFIX => Stage::Vertex,
        PIXEL_VERSION_PREFIX => Stage::Pixel,
        _ => return Err(Error::BadVersion(version)),
    };
    let major = ((version >> 8) & 0xFF) as u8;
    let minor = (version & 0xFF) as u8;
    if (major, minor) != (3, 0) {
        return Err(Error::UnsupportedVersion { major, minor });
    }
    let mut instructions = Vec::new();
    let mut has_ctab = false;
    let mut pos = 1usize;
    loop {
        let &token = words.get(pos).ok_or(Error::MissingEnd)?;
        if token == END_TOKEN {
            return Ok(Shader {
                stage,
                major,
                minor,
                instructions,
                has_ctab,
                end_offset: pos + 1,
            });
        }
        if token & 0xFFFF == COMMENT_OPCODE {
            let len = ((token >> COMMENT_LENGTH_SHIFT) & COMMENT_LENGTH_MASK) as usize;
            let next = pos + 1 + len;
            if next > words.len() {
                return Err(Error::Truncated { offset: pos });
            }
            if len > 0 && words[pos + 1] == u32::from_le_bytes(*b"CTAB") {
                has_ctab = true;
            }
            pos = next;
            continue;
        }
        let len = ((token >> INSTR_LENGTH_SHIFT) & INSTR_LENGTH_MASK) as usize;
        let next = pos + 1 + len;
        if next > words.len() {
            return Err(Error::Truncated { offset: pos });
        }
        instructions.push(decode_instruction(pos, token, &words[pos + 1..next])?);
        pos = next;
    }
}

struct Cursor<'a> {
    body: &'a [u32],
    at: usize,
    offset: usize,
}

impl Cursor<'_> {
    fn next(&mut self) -> Result<u32, Error> {
        let &t = self.body.get(self.at).ok_or(Error::BadInstruction {
            offset: self.offset,
            reason: "parameter tokens run past the instruction length",
        })?;
        self.at += 1;
        Ok(t)
    }

    fn done(&self) -> bool {
        self.at >= self.body.len()
    }
}

fn bad(offset: usize, reason: &'static str) -> Error {
    Error::BadInstruction { offset, reason }
}

fn decode_instruction(offset: usize, token: u32, body: &[u32]) -> Result<Instruction, Error> {
    if token & PARAM_MARKER != 0 {
        return Err(bad(offset, "instruction token has bit 31 set"));
    }
    let raw = (token & 0xFFFF) as u16;
    let opcode = Opcode::from_raw(raw).ok_or(Error::UnknownOpcode {
        offset,
        opcode: raw,
    })?;
    if token & INSTR_COISSUE != 0 {
        return Err(Error::Unsupported {
            offset,
            what: "co-issue bit (pixel shader 1.x only)".into(),
        });
    }
    let control = ((token >> INSTR_CONTROL_SHIFT) & 0xFF) as u8;
    let predicated = token & INSTR_PREDICATED != 0;
    let mut cur = Cursor {
        body,
        at: 0,
        offset,
    };
    let mut ins = Instruction {
        offset,
        opcode,
        control,
        dst: None,
        predicate: None,
        src: Vec::new(),
        payload: Payload::None,
    };
    match opcode {
        Opcode::Dcl | Opcode::Def | Opcode::DefI | Opcode::DefB => {
            if predicated {
                return Err(bad(offset, "declaration carries the predicated bit"));
            }
            decode_declaration(&mut ins, &mut cur)?;
        }
        _ => {
            let Some((has_dst, min_src, max_src)) = opcode.operand_shape() else {
                return Err(Error::Unsupported {
                    offset,
                    what: format!(
                        "instruction {} (pixel shader 1.x only, not valid in shader model 3)",
                        opcode.name()
                    ),
                });
            };
            if has_dst {
                ins.dst = Some(parse_dst(&mut cur)?);
            }
            if predicated {
                ins.predicate = Some(parse_src(&mut cur)?);
            }
            while !cur.done() {
                if ins.src.len() >= usize::from(max_src) {
                    return Err(bad(
                        offset,
                        "more source parameters than the instruction takes",
                    ));
                }
                ins.src.push(parse_src(&mut cur)?);
            }
            if ins.src.len() < usize::from(min_src) {
                return Err(bad(
                    offset,
                    "fewer source parameters than the instruction takes",
                ));
            }
        }
    }
    if !cur.done() {
        return Err(bad(offset, "instruction length longer than its parameters"));
    }
    Ok(ins)
}

fn decode_declaration(ins: &mut Instruction, cur: &mut Cursor<'_>) -> Result<(), Error> {
    let offset = ins.offset;
    match ins.opcode {
        Opcode::Dcl => {
            let raw = cur.next()?;
            if raw & PARAM_MARKER == 0 {
                return Err(bad(offset, "dcl token lacks bit 31"));
            }
            let dst = parse_dst(cur)?;
            if dst.relative.is_some() {
                return Err(bad(offset, "dcl register uses relative addressing"));
            }
            let texture_type = match ((raw >> DCL_TEXTURE_TYPE_SHIFT) & DCL_TEXTURE_TYPE_MASK) as u8
            {
                2 => TextureType::Tex2d,
                3 => TextureType::Cube,
                4 => TextureType::Volume,
                other => TextureType::Unknown(other),
            };
            ins.dst = Some(dst);
            ins.payload = Payload::Dcl(Dcl {
                raw,
                usage: (raw & DCL_USAGE_MASK) as u8,
                usage_index: ((raw >> DCL_USAGE_INDEX_SHIFT) & DCL_USAGE_INDEX_MASK) as u8,
                texture_type,
                dst,
            });
        }
        Opcode::Def => {
            let dst = parse_dst(cur)?;
            let v = [cur.next()?, cur.next()?, cur.next()?, cur.next()?];
            ins.dst = Some(dst);
            ins.payload = Payload::DefF(v);
        }
        Opcode::DefI => {
            let dst = parse_dst(cur)?;
            let mut v = [0i32; 4];
            for slot in &mut v {
                *slot = cur.next()?.cast_signed();
            }
            ins.dst = Some(dst);
            ins.payload = Payload::DefI(v);
        }
        Opcode::DefB => {
            let dst = parse_dst(cur)?;
            let v = cur.next()?;
            ins.dst = Some(dst);
            ins.payload = Payload::DefB(v != 0);
        }
        _ => return Err(bad(offset, "not a declaration")),
    }
    if ins.dst.is_some_and(|d| d.relative.is_some()) {
        return Err(bad(offset, "declaration uses relative addressing"));
    }
    Ok(())
}

fn parse_register(token: u32, offset: usize) -> Result<Register, Error> {
    if token & PARAM_MARKER == 0 {
        return Err(bad(offset, "parameter token lacks bit 31"));
    }
    let raw_type = ((token >> REGTYPE_LOW_SHIFT) & REGTYPE_LOW_MASK)
        | (((token >> REGTYPE_HIGH_SHIFT) & REGTYPE_HIGH_MASK) << 3);
    let kind = RegType::from_raw(raw_type).ok_or(bad(offset, "unknown register type"))?;
    Ok(Register {
        kind,
        num: (token & REGNUM_MASK) as u16,
    })
}

fn swizzle_of(token: u32) -> [u8; 4] {
    let s = (token >> SWIZZLE_SHIFT) & 0xFF;
    [
        (s & 3) as u8,
        ((s >> 2) & 3) as u8,
        ((s >> 4) & 3) as u8,
        ((s >> 6) & 3) as u8,
    ]
}

fn parse_relative(cur: &mut Cursor<'_>) -> Result<RelAddr, Error> {
    let token = cur.next()?;
    let reg = parse_register(token, cur.offset)?;
    match reg.kind {
        RegType::Addr | RegType::Loop => {}
        _ => return Err(bad(cur.offset, "relative address register is not a0 or aL")),
    }
    Ok(RelAddr {
        reg,
        component: swizzle_of(token)[0],
    })
}

fn parse_dst(cur: &mut Cursor<'_>) -> Result<DstParam, Error> {
    let token = cur.next()?;
    let reg = parse_register(token, cur.offset)?;
    let relative = if token & RELATIVE_BIT != 0 {
        Some(parse_relative(cur)?)
    } else {
        None
    };
    let modifiers = (token >> RESULT_MOD_SHIFT) & 0xF;
    let shift_raw = ((token >> DST_SHIFT_SHIFT) & 0xF) as u8;
    // Four-bit two's complement.
    let shift = if shift_raw & 0x8 != 0 {
        (shift_raw | 0xF0).cast_signed()
    } else {
        shift_raw.cast_signed()
    };
    Ok(DstParam {
        reg,
        relative,
        write_mask: ((token >> WRITEMASK_SHIFT) & 0xF) as u8,
        saturate: modifiers & RESULT_SATURATE != 0,
        partial_precision: modifiers & RESULT_PARTIAL_PRECISION != 0,
        centroid: modifiers & RESULT_CENTROID != 0,
        shift,
    })
}

fn parse_src(cur: &mut Cursor<'_>) -> Result<SrcParam, Error> {
    let token = cur.next()?;
    let reg = parse_register(token, cur.offset)?;
    let relative = if token & RELATIVE_BIT != 0 {
        Some(parse_relative(cur)?)
    } else {
        None
    };
    let modifier = SrcMod::from_raw((token >> SRC_MOD_SHIFT) & 0xF)
        .ok_or(bad(cur.offset, "unknown source modifier"))?;
    Ok(SrcParam {
        reg,
        relative,
        swizzle: swizzle_of(token),
        modifier,
    })
}
