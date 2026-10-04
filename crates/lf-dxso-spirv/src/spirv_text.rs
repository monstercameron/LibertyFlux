//! A readable text form of the SPIR-V this crate emits, and a summary of a
//! translated [`Module`]'s reflection, so a translation can be inspected
//! without external tools (no `spirv-dis` is available here).
//!
//! **The text of a module translated from the game's own shaders is
//! derived from game files.** It is a debugging aid for the person who owns
//! the game: keep it on your own machine and never commit, paste or share
//! it (`AGENTS.md`, rules 1 and 6).
//!
//! The layout follows the public SPIR-V assembly convention: five header
//! comment lines, then one instruction per line, with the result id
//! right-aligned before `=`, then the opcode name and its operands:
//!
//! ```text
//!           %main = OpFunction %2 None %12
//!             %33 = OpLoad %6 %v0_texcoord0
//!                   OpDecorate %v0_texcoord0 Location 0
//! ```
//!
//! * Every opcode in [`crate::spirv::op`] has a name ([`opcode_name`]);
//!   any other opcode prints as `Op<number>` followed by its raw operand
//!   words in hexadecimal, and is listed in [`SpirvText::unknown`].
//! * Ids print as `%number`, or as `%name` when an `OpName` gives the id a
//!   name that no other id shares and that is a plain identifier (letters,
//!   digits, `_`, not starting with a digit), so a friendly name never
//!   collides with a number or another name. [`TextOptions::names`] turns
//!   names off, to match the `%number` form [`crate::validate`] reports.
//! * Enumerants print by name when [`crate::spirv`] defines the value
//!   (capabilities, models, modes, storage classes, decorations, built-ins,
//!   dimensions, image operands, the `GLSL.std.450` instructions); any
//!   other value prints as its number. Masks of zero print as `None`.
//! * `OpConstant` values print by their result type: 32-bit floats in
//!   Rust's shortest round-trip notation (a NaN as its bits,
//!   `nan(0x7fc00000)`), 32-bit integers in decimal.
//! * An instruction whose operand words do not fit the grammar this module
//!   expects (too few, too many, an unterminated string) is printed with
//!   the leftover words as `; extra 0x...` and listed in
//!   [`SpirvText::mismatched`]; a header or word-count problem stops the
//!   walk and is reported in [`SpirvText::error`]. Nothing panics.
//!
//! Opcode and enumerant names are the public Khronos SPIR-V 1.0 and
//! `GLSL.std.450` names (Inferred, like the numbers in [`crate::spirv`]).

use std::collections::HashMap;
use std::fmt::Write as _;

use crate::binding;
use crate::listing::register_name;
use crate::spirv::{
    self, builtin, capability, decoration, dim, glsl, image_operands, mode, model, op, storage,
};
use crate::validate::{ValidationError, read_string};
use crate::{InterfaceVar, Module, Stage, TextureDim};

/// Words in the SPIR-V header.
const HEADER_WORDS: usize = 5;
/// Width of the right-aligned result-id column (`%id = `).
const RESULT_COLUMN: usize = 14;
/// The only extended instruction set this crate imports.
const GLSL_SET_NAME: &str = "GLSL.std.450";
/// Image format `Unknown` (the value [`crate::spirv::Builder::t_image`] writes).
const IMAGE_FORMAT_UNKNOWN: u32 = 0;

/// How [`disassemble_with`] prints a module.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TextOptions {
    /// Print ids by their unique `OpName` where there is one.
    pub names: bool,
    /// Start every instruction line with its word offset, the offset
    /// [`crate::validate::ValidationError`] reports.
    pub offsets: bool,
}

impl Default for TextOptions {
    fn default() -> Self {
        TextOptions {
            names: true,
            offsets: false,
        }
    }
}

/// The text form of a module and what the dumper could not account for.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpirvText {
    /// The text, one line per instruction after the header comments.
    pub text: String,
    /// Instructions printed.
    pub instructions: usize,
    /// `(word offset, opcode)` of every instruction with an opcode this
    /// module has no name for.
    pub unknown: Vec<(usize, u16)>,
    /// Word offsets of instructions whose operand words did not match the
    /// expected grammar.
    pub mismatched: Vec<usize>,
    /// A header problem, or where the instruction walk had to stop.
    pub error: Option<ValidationError>,
}

impl SpirvText {
    /// Whether every word was accounted for: no unknown opcode, no operand
    /// mismatch, no structural error.
    #[must_use]
    pub fn is_clean(&self) -> bool {
        self.unknown.is_empty() && self.mismatched.is_empty() && self.error.is_none()
    }
}

/// The SPIR-V name of every opcode this crate emits (`OpFAdd`, ...).
#[must_use]
pub fn opcode_name(opcode: u16) -> Option<&'static str> {
    grammar(opcode).map(|(name, _)| name)
}

/// The `GLSL.std.450` name of every extended instruction this crate emits.
#[must_use]
pub fn glsl_name(instruction: u32) -> Option<&'static str> {
    Some(match instruction {
        glsl::FABS => "FAbs",
        glsl::FSIGN => "FSign",
        glsl::FLOOR => "Floor",
        glsl::FRACT => "Fract",
        glsl::SIN => "Sin",
        glsl::COS => "Cos",
        glsl::EXP2 => "Exp2",
        glsl::LOG2 => "Log2",
        glsl::SQRT => "Sqrt",
        glsl::FMIN => "FMin",
        glsl::FMAX => "FMax",
        glsl::SCLAMP => "SClamp",
        glsl::CROSS => "Cross",
        _ => return None,
    })
}

/// The text of a module with default [`TextOptions`].
#[must_use]
pub fn disassemble(words: &[u32]) -> SpirvText {
    disassemble_with(words, &TextOptions::default())
}

/// The text of a module; see the [module documentation](self).
#[must_use]
pub fn disassemble_with(words: &[u32], options: &TextOptions) -> SpirvText {
    let mut d = Dumper {
        options: *options,
        names: HashMap::new(),
        scalar_types: HashMap::new(),
        glsl_set: None,
        out: SpirvText {
            text: String::new(),
            instructions: 0,
            unknown: Vec::new(),
            mismatched: Vec::new(),
            error: None,
        },
    };
    if words.len() < HEADER_WORDS {
        d.fail(0, "module shorter than its header");
        return d.out;
    }
    if words[0] != spirv::MAGIC {
        d.fail(0, format!("bad magic number 0x{:08x}", words[0]));
        return d.out;
    }
    let version = words[1];
    let _ = writeln!(d.out.text, "; SPIR-V");
    let _ = writeln!(
        d.out.text,
        "; Version: {}.{}",
        (version >> 16) & 0xFF,
        (version >> 8) & 0xFF
    );
    let _ = writeln!(d.out.text, "; Generator: {}", words[2]);
    let _ = writeln!(d.out.text, "; Bound: {}", words[3]);
    let _ = writeln!(d.out.text, "; Schema: {}", words[4]);
    if options.names {
        d.names = friendly_names(words);
    }
    let mut pos = HEADER_WORDS;
    while pos < words.len() {
        let count = (words[pos] >> 16) as usize;
        let opcode = (words[pos] & 0xFFFF) as u16;
        if count == 0 {
            d.fail(pos, "instruction word count is zero");
            break;
        }
        if pos + count > words.len() {
            d.fail(pos, "instruction runs past the end of the module");
            break;
        }
        d.instruction(pos, opcode, &words[pos + 1..pos + count]);
        pos += count;
    }
    d.out
}

/// Kinds of operand, in the order an instruction's words hold them.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum K {
    /// Result type id (printed first after `=`).
    Ty,
    /// Result id (printed before `=`).
    Res,
    /// One id.
    Id,
    /// An optional id.
    OptId,
    /// All remaining words are ids.
    Ids,
    /// One literal number.
    Lit,
    /// All remaining words are literal numbers.
    Lits,
    /// A string literal.
    Str,
    /// `OpConstant` value words, typed by the result type.
    Value,
    /// A capability.
    Capability,
    /// An addressing model.
    Addressing,
    /// A memory model.
    Memory,
    /// An execution model.
    ExecModel,
    /// An execution mode and its literal operands.
    ExecMode,
    /// A storage class.
    Storage,
    /// A decoration and its operands.
    Decoration,
    /// An image dimension.
    Dim,
    /// An image format.
    Format,
    /// A function-control mask.
    FnControl,
    /// A selection-control mask.
    SelControl,
    /// A loop-control mask and its literal operands.
    LoopControl,
    /// Optional image operands: a mask, then ids.
    ImageOps,
    /// The instruction number of the set named by the previous id.
    ExtInst,
}

/// Unary arithmetic, conversion and derivative instructions.
const UNARY: &[K] = &[K::Ty, K::Res, K::Id];
/// Binary arithmetic, comparison, logical and bitwise instructions.
const BINARY: &[K] = &[K::Ty, K::Res, K::Id, K::Id];

/// Name and operand grammar of every opcode this crate emits.
#[allow(clippy::too_many_lines)]
fn grammar(opcode: u16) -> Option<(&'static str, &'static [K])> {
    use K::{
        Addressing, Capability, Decoration, Dim, ExecMode, ExecModel, ExtInst, FnControl, Format,
        Id, Ids, ImageOps, Lit, Lits, LoopControl, Memory, OptId, Res, SelControl, Storage, Str,
        Ty, Value,
    };
    Some(match opcode {
        op::NAME => ("OpName", &[Id, Str]),
        op::MEMBER_NAME => ("OpMemberName", &[Id, Lit, Str]),
        op::EXT_INST_IMPORT => ("OpExtInstImport", &[Res, Str]),
        op::EXT_INST => ("OpExtInst", &[Ty, Res, Id, ExtInst, Ids]),
        op::MEMORY_MODEL => ("OpMemoryModel", &[Addressing, Memory]),
        op::ENTRY_POINT => ("OpEntryPoint", &[ExecModel, Id, Str, Ids]),
        op::EXECUTION_MODE => ("OpExecutionMode", &[Id, ExecMode]),
        op::CAPABILITY => ("OpCapability", &[Capability]),
        op::TYPE_VOID => ("OpTypeVoid", &[Res]),
        op::TYPE_BOOL => ("OpTypeBool", &[Res]),
        op::TYPE_INT => ("OpTypeInt", &[Res, Lit, Lit]),
        op::TYPE_FLOAT => ("OpTypeFloat", &[Res, Lit]),
        op::TYPE_VECTOR => ("OpTypeVector", &[Res, Id, Lit]),
        // Sampled type, Dim, Depth, Arrayed, MS, Sampled, Format, access qualifier.
        op::TYPE_IMAGE => (
            "OpTypeImage",
            &[Res, Id, Dim, Lit, Lit, Lit, Lit, Format, Lits],
        ),
        op::TYPE_SAMPLED_IMAGE => ("OpTypeSampledImage", &[Res, Id]),
        op::TYPE_ARRAY => ("OpTypeArray", &[Res, Id, Id]),
        op::TYPE_STRUCT => ("OpTypeStruct", &[Res, Ids]),
        op::TYPE_POINTER => ("OpTypePointer", &[Res, Storage, Id]),
        op::TYPE_FUNCTION => ("OpTypeFunction", &[Res, Id, Ids]),
        op::CONSTANT_TRUE => ("OpConstantTrue", &[Ty, Res]),
        op::CONSTANT_FALSE => ("OpConstantFalse", &[Ty, Res]),
        op::CONSTANT => ("OpConstant", &[Ty, Res, Value]),
        op::CONSTANT_COMPOSITE => ("OpConstantComposite", &[Ty, Res, Ids]),
        op::FUNCTION => ("OpFunction", &[Ty, Res, FnControl, Id]),
        op::FUNCTION_END => ("OpFunctionEnd", &[]),
        op::FUNCTION_CALL => ("OpFunctionCall", &[Ty, Res, Id, Ids]),
        op::VARIABLE => ("OpVariable", &[Ty, Res, Storage, OptId]),
        // The memory-access operands this crate never emits print as numbers.
        op::LOAD => ("OpLoad", &[Ty, Res, Id, Lits]),
        op::STORE => ("OpStore", &[Id, Id, Lits]),
        op::ACCESS_CHAIN => ("OpAccessChain", &[Ty, Res, Id, Ids]),
        op::DECORATE => ("OpDecorate", &[Id, Decoration]),
        op::MEMBER_DECORATE => ("OpMemberDecorate", &[Id, Lit, Decoration]),
        op::VECTOR_SHUFFLE => ("OpVectorShuffle", &[Ty, Res, Id, Id, Lits]),
        op::COMPOSITE_CONSTRUCT => ("OpCompositeConstruct", &[Ty, Res, Ids]),
        op::COMPOSITE_EXTRACT => ("OpCompositeExtract", &[Ty, Res, Id, Lits]),
        op::IMAGE_SAMPLE_IMPLICIT_LOD => ("OpImageSampleImplicitLod", &[Ty, Res, Id, Id, ImageOps]),
        op::IMAGE_SAMPLE_EXPLICIT_LOD => ("OpImageSampleExplicitLod", &[Ty, Res, Id, Id, ImageOps]),
        op::IMAGE_SAMPLE_DREF_IMPLICIT_LOD => (
            "OpImageSampleDrefImplicitLod",
            &[Ty, Res, Id, Id, Id, ImageOps],
        ),
        op::IMAGE_SAMPLE_DREF_EXPLICIT_LOD => (
            "OpImageSampleDrefExplicitLod",
            &[Ty, Res, Id, Id, Id, ImageOps],
        ),
        op::CONVERT_F_TO_S => ("OpConvertFToS", UNARY),
        op::F_NEGATE => ("OpFNegate", UNARY),
        op::I_ADD => ("OpIAdd", BINARY),
        op::F_ADD => ("OpFAdd", BINARY),
        op::F_SUB => ("OpFSub", BINARY),
        op::F_MUL => ("OpFMul", BINARY),
        op::F_DIV => ("OpFDiv", BINARY),
        op::DOT => ("OpDot", BINARY),
        op::ANY => ("OpAny", UNARY),
        op::LOGICAL_OR => ("OpLogicalOr", BINARY),
        op::LOGICAL_AND => ("OpLogicalAnd", BINARY),
        op::LOGICAL_NOT => ("OpLogicalNot", UNARY),
        op::SELECT => ("OpSelect", &[Ty, Res, Id, Id, Id]),
        op::I_EQUAL => ("OpIEqual", BINARY),
        op::I_NOT_EQUAL => ("OpINotEqual", BINARY),
        op::S_GREATER_THAN_EQUAL => ("OpSGreaterThanEqual", BINARY),
        op::S_LESS_THAN => ("OpSLessThan", BINARY),
        op::F_ORD_EQUAL => ("OpFOrdEqual", BINARY),
        op::F_UNORD_NOT_EQUAL => ("OpFUnordNotEqual", BINARY),
        op::F_ORD_LESS_THAN => ("OpFOrdLessThan", BINARY),
        op::F_ORD_GREATER_THAN => ("OpFOrdGreaterThan", BINARY),
        op::F_ORD_LESS_THAN_EQUAL => ("OpFOrdLessThanEqual", BINARY),
        op::F_ORD_GREATER_THAN_EQUAL => ("OpFOrdGreaterThanEqual", BINARY),
        op::SHIFT_RIGHT_LOGICAL => ("OpShiftRightLogical", BINARY),
        op::BITWISE_AND => ("OpBitwiseAnd", BINARY),
        op::DPDX => ("OpDPdx", UNARY),
        op::DPDY => ("OpDPdy", UNARY),
        op::LOOP_MERGE => ("OpLoopMerge", &[Id, Id, LoopControl]),
        op::SELECTION_MERGE => ("OpSelectionMerge", &[Id, SelControl]),
        op::LABEL => ("OpLabel", &[Res]),
        op::BRANCH => ("OpBranch", &[Id]),
        op::BRANCH_CONDITIONAL => ("OpBranchConditional", &[Id, Id, Id, Lits]),
        op::KILL => ("OpKill", &[]),
        op::RETURN => ("OpReturn", &[]),
        op::UNREACHABLE => ("OpUnreachable", &[]),
        _ => return None,
    })
}

/// A scalar type, for printing constant values.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Scalar {
    Float32,
    Int32 { signed: bool },
}

/// State of one dump.
struct Dumper {
    options: TextOptions,
    /// Unique friendly names by id.
    names: HashMap<u32, String>,
    /// 32-bit scalar types seen so far, by id.
    scalar_types: HashMap<u32, Scalar>,
    /// Id of the `GLSL.std.450` import, once seen.
    glsl_set: Option<u32>,
    out: SpirvText,
}

impl Dumper {
    fn fail(&mut self, offset: usize, message: impl Into<String>) {
        let message = message.into();
        let _ = writeln!(self.out.text, "; error at word {offset}: {message}");
        self.out.error = Some(ValidationError { offset, message });
    }

    fn id(&self, id: u32) -> String {
        match self.names.get(&id) {
            Some(name) => format!("%{name}"),
            None => format!("%{id}"),
        }
    }

    fn emit(&mut self, offset: usize, result: Option<&str>, body: &str) {
        if self.options.offsets {
            let _ = write!(self.out.text, "{offset:>6} ");
        }
        match result {
            Some(r) => {
                let _ = write!(self.out.text, "{r:>RESULT_COLUMN$} = ");
            }
            None => self.out.text.push_str(&" ".repeat(RESULT_COLUMN + 3)),
        }
        self.out.text.push_str(body);
        self.out.text.push('\n');
        self.out.instructions += 1;
    }

    #[allow(clippy::too_many_lines)]
    fn instruction(&mut self, offset: usize, opcode: u16, ops: &[u32]) {
        let Some((name, kinds)) = grammar(opcode) else {
            let mut body = format!("Op<{opcode}>");
            for w in ops {
                let _ = write!(body, " 0x{w:08x}");
            }
            self.out.unknown.push((offset, opcode));
            self.emit(offset, None, &body);
            return;
        };
        let mut parts: Vec<String> = Vec::new();
        let mut result = None;
        let mut result_type = None;
        let mut last_id = None;
        let mut i = 0usize;
        let mut complete = true;
        for &k in kinds {
            let rest = &ops[i.min(ops.len())..];
            match k {
                K::Ids => {
                    parts.extend(rest.iter().map(|&x| self.id(x)));
                    i = ops.len();
                }
                K::Lits => {
                    parts.extend(rest.iter().map(u32::to_string));
                    i = ops.len();
                }
                K::OptId => {
                    if let Some(&x) = rest.first() {
                        parts.push(self.id(x));
                        i += 1;
                    }
                }
                K::Str => {
                    let Some((s, n)) = read_string(rest) else {
                        complete = false;
                        break;
                    };
                    parts.push(quoted(&s));
                    i += n;
                }
                K::Value => {
                    if rest.is_empty() {
                        complete = false;
                        break;
                    }
                    parts.push(self.constant_value(result_type, rest));
                    i = ops.len();
                }
                K::ImageOps => {
                    if let Some((&mask, ids)) = rest.split_first() {
                        parts.push(image_operand_mask(mask));
                        parts.extend(ids.iter().map(|&x| self.id(x)));
                        if let Some(expected) = image_operand_count(mask)
                            && expected != ids.len()
                        {
                            self.out.mismatched.push(offset);
                        }
                        i = ops.len();
                    }
                }
                K::ExecMode | K::LoopControl | K::Decoration => {
                    let Some((&first, tail)) = rest.split_first() else {
                        complete = false;
                        break;
                    };
                    parts.push(match k {
                        K::ExecMode => name_or_number(execution_mode_name(first), first),
                        K::LoopControl => control_mask(first),
                        _ => name_or_number(decoration_name(first), first),
                    });
                    if k == K::Decoration && first == decoration::BUILT_IN {
                        let Some((&b, more)) = tail.split_first() else {
                            complete = false;
                            break;
                        };
                        parts.push(name_or_number(builtin_name(b), b));
                        parts.extend(more.iter().map(u32::to_string));
                    } else {
                        parts.extend(tail.iter().map(u32::to_string));
                    }
                    i = ops.len();
                }
                _ => {
                    let Some(&x) = rest.first() else {
                        complete = false;
                        break;
                    };
                    i += 1;
                    match k {
                        K::Ty => {
                            result_type = Some(x);
                            parts.push(self.id(x));
                        }
                        K::Res => result = Some(x),
                        K::Id => {
                            last_id = Some(x);
                            parts.push(self.id(x));
                        }
                        K::Lit => parts.push(x.to_string()),
                        K::Capability => parts.push(name_or_number(capability_name(x), x)),
                        K::Addressing => parts.push(name_or_number(addressing_name(x), x)),
                        K::Memory => parts.push(name_or_number(memory_model_name(x), x)),
                        K::ExecModel => parts.push(name_or_number(execution_model_name(x), x)),
                        K::Storage => parts.push(name_or_number(storage_name(x), x)),
                        K::Dim => parts.push(name_or_number(dim_name(x), x)),
                        K::Format => parts.push(if x == IMAGE_FORMAT_UNKNOWN {
                            "Unknown".to_string()
                        } else {
                            x.to_string()
                        }),
                        K::FnControl | K::SelControl => parts.push(control_mask(x)),
                        K::ExtInst => parts.push(match (last_id, glsl_name(x)) {
                            (Some(set), Some(n)) if Some(set) == self.glsl_set => n.to_string(),
                            _ => x.to_string(),
                        }),
                        _ => {}
                    }
                }
            }
        }
        if !complete {
            parts.push("<missing operand>".to_string());
            self.out.mismatched.push(offset);
        } else if i < ops.len() {
            parts.push("; extra".to_string());
            parts.extend(ops[i..].iter().map(|w| format!("0x{w:08x}")));
            self.out.mismatched.push(offset);
        }
        self.record(opcode, ops);
        let mut body = name.to_string();
        for p in &parts {
            body.push(' ');
            body.push_str(p);
        }
        let result = result.map(|r| self.id(r));
        self.emit(offset, result.as_deref(), &body);
    }

    /// Remember what later instructions need: scalar types for constant
    /// values and the `GLSL.std.450` import for instruction names.
    fn record(&mut self, opcode: u16, ops: &[u32]) {
        match opcode {
            op::TYPE_FLOAT if ops.get(1) == Some(&32) => {
                self.scalar_types.insert(ops[0], Scalar::Float32);
            }
            op::TYPE_INT if ops.len() >= 3 && ops[1] == 32 => {
                self.scalar_types.insert(
                    ops[0],
                    Scalar::Int32 {
                        signed: ops[2] != 0,
                    },
                );
            }
            op::EXT_INST_IMPORT
                if !ops.is_empty()
                    && read_string(&ops[1..]).is_some_and(|(s, _)| s == GLSL_SET_NAME) =>
            {
                self.glsl_set = Some(ops[0]);
            }
            _ => {}
        }
    }

    fn constant_value(&self, result_type: Option<u32>, words: &[u32]) -> String {
        let scalar = result_type.and_then(|t| self.scalar_types.get(&t)).copied();
        match (scalar, words) {
            (Some(Scalar::Float32), &[bits]) => float_text(bits),
            (Some(Scalar::Int32 { signed: true }), &[bits]) => bits.cast_signed().to_string(),
            _ => words
                .iter()
                .map(u32::to_string)
                .collect::<Vec<_>>()
                .join(" "),
        }
    }
}

/// Unique, identifier-like `OpName`s by id. Reads only what it can split.
fn friendly_names(words: &[u32]) -> HashMap<u32, String> {
    let mut named: HashMap<u32, String> = HashMap::new();
    let mut pos = HEADER_WORDS;
    while pos < words.len() {
        let count = (words[pos] >> 16) as usize;
        if count == 0 || pos + count > words.len() {
            break;
        }
        if (words[pos] & 0xFFFF) as u16 == op::NAME
            && let Some((&id, rest)) = words[pos + 1..pos + count].split_first()
            && let Some((name, _)) = read_string(rest)
            && is_identifier(&name)
        {
            named.entry(id).or_insert(name);
        }
        pos += count;
    }
    let mut uses: HashMap<String, usize> = HashMap::new();
    for name in named.values() {
        *uses.entry(name.clone()).or_default() += 1;
    }
    named.retain(|_, name| uses[name.as_str()] == 1);
    named
}

/// Letters, digits and `_`, not empty, not starting with a digit.
fn is_identifier(s: &str) -> bool {
    s.chars().next().is_some_and(|c| !c.is_ascii_digit())
        && s.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
}

fn quoted(s: &str) -> String {
    let mut out = String::from("\"");
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

/// Shortest round-trip decimal, or a NaN with its bits.
fn float_text(bits: u32) -> String {
    let v = f32::from_bits(bits);
    if v.is_nan() {
        format!("nan(0x{bits:08x})")
    } else {
        format!("{v:?}")
    }
}

fn name_or_number(name: Option<&'static str>, value: u32) -> String {
    name.map_or_else(|| value.to_string(), str::to_string)
}

/// `None` for an empty control mask, the mask in hexadecimal otherwise
/// (this crate never sets a control bit).
fn control_mask(mask: u32) -> String {
    if mask == 0 {
        "None".to_string()
    } else {
        format!("0x{mask:x}")
    }
}

/// `Bias`, `Lod`, `Grad` joined with `|`; unnamed bits as hexadecimal.
fn image_operand_mask(mask: u32) -> String {
    if mask == 0 {
        return "None".to_string();
    }
    let named = [
        (image_operands::BIAS, "Bias"),
        (image_operands::LOD, "Lod"),
        (image_operands::GRAD, "Grad"),
    ];
    let mut parts: Vec<String> = named
        .iter()
        .filter(|(bit, _)| mask & bit != 0)
        .map(|(_, n)| (*n).to_string())
        .collect();
    let known = image_operands::BIAS | image_operands::LOD | image_operands::GRAD;
    if mask & !known != 0 {
        parts.push(format!("0x{:x}", mask & !known));
    }
    parts.join("|")
}

/// Ids the image operand mask calls for, when every bit is one this
/// crate knows (`Grad` takes two).
fn image_operand_count(mask: u32) -> Option<usize> {
    let known = image_operands::BIAS | image_operands::LOD | image_operands::GRAD;
    (mask & !known == 0).then(|| {
        usize::from(mask & image_operands::BIAS != 0)
            + usize::from(mask & image_operands::LOD != 0)
            + 2 * usize::from(mask & image_operands::GRAD != 0)
    })
}

fn capability_name(v: u32) -> Option<&'static str> {
    (v == capability::SHADER).then_some("Shader")
}

fn addressing_name(v: u32) -> Option<&'static str> {
    (v == spirv::ADDRESSING_LOGICAL).then_some("Logical")
}

fn memory_model_name(v: u32) -> Option<&'static str> {
    (v == spirv::MEMORY_GLSL450).then_some("GLSL450")
}

fn execution_model_name(v: u32) -> Option<&'static str> {
    match v {
        model::VERTEX => Some("Vertex"),
        model::FRAGMENT => Some("Fragment"),
        _ => None,
    }
}

fn execution_mode_name(v: u32) -> Option<&'static str> {
    match v {
        mode::ORIGIN_UPPER_LEFT => Some("OriginUpperLeft"),
        mode::DEPTH_REPLACING => Some("DepthReplacing"),
        _ => None,
    }
}

fn storage_name(v: u32) -> Option<&'static str> {
    Some(match v {
        storage::UNIFORM_CONSTANT => "UniformConstant",
        storage::INPUT => "Input",
        storage::UNIFORM => "Uniform",
        storage::OUTPUT => "Output",
        storage::PRIVATE => "Private",
        storage::FUNCTION => "Function",
        storage::PUSH_CONSTANT => "PushConstant",
        _ => return None,
    })
}

fn decoration_name(v: u32) -> Option<&'static str> {
    Some(match v {
        decoration::BLOCK => "Block",
        decoration::ARRAY_STRIDE => "ArrayStride",
        decoration::BUILT_IN => "BuiltIn",
        decoration::CENTROID => "Centroid",
        decoration::LOCATION => "Location",
        decoration::BINDING => "Binding",
        decoration::DESCRIPTOR_SET => "DescriptorSet",
        decoration::OFFSET => "Offset",
        _ => return None,
    })
}

fn builtin_name(v: u32) -> Option<&'static str> {
    Some(match v {
        builtin::POSITION => "Position",
        builtin::POINT_SIZE => "PointSize",
        builtin::FRAG_COORD => "FragCoord",
        builtin::FRONT_FACING => "FrontFacing",
        builtin::FRAG_DEPTH => "FragDepth",
        _ => return None,
    })
}

fn dim_name(v: u32) -> Option<&'static str> {
    Some(match v {
        dim::D2 => "2D",
        dim::D3 => "3D",
        dim::CUBE => "Cube",
        _ => return None,
    })
}

/// A plain-text summary of a module's reflection: stage, interface with
/// usages and locations, samplers with their bindings, constant registers
/// read and defined, buffers and push constants. The numbers are the ones
/// a renderer builds its Vulkan layouts from (see [`crate::binding`]).
#[must_use]
pub fn reflection_summary(module: &Module) -> String {
    let mut out = String::new();
    let stage = match module.stage {
        Stage::Vertex => "vertex",
        Stage::Pixel => "pixel (fragment)",
    };
    let _ = writeln!(out, "stage: {stage}; {} SPIR-V words", module.words.len());
    for (title, vars) in [("inputs", &module.inputs), ("outputs", &module.outputs)] {
        if vars.is_empty() {
            let _ = writeln!(out, "{title}: none");
            continue;
        }
        let _ = writeln!(out, "{title}:");
        for v in vars {
            let _ = writeln!(out, "  {}", interface_text(v, module.stage));
        }
    }
    if module.samplers.is_empty() {
        let _ = writeln!(out, "samplers: none");
    } else {
        let _ = writeln!(out, "samplers:");
        for s in &module.samplers {
            let dim = match s.dim {
                TextureDim::D2 => "2D",
                TextureDim::Cube => "cube",
                TextureDim::D3 => "volume",
            };
            let _ = writeln!(
                out,
                "  s{}: {dim}, set {} binding {}{}",
                s.register,
                s.set,
                s.binding,
                if s.depth_compare {
                    ", depth compare"
                } else {
                    ""
                }
            );
        }
    }
    let c = &module.constants;
    let _ = writeln!(out, "constants:");
    let _ = writeln!(
        out,
        "  float read: {}; relative addressing: {}",
        register_ranges('c', &c.float_read),
        if c.float_relative { "yes" } else { "no" }
    );
    let _ = writeln!(
        out,
        "  float defined by def: {}",
        register_ranges('c', &c.float_defined)
    );
    let _ = writeln!(
        out,
        "  integer read: {}; defined by defi: {}",
        register_ranges('i', &c.int_read),
        register_ranges('i', &c.int_defined)
    );
    let _ = writeln!(
        out,
        "  boolean read: {}; defined by defb: {}",
        register_ranges('b', &c.bool_read),
        register_ranges('b', &c.bool_defined)
    );
    let buffer = |b: Option<u32>| {
        b.map_or_else(
            || "none".to_string(),
            |b| format!("set {} binding {b}", binding::CONSTANT_SET),
        )
    };
    let _ = writeln!(
        out,
        "buffers: float constants {}; integer/boolean constants {}; push constants {} bytes",
        buffer(module.float_constant_binding),
        buffer(module.int_bool_constant_binding),
        module.push_constant_size
    );
    let _ = writeln!(
        out,
        "depth replacing: {}",
        if module.depth_replacing { "yes" } else { "no" }
    );
    out
}

/// `v1 texcoord0 -> location 0, mask .xy, centroid`.
fn interface_text(v: &InterfaceVar, stage: Stage) -> String {
    let mut s = register_name(v.reg, stage);
    if let Some((u, index)) = v.usage {
        match binding::usage_name(u) {
            "unknown" => {
                let _ = write!(s, " usage{u}_{index}");
            }
            name => {
                let _ = write!(s, " {name}{index}");
            }
        }
    }
    match (v.location, v.builtin) {
        (Some(l), _) => {
            let _ = write!(s, " -> location {l}");
        }
        (None, Some(b)) => {
            let _ = write!(s, " -> built-in {b:?}");
        }
        (None, None) => s.push_str(" -> no location"),
    }
    if v.write_mask & 0xF != 0xF {
        let comps: String = ['x', 'y', 'z', 'w']
            .iter()
            .enumerate()
            .filter(|(i, _)| v.write_mask & (1 << i) != 0)
            .map(|(_, c)| c)
            .collect();
        let _ = write!(s, ", mask .{comps}");
    }
    if v.centroid {
        s.push_str(", centroid");
    }
    s
}

/// `c0-c3, c8, c10-c11`, or `none`.
fn register_ranges(prefix: char, regs: &[u16]) -> String {
    let mut parts: Vec<String> = Vec::new();
    let mut i = 0;
    while i < regs.len() {
        let start = regs[i];
        let mut end = start;
        while i + 1 < regs.len() && regs[i + 1] == end.saturating_add(1) {
            i += 1;
            end = regs[i];
        }
        parts.push(if start == end {
            format!("{prefix}{start}")
        } else {
            format!("{prefix}{start}-{prefix}{end}")
        });
        i += 1;
    }
    if parts.is_empty() {
        "none".to_string()
    } else {
        parts.join(", ")
    }
}
