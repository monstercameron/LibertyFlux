//! Translator from Direct3D 9 shader model 3.0 bytecode (`vs_3_0`,
//! `ps_3_0`, the public "DXSO" token format) to SPIR-V 1.0 for Vulkan.
//!
//! The game's shaders are compiled Direct3D 9 programs inside `.fxc`
//! containers (parsed by `lf-shaderpack`). This crate converts each program
//! on the player's own machine, from the player's own purchased copy, so
//! the renderer needs no external shader tools and no game-derived SPIR-V
//! is ever distributed. It has no dependencies and no `unsafe` code.
//!
//! ```text
//! bytes/words --decode--> Shader (typed instructions) --translate--> Module
//!                                                       (SPIR-V words + reflection)
//! ```
//!
//! Labels used below: **Verified** means pinned by a test in this crate
//! that runs the code; **Inferred** means written from public
//! documentation or chosen by this project without a reference to check
//! against in this environment (no Direct3D headers, no `spirv-val`, no
//! game files were available when the crate was written). Every
//! Direct3D 9 semantic choice that is a guess is marked Inferred.
//!
//! # 1. Decoder ([`mod@decode`])
//!
//! The token layout is in the [`mod@decode`] module documentation (Inferred
//! from Microsoft's public `d3d9types.h` documentation; its round trip with
//! the hand-built test assembler is Verified). The decoder:
//!
//! * accepts version tokens `0xFFFE0300` (`vs_3_0`) and `0xFFFF0300`
//!   (`ps_3_0`) only; any other version is [`Error::UnsupportedVersion`];
//! * skips comments (`CTAB` presence is recorded, its contents are not
//!   parsed: the translator does not need names);
//! * decodes the destination (register type split across bits 30:28 and
//!   12:11, write mask, saturate / partial precision / centroid, shift),
//!   the predicate token that follows the destination when bit 28 is set,
//!   sources (swizzle, all fourteen source modifiers), the extra relative
//!   address token (`a0.c` or `aL`) after any parameter with bit 13 set,
//!   `dcl` (usage, usage index, sampler texture type), `def`, `defi`,
//!   `defb` and the end token;
//! * checks that every instruction's parameters exactly fill its length
//!   field and that the operand count fits the opcode, and returns a typed
//!   [`Error`] for anything malformed. It never panics (Verified by the
//!   robustness test over truncations, random words and bit flips).
//!
//! # 2. Translation
//!
//! Every register file becomes module-scope storage so that values cross
//! structured blocks and subroutines without phi nodes:
//!
//! | Direct3D 9 | SPIR-V |
//! |---|---|
//! | `r#` | `Private vec4`, zero-initialised |
//! | `v#` (vertex) | `Input vec4` per `dcl`, location from [`binding::VERTEX_INPUT_LOCATIONS`] |
//! | `v#` (pixel) | `Input vec4` per `dcl`, location from [`binding::VARYING_LOCATIONS`]; `Centroid` from `_centroid`; copied to a `Private vec4[10]` at entry if any `v[aL + n]` read exists |
//! | `c#` | uniform block member `c[n]`; a `def` replaces the read with a constant |
//! | `c[a0.x + n]`, `c[aL + n]` | index clamped for the load, then 0 if out of range, then each `def`'d register selected by index equality (so defs override relative reads too) |
//! | `i#`, `b#` | uniform block `{ ivec4 i[16]; uint b; }`; `defi`/`defb` replace reads |
//! | `s#` | `UniformConstant` combined image sampler, dimension from `dcl_2d`/`dcl_cube`/`dcl_volume` |
//! | `o#` (vertex) | `Private vec4` staging register, copied to its outputs before every return from `main` |
//! | `o[aL + n]` | the staging registers become a `Private vec4[12]` |
//! | `oPos` (`dcl_position o#`) | `BuiltIn Position`, with the push-constant fixup (see [`binding`]) |
//! | `oPts` (`dcl_psize o#`) | `BuiltIn PointSize` from the first masked component |
//! | `oFog` (`dcl_fog o#`) and other varyings | `Output vec4` at the varying location |
//! | `oC0`-`oC3` | `Private` staging, copied to `Output vec4` locations 0-3 at return |
//! | `oDepth` | `Private` staging, copied to `BuiltIn FragDepth`; `DepthReplacing` mode |
//! | `vPos` | `BuiltIn FragCoord` with `.xy - 0.5` (Direct3D 9 pixel centres are integers, Vulkan's are at .5) |
//! | `vFace` | `BuiltIn FrontFacing` mapped to `+1.0` / `-1.0` in every component |
//! | `a0` | `Private ivec4` written by `mova` |
//! | `aL` | `Private int`, saved before each `loop` and restored after it |
//! | `p0` | `Private bvec4` written by `setp` |
//! | `l#` | a SPIR-V function; `call` is `OpFunctionCall` |
//!
//! When a vertex shader declares two semantics in one output register
//! (`dcl_texcoord0 o3.xy`, `dcl_texcoord1 o3.zw`), each output variable
//! receives the whole staging register, so components keep their
//! positions; a pixel input register with several declarations is
//! assembled the same way. That component-preserving linkage is Inferred.
//!
//! Instructions (all arithmetic is done on `vec4`; a scalar result is
//! replicated, then the write mask applies):
//!
//! | Instruction | SPIR-V and Direct3D 9 semantics |
//! |---|---|
//! | `mov add sub mul` | `FAdd`, `FSub`, `FMul` (IEEE; the 0 × inf = 0 rule of some Direct3D 9 hardware is not emulated, Inferred) |
//! | `mad` | `FMul` then `FAdd`, not fused (Inferred) |
//! | `lrp` | `src0 * (src1 - src2) + src2` |
//! | `min max` | `GLSL.std.450 FMin/FMax` (NaN handling implementation-defined) |
//! | `slt sge` | ordered compare, `select(1.0, 0.0)`: NaN gives 0.0 |
//! | `cmp` | `src0 >= 0 ? src1 : src2` per component (ordered: NaN picks `src2`) |
//! | `cnd` | `src0 > 0.5 ? src1 : src2` per component |
//! | `rcp` | `1.0 / x` (IEEE division: `rcp(+0) = +inf`, `rcp(-0) = -inf`; Inferred to match Direct3D 9) |
//! | `rsq` | `1.0 / sqrt(abs(x))`, so `rsq(0) = +inf` (`InverseSqrt` is avoided because it is undefined at 0) |
//! | `exp expp` | `exp2(x)`; `expp` uses full precision (Inferred: model 2+ `expp` returns a scalar) |
//! | `log logp` | `log2(abs(x))`, with `log(0) = -inf` made explicit |
//! | `pow` | `exp2(y * log2(abs(x)))`: `pow(0, y>0) = 0`, `pow(0, 0) = NaN` (Inferred; Direct3D documents `abs(x)^y`) |
//! | `frc abs sgn` | `Fract`, `FAbs`, `FSign` (the model 2 scratch operands of `sgn` are ignored) |
//! | `dp2add dp3 dp4` | `OpDot` on 2/3/4 components (+ `src2` first component) |
//! | `crs` | `Cross` of `.xyz`; `.w` is never written |
//! | `nrm` | `src * (1 / sqrt(dot3(src, src)))`, including `.w`; a zero vector gives NaN (Inferred) |
//! | `sincos` | `.x = cos`, `.y = sin` of the first source component; other components are not written |
//! | `m4x4 m4x3 m3x4 m3x3 m3x2` | one `OpDot` per row against consecutive registers (relative addressing applies to each row) |
//! | `dst` | `(1, src0.y * src1.y, src0.z, src1.w)` |
//! | `lit` | `(1, x > 0 ? x : 0, x > 0 && y > 0 ? y^clamp(w, ±127.9961) : 0, 1)` |
//! | `mova` | `a0 = int(floor(src + 0.5))` (round half up; Inferred) |
//! | `setp_*` | per-component compare into `p0` |
//! | `if b#`, `if [!]p0.c`, `if_*` | `OpSelectionMerge` + `OpBranchConditional` |
//! | `rep i#` | counted loop, `i#.x` iterations |
//! | `loop aL, i#` | `i#.x` iterations, `aL = i#.y`, `aL += i#.z` in the continue block |
//! | `break`, `break_*`, `breakp` | branch to the innermost loop's merge block |
//! | `call`, `callnz b#`, `callnz [!]p0.c` | `OpFunctionCall` (recursion is rejected) |
//! | `ret` | end of function; inside flow control, `OpReturn` |
//! | `texld` | `OpImageSampleImplicitLod` (pixel shaders only) |
//! | `texldp` | coordinate divided by `.w` in the shader, then sampled |
//! | `texldb` | `Bias` image operand = `.w` |
//! | `texldl` | `OpImageSampleExplicitLod`, `Lod` = `.w` |
//! | `texldd` | `OpImageSampleExplicitLod`, `Grad` |
//! | `texkill` | `OpKill` if any masked component is `< 0` |
//! | `dsx dsy` | `OpDPdx`, `OpDPdy` |
//!
//! Comparisons for `if_*`, `break_*` and `setp_*`: `gt ge lt le eq` are
//! ordered, `ne` is unordered (NaN compares not-equal; Inferred). Scalar
//! instructions read the first component of the swizzled source, which is
//! exactly the documented behaviour for the replicate swizzles compilers
//! emit (Inferred for other swizzles).
//!
//! Modifiers: `_sat` is `x > 0 ? min(x, 1) : 0` (NaN saturates to 0;
//! Inferred). `_pp` is ignored (full precision is always allowed).
//! `_centroid` decorates pixel inputs. Source `neg`, `abs`, `abs neg`
//! and the shader model 1 modifiers `bias`, `bx2`, `comp`, `x2` are
//! implemented; `!` is accepted on booleans and the predicate only.
//!
//! A predicated instruction (`(p0.c) op ...`) computes its result
//! unconditionally and then writes each masked component only where the
//! swizzled (and possibly negated) predicate is true. `texkill` and
//! `mova`/`setp` honour the predicate as well.
//!
//! # 3. Not supported
//!
//! Each of these returns [`Error::Unsupported`]; nothing is translated
//! silently wrong:
//!
//! * shader models other than 3.0 (including `vs_3_sw`/`ps_3_sw`);
//! * the pixel shader 1.x instructions (`texcoord`, `texbem`, `texbeml`,
//!   `texreg2*`, `texm3x*`, `texdp3*`, `texdepth`, `bem`, `phase`),
//!   co-issue, destination shift and the `_dz`/`_dw` source modifiers,
//!   none of which is valid in model 3;
//! * register files that model 3 does not have (`oPos`/`oFog`/`oPts` as
//!   `RASTOUT`, `oD#`, `t#`, the half-float temp file, `c` banks 2-4);
//! * reading `a0`, `aL`, `i#`, `b#`, `p0`, `o#` or `oC#` as a float source;
//! * `texld` with an implicit level of detail in a vertex shader;
//! * predicated flow-control instructions (`if`, `loop`, `break`, `call`, ...);
//! * `dcl` usages missing from the location tables (for example a vertex
//!   input `texcoord8`, `position1` or `psize`, or a pixel input
//!   `position0`), sampler texture types other than 2D, cube and volume,
//!   and depth-compare samplers that are not 2D.
//!
//! Semantically invalid programs (an undeclared input, a register out of
//! range, an unmatched `endif`, a call to an undefined label, recursion)
//! return [`Error::Invalid`].
//!
//! # 4. Vulkan binding convention
//!
//! See [`binding`]; every number lives there as a constant.
//!
//! # 5. Output
//!
//! [`translate`] returns a [`Module`]: the SPIR-V words plus the
//! reflection a renderer needs (stage, inputs and outputs with usage and
//! location, samplers with set, binding and dimension, which constant
//! registers are read, which buffers and push constants exist).
//!
//! # 6. Self-check
//!
//! [`validate::validate`] is a structural SPIR-V validator used by every
//! test (no `spirv-val` is available here): header, logical layout order,
//! single definition of every id, definition before use, types before use,
//! entry point and execution modes, well-formed blocks with structured
//! merges, operand types of the instructions this crate emits, and the
//! Vulkan interface decorations. An ignored test runs the real `spirv-val`
//! when `LF_SPIRV_VAL` points at it.

#![forbid(unsafe_code)]

pub mod binding;
pub mod decode;
pub mod spirv;
mod translate;
pub mod validate;

pub use decode::{RegType, Register, Shader, Stage, decode, decode_bytes};

/// Everything that can go wrong. Offsets are dword indexes from the start
/// of the program (the version token is offset 0).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    /// Input is empty or its byte length is not a multiple of four.
    BadLength(usize),
    /// The first token is not a vertex or pixel version token.
    BadVersion(u32),
    /// A shader model other than 3.0.
    UnsupportedVersion {
        /// Major version.
        major: u8,
        /// Minor version.
        minor: u8,
    },
    /// Tokens ran out before the end token.
    MissingEnd,
    /// An instruction or comment runs past the end of the input.
    Truncated {
        /// Offset of the instruction or comment token.
        offset: usize,
    },
    /// An opcode number that Direct3D 9 does not define.
    UnknownOpcode {
        /// Offset of the instruction token.
        offset: usize,
        /// The raw opcode.
        opcode: u16,
    },
    /// The instruction's parameter tokens are malformed.
    BadInstruction {
        /// Offset of the instruction token.
        offset: usize,
        /// What is wrong.
        reason: &'static str,
    },
    /// Well-formed tokens that do not make a valid shader model 3 program.
    Invalid {
        /// Offset of the instruction token.
        offset: usize,
        /// What is wrong.
        reason: String,
    },
    /// A valid construct this translator does not handle.
    Unsupported {
        /// Offset of the instruction token.
        offset: usize,
        /// What is not supported.
        what: String,
    },
}

impl Error {
    /// Whether this is [`Error::Unsupported`] or [`Error::UnsupportedVersion`].
    #[must_use]
    pub fn is_unsupported(&self) -> bool {
        matches!(
            self,
            Error::Unsupported { .. } | Error::UnsupportedVersion { .. }
        )
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::BadLength(n) => write!(f, "bytecode length {n} is not a whole number of tokens"),
            Error::BadVersion(v) => write!(f, "bad version token 0x{v:08x}"),
            Error::UnsupportedVersion { major, minor } => {
                write!(
                    f,
                    "shader model {major}.{minor} is not supported (3.0 only)"
                )
            }
            Error::MissingEnd => write!(f, "no end token"),
            Error::Truncated { offset } => write!(f, "token at {offset} runs past the end"),
            Error::UnknownOpcode { offset, opcode } => {
                write!(f, "unknown opcode {opcode} at {offset}")
            }
            Error::BadInstruction { offset, reason } => {
                write!(f, "malformed instruction at {offset}: {reason}")
            }
            Error::Invalid { offset, reason } => write!(f, "invalid program at {offset}: {reason}"),
            Error::Unsupported { offset, what } => write!(f, "unsupported at {offset}: {what}"),
        }
    }
}

impl std::error::Error for Error {}

/// Translation options.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Options {
    /// Apply the push-constant position fixup in vertex shaders (see
    /// [`binding`]). Default on.
    pub position_fixup: bool,
    /// Bit `n` set: pixel/vertex sampler `s#n` samples a depth texture
    /// with comparison (`OpImageSampleDref*`, reference from the
    /// coordinate's `.z`, result replicated to all four components). A
    /// Direct3D 9 program does not say this; the renderer knows it from
    /// the bound texture format. Default none.
    pub depth_compare_samplers: u16,
}

impl Default for Options {
    fn default() -> Self {
        Options {
            position_fixup: true,
            depth_compare_samplers: 0,
        }
    }
}

/// A built-in variable used by a translated shader.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BuiltIn {
    /// Vertex `Position` (from `dcl_position o#`).
    Position,
    /// Vertex `PointSize` (from `dcl_psize o#`).
    PointSize,
    /// Fragment `FragCoord` (from `vPos`).
    FragCoord,
    /// Fragment `FrontFacing` (from `vFace`).
    FrontFacing,
    /// Fragment `FragDepth` (from `oDepth`).
    FragDepth,
}

/// One input or output of a translated shader.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InterfaceVar {
    /// The Direct3D 9 register.
    pub reg: Register,
    /// `(usage, usage index)` from the `dcl`, if declared with one.
    pub usage: Option<(u8, u8)>,
    /// Declared write mask (bit 0 = x).
    pub write_mask: u8,
    /// Vulkan location, for non-built-ins.
    pub location: Option<u32>,
    /// Built-in, if any.
    pub builtin: Option<BuiltIn>,
    /// Declared `_centroid`.
    pub centroid: bool,
}

/// Sampler texture dimension.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextureDim {
    /// `dcl_2d`
    D2,
    /// `dcl_cube`
    Cube,
    /// `dcl_volume`
    D3,
}

/// One sampler binding.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SamplerBinding {
    /// Sampler register `s#`.
    pub register: u16,
    /// Descriptor set.
    pub set: u32,
    /// Binding within the set.
    pub binding: u32,
    /// Image dimension.
    pub dim: TextureDim,
    /// Whether it was translated as a depth-compare sampler.
    pub depth_compare: bool,
}

/// Which constant registers a shader reads.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ConstantUse {
    /// Float registers read directly from the buffer (sorted; `def`'d
    /// registers excluded).
    pub float_read: Vec<u16>,
    /// Whether any float register is read with relative addressing (then
    /// the whole buffer may be read).
    pub float_relative: bool,
    /// Float registers defined in the shader by `def`.
    pub float_defined: Vec<u16>,
    /// Integer registers read from the buffer.
    pub int_read: Vec<u16>,
    /// Integer registers defined by `defi`.
    pub int_defined: Vec<u16>,
    /// Boolean registers read from the buffer.
    pub bool_read: Vec<u16>,
    /// Boolean registers defined by `defb`.
    pub bool_defined: Vec<u16>,
}

/// A translated shader.
#[derive(Debug, Clone, PartialEq)]
pub struct Module {
    /// SPIR-V 1.0 words, entry point `main`.
    pub words: Vec<u32>,
    /// Vertex or pixel (fragment).
    pub stage: Stage,
    /// Inputs in declaration order.
    pub inputs: Vec<InterfaceVar>,
    /// Outputs in declaration order.
    pub outputs: Vec<InterfaceVar>,
    /// Samplers, sorted by register.
    pub samplers: Vec<SamplerBinding>,
    /// Constant register usage.
    pub constants: ConstantUse,
    /// Binding in [`binding::CONSTANT_SET`] of the float constant buffer,
    /// if the shader reads one.
    pub float_constant_binding: Option<u32>,
    /// Binding in [`binding::CONSTANT_SET`] of the integer/boolean buffer,
    /// if the shader reads one.
    pub int_bool_constant_binding: Option<u32>,
    /// Bytes of vertex push constants used (0 or
    /// [`binding::PUSH_CONSTANT_SIZE`]).
    pub push_constant_size: u32,
    /// Whether the fragment shader writes depth.
    pub depth_replacing: bool,
}

impl Module {
    /// The SPIR-V as little-endian bytes (a `.spv` file).
    #[must_use]
    pub fn to_bytes(&self) -> Vec<u8> {
        self.words.iter().flat_map(|w| w.to_le_bytes()).collect()
    }
}

/// Translate a token stream with default [`Options`].
///
/// # Errors
///
/// See [`Error`].
pub fn translate(words: &[u32]) -> Result<Module, Error> {
    translate_with(words, &Options::default())
}

/// Translate a little-endian byte blob with default [`Options`].
///
/// # Errors
///
/// See [`Error`].
pub fn translate_bytes(bytes: &[u8]) -> Result<Module, Error> {
    let shader = decode_bytes(bytes)?;
    translate_shader(&shader, &Options::default())
}

/// Translate a token stream.
///
/// # Errors
///
/// See [`Error`].
pub fn translate_with(words: &[u32], options: &Options) -> Result<Module, Error> {
    let shader = decode(words)?;
    translate_shader(&shader, options)
}

/// Translate an already decoded shader.
///
/// # Errors
///
/// See [`Error`].
pub fn translate_shader(shader: &Shader, options: &Options) -> Result<Module, Error> {
    translate::translate(shader, *options)
}
