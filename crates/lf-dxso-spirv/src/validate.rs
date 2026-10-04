//! A structural SPIR-V validator for the modules this crate emits.
//!
//! It is not a replacement for `spirv-val`; it knows only the opcodes the
//! translator emits (any other opcode is an error) and checks what an
//! emitter bug would most likely break:
//!
//! * header: magic, version 1.0, id bound, schema 0;
//! * instruction word counts fit the module and the operand grammar;
//! * logical layout order of sections; exactly one memory model; at least
//!   one entry point;
//! * every result id defined once and below the bound; every id operand
//!   defined textually before use (labels and functions may be forward
//!   references), function-local ids used only in their function;
//! * result types are type declarations; non-aggregate types are not
//!   declared twice;
//! * functions: each block starts with `OpLabel` and ends with exactly one
//!   terminator; `OpSelectionMerge`/`OpLoopMerge` immediately precede the
//!   right branch; every conditional branch is preceded by a merge; a
//!   block is the merge target of at most one construct; branch and merge
//!   targets are labels of the same function;
//! * dominance: inside a function, every id defined in another block is
//!   defined in a block that dominates the use; every back edge goes from
//!   a loop's continue target to its header; a header dominates its
//!   reachable merge block;
//! * operand types of loads, stores, access chains, arithmetic,
//!   comparisons, selects, shuffles, composites, conversions, the
//!   `GLSL.std.450` instructions used, image sampling and calls;
//! * entry point and execution modes (fragment needs `OriginUpperLeft`,
//!   `FragDepth` needs `DepthReplacing`; `OpKill`, derivatives and
//!   implicit-LOD sampling only in fragment shaders);
//! * Vulkan interface rules: every `Input`/`Output` variable is listed in
//!   the entry point and has exactly one of `BuiltIn` or `Location`, with
//!   unique locations; every resource variable has a unique
//!   `DescriptorSet`/`Binding`; buffer blocks are `Block` structures with
//!   member offsets and array strides.

// Single-letter bindings follow the operand layout (`o` operands, `t`
// result type, `r` result id, `n`/`k` counts and indexes).
#![allow(clippy::many_single_char_names)]

use std::collections::{HashMap, HashSet};

use crate::spirv::{self, decoration, glsl, image_operands, mode, model, op, storage};

/// One decoded instruction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Inst<'a> {
    /// Word index of the instruction in the module.
    pub offset: usize,
    /// Opcode.
    pub opcode: u16,
    /// Operand words.
    pub operands: &'a [u32],
}

/// A validation failure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationError {
    /// Word index of the offending instruction (0 for the header).
    pub offset: usize,
    /// What is wrong.
    pub message: String,
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SPIR-V word {}: {}", self.offset, self.message)
    }
}

impl std::error::Error for ValidationError {}

/// Summary of a module that passed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Report {
    /// Instructions after the header.
    pub instructions: usize,
    /// Function definitions.
    pub functions: usize,
    /// Basic blocks.
    pub blocks: usize,
    /// Id bound from the header.
    pub bound: u32,
}

fn err<T>(offset: usize, message: impl Into<String>) -> Result<T, ValidationError> {
    Err(ValidationError {
        offset,
        message: message.into(),
    })
}

/// Split a module (after its five-word header) into instructions.
///
/// # Errors
///
/// A header shorter than five words, or a word count of zero or one that
/// runs past the end.
pub fn instructions(words: &[u32]) -> Result<Vec<Inst<'_>>, ValidationError> {
    if words.len() < 5 {
        return err(0, "module shorter than its header");
    }
    let mut out = Vec::new();
    let mut pos = 5;
    while pos < words.len() {
        let w = words[pos];
        let count = (w >> 16) as usize;
        let opcode = (w & 0xFFFF) as u16;
        if count == 0 {
            return err(pos, "instruction word count is zero");
        }
        if pos + count > words.len() {
            return err(pos, "instruction runs past the end of the module");
        }
        out.push(Inst {
            offset: pos,
            opcode,
            operands: &words[pos + 1..pos + count],
        });
        pos += count;
    }
    Ok(out)
}

/// Decode a string literal at the start of `words`; returns the string and
/// the words it occupies.
#[must_use]
pub fn read_string(words: &[u32]) -> Option<(String, usize)> {
    let mut bytes = Vec::new();
    for (i, w) in words.iter().enumerate() {
        for b in w.to_le_bytes() {
            if b == 0 {
                return Some((String::from_utf8_lossy(&bytes).into_owned(), i + 1));
            }
            bytes.push(b);
        }
    }
    None
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum K {
    /// Result type.
    Ty,
    /// Result id.
    Res,
    /// Id defined before use.
    Id,
    /// Label or function id, forward reference allowed.
    Fwd,
    /// Id defined anywhere (decoration and debug targets).
    Any,
    /// One literal word.
    Lit,
    /// String literal.
    Str,
    /// Remaining words are ids defined before use.
    Ids,
    /// Remaining words are ids defined anywhere.
    AnyIds,
    /// Remaining words are literals.
    Lits,
    /// Optional id defined before use.
    OptId,
    /// Optional image operands: a mask literal, then ids.
    ImgOps,
}

fn grammar(opcode: u16) -> Option<&'static [K]> {
    use K::{Any, AnyIds, Fwd, Id, Ids, ImgOps, Lit, Lits, OptId, Res, Str, Ty};
    Some(match opcode {
        op::NAME => &[Any, Str],
        op::MEMBER_NAME => &[Any, Lit, Str],
        op::EXT_INST_IMPORT => &[Res, Str],
        op::EXT_INST => &[Ty, Res, Id, Lit, Ids],
        op::MEMORY_MODEL => &[Lit, Lit],
        op::ENTRY_POINT => &[Lit, Any, Str, AnyIds],
        op::CAPABILITY => &[Lit],
        op::TYPE_VOID | op::TYPE_BOOL | op::LABEL => &[Res],
        op::TYPE_INT => &[Res, Lit, Lit],
        op::TYPE_FLOAT => &[Res, Lit],
        op::TYPE_VECTOR => &[Res, Id, Lit],
        op::TYPE_IMAGE => &[Res, Id, Lit, Lit, Lit, Lit, Lit, Lit, Lits],
        op::TYPE_SAMPLED_IMAGE => &[Res, Id],
        op::TYPE_ARRAY => &[Res, Id, Id],
        op::TYPE_STRUCT => &[Res, Ids],
        op::TYPE_POINTER => &[Res, Lit, Id],
        op::TYPE_FUNCTION => &[Res, Id, Ids],
        op::CONSTANT_TRUE | op::CONSTANT_FALSE => &[Ty, Res],
        op::CONSTANT => &[Ty, Res, Lits],
        op::CONSTANT_COMPOSITE | op::COMPOSITE_CONSTRUCT => &[Ty, Res, Ids],
        op::FUNCTION => &[Ty, Res, Lit, Id],
        op::FUNCTION_END | op::KILL | op::RETURN | op::UNREACHABLE => &[],
        op::FUNCTION_CALL => &[Ty, Res, Fwd, Ids],
        op::VARIABLE => &[Ty, Res, Lit, OptId],
        op::LOAD | op::COMPOSITE_EXTRACT => &[Ty, Res, Id, Lits],
        op::STORE => &[Id, Id, Lits],
        op::ACCESS_CHAIN => &[Ty, Res, Id, Ids],
        op::DECORATE | op::EXECUTION_MODE => &[Any, Lit, Lits],
        op::MEMBER_DECORATE => &[Any, Lit, Lit, Lits],
        op::VECTOR_SHUFFLE => &[Ty, Res, Id, Id, Lits],
        op::IMAGE_SAMPLE_IMPLICIT_LOD | op::IMAGE_SAMPLE_EXPLICIT_LOD => &[Ty, Res, Id, Id, ImgOps],
        op::IMAGE_SAMPLE_DREF_IMPLICIT_LOD | op::IMAGE_SAMPLE_DREF_EXPLICIT_LOD => {
            &[Ty, Res, Id, Id, Id, ImgOps]
        }
        op::CONVERT_F_TO_S | op::F_NEGATE | op::DPDX | op::DPDY | op::ANY | op::LOGICAL_NOT => {
            &[Ty, Res, Id]
        }
        op::I_ADD
        | op::F_ADD
        | op::F_SUB
        | op::F_MUL
        | op::F_DIV
        | op::DOT
        | op::LOGICAL_OR
        | op::LOGICAL_AND
        | op::I_EQUAL
        | op::I_NOT_EQUAL
        | op::S_GREATER_THAN_EQUAL
        | op::S_LESS_THAN
        | op::F_ORD_EQUAL
        | op::F_UNORD_NOT_EQUAL
        | op::F_ORD_LESS_THAN
        | op::F_ORD_GREATER_THAN
        | op::F_ORD_LESS_THAN_EQUAL
        | op::F_ORD_GREATER_THAN_EQUAL
        | op::SHIFT_RIGHT_LOGICAL
        | op::BITWISE_AND => &[Ty, Res, Id, Id],
        op::SELECT => &[Ty, Res, Id, Id, Id],
        op::LOOP_MERGE => &[Fwd, Fwd, Lit, Lits],
        op::SELECTION_MERGE => &[Fwd, Lit],
        op::BRANCH => &[Fwd],
        op::BRANCH_CONDITIONAL => &[Id, Fwd, Fwd, Lits],
        _ => return None,
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Use {
    Before,
    Forward,
    Anywhere,
}

#[derive(Debug, Default)]
struct Parsed {
    ty: Option<u32>,
    res: Option<u32>,
    uses: Vec<(u32, Use)>,
}

fn parse_operands(inst: &Inst<'_>) -> Result<Parsed, ValidationError> {
    let Some(g) = grammar(inst.opcode) else {
        return err(
            inst.offset,
            format!("opcode {} is not one this crate emits", inst.opcode),
        );
    };
    let ops = inst.operands;
    let mut i = 0usize;
    let mut p = Parsed::default();
    let need = |i: usize| -> Result<u32, ValidationError> {
        ops.get(i).copied().ok_or(ValidationError {
            offset: inst.offset,
            message: format!("opcode {}: too few operands", inst.opcode),
        })
    };
    for &k in g {
        match k {
            K::Ty => {
                p.ty = Some(need(i)?);
                i += 1;
            }
            K::Res => {
                p.res = Some(need(i)?);
                i += 1;
            }
            K::Id => {
                p.uses.push((need(i)?, Use::Before));
                i += 1;
            }
            K::Fwd => {
                p.uses.push((need(i)?, Use::Forward));
                i += 1;
            }
            K::Any => {
                p.uses.push((need(i)?, Use::Anywhere));
                i += 1;
            }
            K::Lit => {
                need(i)?;
                i += 1;
            }
            K::Str => {
                let Some((_, n)) = read_string(&ops[i.min(ops.len())..]) else {
                    return err(inst.offset, "unterminated string literal");
                };
                i += n;
            }
            K::Ids => {
                p.uses
                    .extend(ops[i.min(ops.len())..].iter().map(|&x| (x, Use::Before)));
                i = ops.len();
            }
            K::AnyIds => {
                p.uses
                    .extend(ops[i.min(ops.len())..].iter().map(|&x| (x, Use::Anywhere)));
                i = ops.len();
            }
            K::Lits => i = ops.len(),
            K::OptId => {
                if let Some(&x) = ops.get(i) {
                    p.uses.push((x, Use::Before));
                    i += 1;
                }
            }
            K::ImgOps => {
                if let Some(&mask) = ops.get(i) {
                    let expected = (mask & image_operands::BIAS).count_ones()
                        + (mask & image_operands::LOD).count_ones()
                        + 2 * (mask & image_operands::GRAD).count_ones();
                    let rest = &ops[i + 1..];
                    if mask & !(image_operands::BIAS | image_operands::LOD | image_operands::GRAD)
                        != 0
                    {
                        return err(inst.offset, "image operand bit this crate does not emit");
                    }
                    if rest.len() != expected as usize {
                        return err(inst.offset, "image operand count does not match its mask");
                    }
                    p.uses.extend(rest.iter().map(|&x| (x, Use::Before)));
                    i = ops.len();
                }
            }
        }
    }
    if i != ops.len() {
        return err(
            inst.offset,
            format!("opcode {}: extra operand words", inst.opcode),
        );
    }
    Ok(p)
}

fn section(opcode: u16) -> u8 {
    match opcode {
        op::CAPABILITY => 0,
        op::EXT_INST_IMPORT => 2,
        op::MEMORY_MODEL => 3,
        op::ENTRY_POINT => 4,
        op::EXECUTION_MODE => 5,
        op::NAME | op::MEMBER_NAME => 6,
        op::DECORATE | op::MEMBER_DECORATE => 7,
        op::TYPE_VOID
        | op::TYPE_BOOL
        | op::TYPE_INT
        | op::TYPE_FLOAT
        | op::TYPE_VECTOR
        | op::TYPE_IMAGE
        | op::TYPE_SAMPLED_IMAGE
        | op::TYPE_ARRAY
        | op::TYPE_STRUCT
        | op::TYPE_POINTER
        | op::TYPE_FUNCTION
        | op::CONSTANT_TRUE
        | op::CONSTANT_FALSE
        | op::CONSTANT
        | op::CONSTANT_COMPOSITE => 8,
        _ => 9,
    }
}

fn is_terminator(opcode: u16) -> bool {
    matches!(
        opcode,
        op::BRANCH | op::BRANCH_CONDITIONAL | op::KILL | op::RETURN | op::UNREACHABLE
    )
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Type {
    Void,
    Bool,
    Int(bool),
    Float,
    Vector(u32, u32),
    Array(u32, u32),
    Struct(Vec<u32>),
    Pointer(u32, u32),
    Function(u32, Vec<u32>),
    Image(u32, u32, u32),
    SampledImage(u32),
}

#[derive(Default)]
struct State {
    defs: HashMap<u32, usize>,
    /// Function index (into the order of `OpFunction`s) of function-local ids.
    local: HashMap<u32, usize>,
    types: HashMap<u32, Type>,
    value_type: HashMap<u32, u32>,
    constants: HashMap<u32, u32>,
    constant_ids: HashSet<u32>,
    functions: HashMap<u32, u32>,
    labels: HashMap<u32, usize>,
    variables: HashMap<u32, u32>,
    decorations: HashMap<u32, Vec<(u32, Vec<u32>)>>,
    member_decorations: HashMap<(u32, u32), Vec<u32>>,
    ext_import: Option<u32>,
}

impl State {
    fn ty(&self, id: u32, at: usize) -> Result<&Type, ValidationError> {
        self.types.get(&id).ok_or(ValidationError {
            offset: at,
            message: format!("%{id} is not a type"),
        })
    }

    fn type_of(&self, id: u32, at: usize) -> Result<u32, ValidationError> {
        self.value_type.get(&id).copied().ok_or(ValidationError {
            offset: at,
            message: format!("%{id} has no value type"),
        })
    }

    /// `(scalar type id, component count)` of a scalar or vector type.
    fn shape(&self, ty: u32) -> Option<(u32, u32)> {
        match self.types.get(&ty)? {
            Type::Bool | Type::Int(_) | Type::Float => Some((ty, 1)),
            Type::Vector(c, n) => Some((*c, *n)),
            _ => None,
        }
    }

    fn is_float_type(&self, ty: u32) -> bool {
        self.shape(ty)
            .is_some_and(|(c, _)| self.types.get(&c) == Some(&Type::Float))
    }

    fn is_int_type(&self, ty: u32) -> bool {
        self.shape(ty)
            .is_some_and(|(c, _)| matches!(self.types.get(&c), Some(Type::Int(_))))
    }

    fn is_bool_type(&self, ty: u32) -> bool {
        self.shape(ty)
            .is_some_and(|(c, _)| self.types.get(&c) == Some(&Type::Bool))
    }

    fn has_decoration(&self, id: u32, d: u32) -> Option<&Vec<u32>> {
        self.decorations
            .get(&id)?
            .iter()
            .find(|(k, _)| *k == d)
            .map(|(_, v)| v)
    }
}

/// Validate a module.
///
/// # Errors
///
/// The first structural problem found.
#[allow(clippy::too_many_lines)]
pub fn validate(words: &[u32]) -> Result<Report, ValidationError> {
    if words.len() < 5 {
        return err(0, "module shorter than its header");
    }
    if words[0] != spirv::MAGIC {
        return err(0, "bad magic number");
    }
    if words[1] != spirv::VERSION_1_0 {
        return err(1, "version is not 1.0");
    }
    let bound = words[3];
    if bound == 0 {
        return err(3, "id bound is zero");
    }
    if words[4] != 0 {
        return err(4, "schema word is not zero");
    }
    let insts = instructions(words)?;
    let mut st = State::default();
    let parsed: Vec<Parsed> = insts.iter().map(parse_operands).collect::<Result<_, _>>()?;

    // Layout order and section membership.
    let mut last_section = 0u8;
    let mut memory_models = 0;
    let mut entry_points = 0;
    for inst in &insts {
        let s = section(inst.opcode);
        let s = if inst.opcode == op::VARIABLE && last_section < 9 {
            8
        } else {
            s
        };
        if s < last_section {
            return err(inst.offset, "instruction out of logical layout order");
        }
        last_section = s;
        if inst.opcode == op::MEMORY_MODEL {
            memory_models += 1;
        }
        if inst.opcode == op::ENTRY_POINT {
            entry_points += 1;
        }
    }
    if memory_models != 1 {
        return err(5, "module needs exactly one OpMemoryModel");
    }
    if entry_points == 0 {
        return err(5, "module has no entry point");
    }

    // Definitions (all of them first, for forward-reference checks).
    let mut function_index = None::<usize>;
    let mut function_count = 0usize;
    for (n, (inst, p)) in insts.iter().zip(&parsed).enumerate() {
        if inst.opcode == op::FUNCTION {
            function_index = Some(function_count);
            function_count += 1;
        }
        if let Some(r) = p.res {
            if r == 0 || r >= bound {
                return err(
                    inst.offset,
                    format!("result id %{r} outside the bound {bound}"),
                );
            }
            if st.defs.insert(r, n).is_some() {
                return err(inst.offset, format!("id %{r} defined twice"));
            }
            if let Some(f) = function_index
                && inst.opcode != op::FUNCTION
            {
                st.local.insert(r, f);
            }
            match inst.opcode {
                op::LABEL => {
                    st.labels.insert(r, function_index.unwrap_or(usize::MAX));
                }
                op::FUNCTION => {
                    st.functions.insert(r, inst.operands[3]);
                }
                op::EXT_INST_IMPORT => st.ext_import = Some(r),
                _ => {}
            }
        }
        if inst.opcode == op::FUNCTION_END {
            function_index = None;
        }
    }

    // Uses.
    let mut function_index = None::<usize>;
    let mut function_count = 0usize;
    for (n, (inst, p)) in insts.iter().zip(&parsed).enumerate() {
        if inst.opcode == op::FUNCTION {
            function_index = Some(function_count);
            function_count += 1;
        }
        let ids =
            p.ty.iter()
                .map(|&t| (t, Use::Before))
                .chain(p.uses.iter().copied());
        for (id, kind) in ids {
            let Some(&def) = st.defs.get(&id) else {
                return err(inst.offset, format!("id %{id} is never defined"));
            };
            if kind == Use::Before && def >= n {
                return err(inst.offset, format!("id %{id} used before its definition"));
            }
            if kind == Use::Forward
                && !st.labels.contains_key(&id)
                && !st.functions.contains_key(&id)
            {
                return err(inst.offset, format!("%{id} is not a label or function"));
            }
            if kind != Use::Anywhere
                && let Some(&owner) = st.local.get(&id)
                && Some(owner) != function_index
            {
                return err(inst.offset, format!("%{id} used outside its function"));
            }
        }
        if inst.opcode == op::FUNCTION_END {
            function_index = None;
        }
    }

    // Types, constants, decorations and value types, in order.
    let mut seen_types: HashSet<Type> = HashSet::new();
    for (inst, p) in insts.iter().zip(&parsed) {
        let o = inst.operands;
        let at = inst.offset;
        if let Some(t) = p.ty {
            st.ty(t, at)?;
        }
        let new_type = match inst.opcode {
            op::TYPE_VOID => Some(Type::Void),
            op::TYPE_BOOL => Some(Type::Bool),
            op::TYPE_INT => {
                if o[1] != 32 || o[2] > 1 {
                    return err(at, "only 32-bit integers are expected");
                }
                Some(Type::Int(o[2] == 1))
            }
            op::TYPE_FLOAT => {
                if o[1] != 32 {
                    return err(at, "only 32-bit floats are expected");
                }
                Some(Type::Float)
            }
            op::TYPE_VECTOR => {
                if !matches!(st.ty(o[1], at)?, Type::Bool | Type::Int(_) | Type::Float) {
                    return err(at, "vector of a non-scalar type");
                }
                if !(2..=4).contains(&o[2]) {
                    return err(at, "vector size outside 2..4");
                }
                Some(Type::Vector(o[1], o[2]))
            }
            op::TYPE_ARRAY => {
                st.ty(o[1], at)?;
                let Some(&len) = st.constants.get(&o[2]) else {
                    return err(at, "array length is not an integer constant");
                };
                if len == 0 {
                    return err(at, "array length zero");
                }
                Some(Type::Array(o[1], len))
            }
            op::TYPE_STRUCT => {
                for &m in &o[1..] {
                    st.ty(m, at)?;
                }
                Some(Type::Struct(o[1..].to_vec()))
            }
            op::TYPE_POINTER => {
                st.ty(o[2], at)?;
                Some(Type::Pointer(o[1], o[2]))
            }
            op::TYPE_FUNCTION => {
                st.ty(o[1], at)?;
                for &m in &o[2..] {
                    st.ty(m, at)?;
                }
                Some(Type::Function(o[1], o[2..].to_vec()))
            }
            op::TYPE_IMAGE => {
                if st.ty(o[1], at)? != &Type::Float {
                    return err(at, "image sampled type is not float");
                }
                if o[6] != 1 {
                    return err(at, "image is not a sampled image");
                }
                Some(Type::Image(o[2], o[3], o[1]))
            }
            op::TYPE_SAMPLED_IMAGE => {
                if !matches!(st.ty(o[1], at)?, Type::Image(..)) {
                    return err(at, "sampled image of a non-image type");
                }
                Some(Type::SampledImage(o[1]))
            }
            _ => None,
        };
        if let Some(t) = new_type {
            let aggregate = matches!(t, Type::Array(..) | Type::Struct(_));
            if !aggregate && !seen_types.insert(t.clone()) {
                return err(at, "non-aggregate type declared twice");
            }
            st.types.insert(p.res.unwrap_or(0), t);
            continue;
        }
        match inst.opcode {
            op::DECORATE => {
                st.decorations
                    .entry(o[0])
                    .or_default()
                    .push((o[1], o[2..].to_vec()));
            }
            op::MEMBER_DECORATE => {
                st.member_decorations
                    .entry((o[0], o[1]))
                    .or_default()
                    .push(o[2]);
            }
            _ => {}
        }
        if let (Some(t), Some(r)) = (p.ty, p.res) {
            st.value_type.insert(r, t);
        }
        if let (Some(t), Some(r)) = (p.ty, p.res) {
            check_types(&mut st, inst, t, r)?;
        } else if inst.opcode == op::STORE {
            check_store(&st, inst)?;
        }
    }

    check_functions(&st, &insts)?;
    check_dominance(&st, &insts, &parsed)?;
    let (blocks, model_id) = check_entry_and_interface(&st, &insts)?;
    for inst in &insts {
        let fragment_only = matches!(
            inst.opcode,
            op::KILL
                | op::DPDX
                | op::DPDY
                | op::IMAGE_SAMPLE_IMPLICIT_LOD
                | op::IMAGE_SAMPLE_DREF_IMPLICIT_LOD
        );
        if fragment_only && model_id != model::FRAGMENT {
            return err(
                inst.offset,
                "fragment-only instruction in a non-fragment shader",
            );
        }
    }
    Ok(Report {
        instructions: insts.len(),
        functions: function_count,
        blocks,
        bound,
    })
}

fn check_store(st: &State, inst: &Inst<'_>) -> Result<(), ValidationError> {
    let at = inst.offset;
    let ptr_ty = st.type_of(inst.operands[0], at)?;
    let Type::Pointer(class, pointee) = st.ty(ptr_ty, at)? else {
        return err(at, "store through a non-pointer");
    };
    if matches!(
        *class,
        storage::INPUT | storage::UNIFORM | storage::UNIFORM_CONSTANT | storage::PUSH_CONSTANT
    ) {
        return err(at, "store to a read-only storage class");
    }
    if st.type_of(inst.operands[1], at)? != *pointee {
        return err(at, "stored value type differs from the pointee type");
    }
    Ok(())
}

#[allow(clippy::too_many_lines)]
fn check_types(st: &mut State, inst: &Inst<'_>, t: u32, r: u32) -> Result<(), ValidationError> {
    let o = inst.operands;
    let at = inst.offset;
    let same = |st: &State, ids: &[u32]| -> Result<(), ValidationError> {
        for &x in ids {
            if st.type_of(x, at)? != t {
                return err(
                    at,
                    format!("opcode {}: operand type differs from result", inst.opcode),
                );
            }
        }
        Ok(())
    };
    match inst.opcode {
        op::CONSTANT => {
            if o.len() != 3 || !matches!(st.ty(t, at)?, Type::Int(_) | Type::Float) {
                return err(at, "constant is not one 32-bit scalar word");
            }
            if matches!(st.ty(t, at)?, Type::Int(_)) {
                st.constants.insert(r, o[2]);
            }
            st.constant_ids.insert(r);
        }
        op::CONSTANT_TRUE | op::CONSTANT_FALSE => {
            if st.ty(t, at)? != &Type::Bool {
                return err(at, "boolean constant of a non-boolean type");
            }
            st.constant_ids.insert(r);
        }
        op::CONSTANT_COMPOSITE | op::COMPOSITE_CONSTRUCT => {
            let parts = &o[2..];
            if inst.opcode == op::CONSTANT_COMPOSITE {
                if let Some(x) = parts.iter().find(|x| !st.constant_ids.contains(x)) {
                    return err(
                        at,
                        format!("constant composite part %{x} is not a constant"),
                    );
                }
                st.constant_ids.insert(r);
            }
            match st.ty(t, at)?.clone() {
                Type::Vector(c, n) => {
                    let mut total = 0;
                    for &x in parts {
                        let pt = st.type_of(x, at)?;
                        let Some((pc, pn)) = st.shape(pt) else {
                            return err(at, "vector built from a non-scalar, non-vector");
                        };
                        if pc != c {
                            return err(at, "vector built from the wrong component type");
                        }
                        total += pn;
                    }
                    if total != n {
                        return err(at, "vector built from the wrong number of components");
                    }
                }
                Type::Array(e, n) => {
                    if parts.len() != n as usize {
                        return err(at, "array built from the wrong number of elements");
                    }
                    for &x in parts {
                        if st.type_of(x, at)? != e {
                            return err(at, "array element of the wrong type");
                        }
                    }
                }
                Type::Struct(m) => {
                    if parts.len() != m.len() {
                        return err(at, "structure built from the wrong number of members");
                    }
                    for (&x, &mt) in parts.iter().zip(&m) {
                        if st.type_of(x, at)? != mt {
                            return err(at, "structure member of the wrong type");
                        }
                    }
                }
                _ => return err(at, "composite of a non-composite type"),
            }
        }
        op::VARIABLE => {
            let Type::Pointer(class, pointee) = st.ty(t, at)?.clone() else {
                return err(at, "variable of a non-pointer type");
            };
            if o[2] != class {
                return err(at, "variable storage class differs from its pointer type");
            }
            if let Some(&init) = o.get(3) {
                if !st.constant_ids.contains(&init) {
                    return err(at, "variable initializer is not a constant");
                }
                if st.type_of(init, at)? != pointee {
                    return err(at, "variable initializer of the wrong type");
                }
            }
            st.variables.insert(r, class);
        }
        op::FUNCTION => {
            let Type::Function(ret, _) = st.ty(o[3], at)?.clone() else {
                return err(at, "function type operand is not a function type");
            };
            if ret != t {
                return err(at, "function result type differs from its function type");
            }
        }
        op::FUNCTION_CALL => {
            let Some(&fty) = st.functions.get(&o[2]) else {
                return err(at, "call target is not a function");
            };
            let Type::Function(ret, params) = st.ty(fty, at)?.clone() else {
                return err(at, "callee has no function type");
            };
            if ret != t || params.len() != o.len() - 3 {
                return err(at, "call result or argument count differs from the callee");
            }
        }
        op::LOAD => {
            let pt = st.type_of(o[2], at)?;
            match st.ty(pt, at)? {
                Type::Pointer(_, pointee) if *pointee == t => {}
                _ => return err(at, "load result type differs from the pointee type"),
            }
        }
        op::ACCESS_CHAIN => {
            let pt = st.type_of(o[2], at)?;
            let Type::Pointer(class, mut cur) = st.ty(pt, at)?.clone() else {
                return err(at, "access chain base is not a pointer");
            };
            for &idx in &o[3..] {
                let it = st.type_of(idx, at)?;
                if !st.is_int_type(it) || st.shape(it).map(|s| s.1) != Some(1) {
                    return err(at, "access chain index is not a scalar integer");
                }
                cur = match st.ty(cur, at)?.clone() {
                    Type::Struct(m) => {
                        let Some(&k) = st.constants.get(&idx) else {
                            return err(at, "structure index is not a constant");
                        };
                        *m.get(k as usize).ok_or(ValidationError {
                            offset: at,
                            message: "structure index out of range".into(),
                        })?
                    }
                    Type::Array(e, n) => {
                        if let Some(&k) = st.constants.get(&idx)
                            && k >= n
                        {
                            return err(at, "constant array index out of range");
                        }
                        e
                    }
                    Type::Vector(c, _) => c,
                    _ => return err(at, "access chain into a non-composite"),
                };
            }
            match st.ty(t, at)? {
                Type::Pointer(c, p) if *c == class && *p == cur => {}
                _ => return err(at, "access chain result type does not match the walk"),
            }
        }
        op::F_ADD | op::F_SUB | op::F_MUL | op::F_DIV | op::F_NEGATE | op::DPDX | op::DPDY => {
            if !st.is_float_type(t) {
                return err(at, "float arithmetic with a non-float result");
            }
            same(st, &o[2..])?;
        }
        op::I_ADD | op::SHIFT_RIGHT_LOGICAL | op::BITWISE_AND => {
            if !st.is_int_type(t) {
                return err(at, "integer arithmetic with a non-integer result");
            }
            same(st, &o[2..])?;
        }
        op::LOGICAL_AND | op::LOGICAL_OR | op::LOGICAL_NOT => {
            if !st.is_bool_type(t) {
                return err(at, "logical operation with a non-boolean result");
            }
            same(st, &o[2..])?;
        }
        op::I_EQUAL
        | op::I_NOT_EQUAL
        | op::S_GREATER_THAN_EQUAL
        | op::S_LESS_THAN
        | op::F_ORD_EQUAL
        | op::F_UNORD_NOT_EQUAL
        | op::F_ORD_LESS_THAN
        | op::F_ORD_GREATER_THAN
        | op::F_ORD_LESS_THAN_EQUAL
        | op::F_ORD_GREATER_THAN_EQUAL => {
            let a = st.type_of(o[2], at)?;
            let b = st.type_of(o[3], at)?;
            let float_cmp = inst.opcode >= op::F_ORD_EQUAL;
            let ok_operand = if float_cmp {
                st.is_float_type(a)
            } else {
                st.is_int_type(a)
            };
            if a != b || !ok_operand {
                return err(at, "comparison operands of different or wrong types");
            }
            if !st.is_bool_type(t) || st.shape(t).map(|s| s.1) != st.shape(a).map(|s| s.1) {
                return err(at, "comparison result is not a matching boolean");
            }
        }
        op::ANY => {
            let a = st.type_of(o[2], at)?;
            if st.ty(t, at)? != &Type::Bool || !st.is_bool_type(a) {
                return err(at, "OpAny with wrong types");
            }
        }
        op::SELECT => {
            let c = st.type_of(o[2], at)?;
            if !st.is_bool_type(c) || st.shape(c).map(|s| s.1) != st.shape(t).map(|s| s.1) {
                return err(
                    at,
                    "select condition does not match the result's component count",
                );
            }
            same(st, &o[3..])?;
        }
        op::DOT => {
            let a = st.type_of(o[2], at)?;
            let b = st.type_of(o[3], at)?;
            match st.ty(a, at)? {
                Type::Vector(c, _) if *c == t && a == b && st.is_float_type(t) => {}
                _ => return err(at, "dot product with wrong types"),
            }
        }
        op::VECTOR_SHUFFLE => {
            let a = st.type_of(o[2], at)?;
            let b = st.type_of(o[3], at)?;
            let (Some((ca, na)), Some((cb, nb)), Type::Vector(c, n)) =
                (st.shape(a), st.shape(b), st.ty(t, at)?.clone())
            else {
                return err(at, "shuffle of non-vectors");
            };
            if na < 2 || nb < 2 || ca != c || cb != c {
                return err(at, "shuffle operands of the wrong component type");
            }
            let lits = &o[4..];
            if lits.len() != n as usize {
                return err(at, "shuffle literal count differs from the result size");
            }
            if lits.iter().any(|&l| l != u32::MAX && l >= na + nb) {
                return err(at, "shuffle component out of range");
            }
        }
        op::COMPOSITE_EXTRACT => {
            let mut cur = st.type_of(o[2], at)?;
            for &l in &o[3..] {
                cur = match st.ty(cur, at)?.clone() {
                    Type::Vector(c, n) if l < n => c,
                    Type::Array(e, n) if l < n => e,
                    Type::Struct(m) if (l as usize) < m.len() => m[l as usize],
                    _ => return err(at, "extract index out of range"),
                };
            }
            if cur != t {
                return err(at, "extract result type differs from the element type");
            }
        }
        op::CONVERT_F_TO_S => {
            let a = st.type_of(o[2], at)?;
            if !st.is_float_type(a)
                || !st.is_int_type(t)
                || st.shape(a).map(|s| s.1) != st.shape(t).map(|s| s.1)
            {
                return err(at, "float-to-int conversion with wrong types");
            }
        }
        op::EXT_INST => check_ext(st, inst, t)?,
        op::IMAGE_SAMPLE_IMPLICIT_LOD
        | op::IMAGE_SAMPLE_EXPLICIT_LOD
        | op::IMAGE_SAMPLE_DREF_IMPLICIT_LOD
        | op::IMAGE_SAMPLE_DREF_EXPLICIT_LOD => check_sample(st, inst, t)?,
        _ => {}
    }
    Ok(())
}

fn check_ext(st: &State, inst: &Inst<'_>, t: u32) -> Result<(), ValidationError> {
    let o = inst.operands;
    let at = inst.offset;
    if Some(o[2]) != st.ext_import {
        return err(at, "extended instruction from an unknown set");
    }
    let args = &o[4..];
    let (count, float) = match o[3] {
        glsl::FABS
        | glsl::FSIGN
        | glsl::FLOOR
        | glsl::FRACT
        | glsl::SIN
        | glsl::COS
        | glsl::EXP2
        | glsl::LOG2
        | glsl::SQRT => (1, true),
        glsl::FMIN | glsl::FMAX => (2, true),
        glsl::SCLAMP => (3, false),
        glsl::CROSS => {
            if st.shape(t).map(|s| s.1) != Some(3) {
                return err(at, "Cross result is not a 3-vector");
            }
            (2, true)
        }
        other => return err(at, format!("GLSL.std.450 instruction {other} not expected")),
    };
    if args.len() != count {
        return err(at, "GLSL.std.450 instruction with the wrong operand count");
    }
    let ok = if float {
        st.is_float_type(t)
    } else {
        st.is_int_type(t)
    };
    if !ok {
        return err(at, "GLSL.std.450 instruction with the wrong result type");
    }
    for &a in args {
        if st.type_of(a, at)? != t {
            return err(at, "GLSL.std.450 operand type differs from the result");
        }
    }
    Ok(())
}

fn check_sample(st: &State, inst: &Inst<'_>, t: u32) -> Result<(), ValidationError> {
    let o = inst.operands;
    let at = inst.offset;
    let dref = matches!(
        inst.opcode,
        op::IMAGE_SAMPLE_DREF_IMPLICIT_LOD | op::IMAGE_SAMPLE_DREF_EXPLICIT_LOD
    );
    let explicit = matches!(
        inst.opcode,
        op::IMAGE_SAMPLE_EXPLICIT_LOD | op::IMAGE_SAMPLE_DREF_EXPLICIT_LOD
    );
    let si = st.type_of(o[2], at)?;
    let Type::SampledImage(img) = st.ty(si, at)? else {
        return err(at, "sample from a non-sampled-image");
    };
    let Type::Image(dimension, depth, _) = st.ty(*img, at)?.clone() else {
        return err(at, "sampled image of a non-image");
    };
    let want = match dimension {
        crate::spirv::dim::D2 => 2,
        crate::spirv::dim::D3 | crate::spirv::dim::CUBE => 3,
        _ => return err(at, "unexpected image dimension"),
    };
    let ct = st.type_of(o[3], at)?;
    if !st.is_float_type(ct) || st.shape(ct).map(|s| s.1) != Some(want) {
        return err(at, "sample coordinate has the wrong size or type");
    }
    if dref {
        if depth != 1 {
            return err(at, "depth-compare sample from a non-depth image");
        }
        if st.ty(t, at)? != &Type::Float {
            return err(at, "depth-compare sample result is not a float");
        }
        let rt = st.type_of(o[4], at)?;
        if st.ty(rt, at)? != &Type::Float {
            return err(at, "depth reference is not a float");
        }
    } else if !st.is_float_type(t) || st.shape(t).map(|s| s.1) != Some(4) {
        return err(at, "sample result is not a float 4-vector");
    }
    let ops_at = if dref { 5 } else { 4 };
    let mask = o.get(ops_at).copied().unwrap_or(0);
    if explicit && mask & (image_operands::LOD | image_operands::GRAD) == 0 {
        return err(at, "explicit-LOD sample without Lod or Grad");
    }
    if !explicit && mask & (image_operands::LOD | image_operands::GRAD) != 0 {
        return err(at, "implicit-LOD sample with Lod or Grad");
    }
    // Operand types: Bias and Lod are floats, Grad are coordinate-sized.
    let mut k = ops_at + 1;
    for bit in [
        image_operands::BIAS,
        image_operands::LOD,
        image_operands::GRAD,
    ] {
        if mask & bit == 0 {
            continue;
        }
        let n = if bit == image_operands::GRAD { 2 } else { 1 };
        for _ in 0..n {
            let ty = st.type_of(o[k], at)?;
            let expect = if bit == image_operands::GRAD { want } else { 1 };
            if !st.is_float_type(ty) || st.shape(ty).map(|s| s.1) != Some(expect) {
                return err(at, "image operand of the wrong type");
            }
            k += 1;
        }
    }
    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Pos {
    Outside,
    FunctionStart,
    InBlock,
    BetweenBlocks,
}

#[allow(clippy::too_many_lines)]
fn check_functions(st: &State, insts: &[Inst<'_>]) -> Result<(), ValidationError> {
    let mut pos = Pos::Outside;
    let mut merge_targets: HashSet<u32> = HashSet::new();
    let mut prev: Option<u16> = None;
    let mut current_fn = 0usize;
    let mut fn_counter = 0usize;
    let mut first_block_instr = false;
    for inst in insts {
        let at = inst.offset;
        if section(inst.opcode) < 9 || (inst.opcode == op::VARIABLE && pos == Pos::Outside) {
            prev = Some(inst.opcode);
            continue;
        }
        match pos {
            Pos::Outside => {
                if inst.opcode != op::FUNCTION {
                    return err(at, "instruction outside a function");
                }
                current_fn = fn_counter;
                fn_counter += 1;
                pos = Pos::FunctionStart;
            }
            Pos::FunctionStart => {
                if inst.opcode != op::LABEL {
                    return err(at, "function body does not start with OpLabel");
                }
                pos = Pos::InBlock;
                first_block_instr = true;
            }
            Pos::BetweenBlocks => match inst.opcode {
                op::LABEL => pos = Pos::InBlock,
                op::FUNCTION_END => pos = Pos::Outside,
                _ => return err(at, "instruction after a terminator outside any block"),
            },
            Pos::InBlock => {
                match inst.opcode {
                    op::LABEL | op::FUNCTION_END | op::FUNCTION => {
                        return err(at, "block does not end with a terminator");
                    }
                    op::VARIABLE => {
                        if !first_block_instr || inst.operands[2] != storage::FUNCTION {
                            return err(
                                at,
                                "function variable not at the start of the entry block",
                            );
                        }
                    }
                    _ => first_block_instr = false,
                }
                if matches!(prev, Some(op::SELECTION_MERGE))
                    && inst.opcode != op::BRANCH_CONDITIONAL
                {
                    return err(at, "OpSelectionMerge not followed by OpBranchConditional");
                }
                if matches!(prev, Some(op::LOOP_MERGE))
                    && !matches!(inst.opcode, op::BRANCH | op::BRANCH_CONDITIONAL)
                {
                    return err(at, "OpLoopMerge not followed by a branch");
                }
                if inst.opcode == op::BRANCH_CONDITIONAL
                    && !matches!(prev, Some(op::SELECTION_MERGE | op::LOOP_MERGE))
                {
                    return err(at, "conditional branch without a structured merge");
                }
                match inst.opcode {
                    op::SELECTION_MERGE | op::LOOP_MERGE => {
                        let merge = inst.operands[0];
                        if !merge_targets.insert(merge) {
                            return err(
                                at,
                                format!("%{merge} is the merge block of two constructs"),
                            );
                        }
                        if inst.opcode == op::LOOP_MERGE && inst.operands[1] == merge {
                            return err(at, "loop continue target equals its merge block");
                        }
                    }
                    _ => {}
                }
                let targets: &[u32] = match inst.opcode {
                    op::BRANCH | op::SELECTION_MERGE => &inst.operands[..1],
                    op::BRANCH_CONDITIONAL => {
                        let c = st.type_of(inst.operands[0], at)?;
                        if st.ty(c, at)? != &Type::Bool {
                            return err(at, "branch condition is not a scalar boolean");
                        }
                        &inst.operands[1..3]
                    }
                    op::LOOP_MERGE => &inst.operands[..2],
                    _ => &[],
                };
                for &tgt in targets {
                    match st.labels.get(&tgt) {
                        Some(&f) if f == current_fn => {}
                        _ => return err(at, format!("branch target %{tgt} is not a label here")),
                    }
                }
                if is_terminator(inst.opcode) {
                    pos = Pos::BetweenBlocks;
                }
            }
        }
        prev = Some(inst.opcode);
    }
    if pos != Pos::Outside {
        return err(insts.last().map_or(0, |i| i.offset), "function not ended");
    }
    Ok(())
}

#[allow(clippy::too_many_lines)]
fn check_entry_and_interface(
    st: &State,
    insts: &[Inst<'_>],
) -> Result<(usize, u32), ValidationError> {
    let blocks = insts.iter().filter(|i| i.opcode == op::LABEL).count();
    let entry = insts
        .iter()
        .find(|i| i.opcode == op::ENTRY_POINT)
        .ok_or(ValidationError {
            offset: 0,
            message: "no entry point".into(),
        })?;
    let at = entry.offset;
    let model_id = entry.operands[0];
    let function = entry.operands[1];
    if !st.functions.contains_key(&function) {
        return err(at, "entry point does not name a function");
    }
    let Some((_, n)) = read_string(&entry.operands[2..]) else {
        return err(at, "entry point name");
    };
    let interface: Vec<u32> = entry.operands[2 + n..].to_vec();
    let modes: Vec<u32> = insts
        .iter()
        .filter(|i| i.opcode == op::EXECUTION_MODE)
        .map(|i| {
            if i.operands[0] == function {
                Ok(i.operands[1])
            } else {
                err(
                    i.offset,
                    "execution mode for a function that is not the entry point",
                )
            }
        })
        .collect::<Result<_, _>>()?;
    match model_id {
        model::FRAGMENT => {
            if !modes.contains(&mode::ORIGIN_UPPER_LEFT) {
                return err(at, "fragment entry point without OriginUpperLeft");
            }
        }
        model::VERTEX => {
            if !modes.is_empty() {
                return err(at, "vertex entry point with an execution mode");
            }
        }
        _ => return err(at, "execution model is neither vertex nor fragment"),
    }
    for &v in &interface {
        match st.variables.get(&v) {
            Some(&storage::INPUT | &storage::OUTPUT) => {}
            _ => {
                return err(
                    at,
                    format!("interface id %{v} is not an input/output variable"),
                );
            }
        }
    }
    let mut locations: HashSet<(u32, u32)> = HashSet::new();
    let mut bindings: HashSet<(u32, u32)> = HashSet::new();
    let mut ids: Vec<(&u32, &u32)> = st.variables.iter().collect();
    ids.sort();
    for (&var, &class) in ids {
        let builtin = st.has_decoration(var, decoration::BUILT_IN);
        let location = st.has_decoration(var, decoration::LOCATION);
        match class {
            storage::INPUT | storage::OUTPUT => {
                if !interface.contains(&var) {
                    return err(
                        0,
                        format!("input/output %{var} missing from the entry point"),
                    );
                }
                match (builtin, location) {
                    (Some(b), None) => {
                        if b.first() == Some(&crate::spirv::builtin::FRAG_DEPTH)
                            && !modes.contains(&mode::DEPTH_REPLACING)
                        {
                            return err(0, "FragDepth written without DepthReplacing");
                        }
                    }
                    (None, Some(l)) => {
                        let l = l.first().copied().unwrap_or(u32::MAX);
                        if !locations.insert((class, l)) {
                            return err(0, format!("location {l} used twice"));
                        }
                    }
                    _ => {
                        return err(
                            0,
                            format!("%{var} needs exactly one of BuiltIn and Location"),
                        );
                    }
                }
            }
            storage::UNIFORM | storage::UNIFORM_CONSTANT => {
                let (Some(set), Some(b)) = (
                    st.has_decoration(var, decoration::DESCRIPTOR_SET),
                    st.has_decoration(var, decoration::BINDING),
                ) else {
                    return err(0, format!("resource %{var} lacks DescriptorSet/Binding"));
                };
                let key = (
                    set.first().copied().unwrap_or(u32::MAX),
                    b.first().copied().unwrap_or(u32::MAX),
                );
                if !bindings.insert(key) {
                    return err(0, format!("set {} binding {} used twice", key.0, key.1));
                }
                if class == storage::UNIFORM {
                    check_block(st, var)?;
                }
            }
            storage::PUSH_CONSTANT => check_block(st, var)?,
            storage::PRIVATE | storage::FUNCTION => {}
            other => return err(0, format!("unexpected storage class {other}")),
        }
    }
    Ok((blocks, model_id))
}

fn check_block(st: &State, var: u32) -> Result<(), ValidationError> {
    let vt = st.type_of(var, 0)?;
    let Type::Pointer(_, pointee) = st.ty(vt, 0)? else {
        return err(0, "buffer variable is not a pointer");
    };
    let Type::Struct(members) = st.ty(*pointee, 0)? else {
        return err(0, "buffer variable does not point to a structure");
    };
    if st.has_decoration(*pointee, decoration::BLOCK).is_none() {
        return err(0, "buffer structure is not decorated Block");
    }
    for (i, m) in members.iter().enumerate() {
        let i = u32::try_from(i).unwrap_or(u32::MAX);
        let has_offset = st
            .member_decorations
            .get(&(*pointee, i))
            .is_some_and(|d| d.contains(&decoration::OFFSET));
        if !has_offset {
            return err(0, "buffer structure member without Offset");
        }
        if matches!(st.ty(*m, 0)?, Type::Array(..))
            && st.has_decoration(*m, decoration::ARRAY_STRIDE).is_none()
        {
            return err(0, "array in a buffer without ArrayStride");
        }
    }
    Ok(())
}

struct Block {
    label: u32,
    first: usize,
    last: usize,
}

fn successors(inst: &Inst<'_>) -> Vec<u32> {
    match inst.opcode {
        op::BRANCH => vec![inst.operands[0]],
        op::BRANCH_CONDITIONAL => vec![inst.operands[1], inst.operands[2]],
        _ => Vec::new(),
    }
}

/// Immediate dominators by the Cooper-Harvey-Kennedy iteration over a
/// reverse post-order; `None` for unreachable blocks.
fn dominators(succ: &[Vec<usize>]) -> Vec<Option<usize>> {
    let n = succ.len();
    let mut order = Vec::new();
    let mut seen = vec![false; n];
    // Iterative post-order DFS from block 0.
    let mut stack = vec![(0usize, 0usize)];
    seen[0] = true;
    while let Some(&mut (b, ref mut next)) = stack.last_mut() {
        if let Some(&s) = succ[b].get(*next) {
            *next += 1;
            if !seen[s] {
                seen[s] = true;
                stack.push((s, 0));
            }
        } else {
            order.push(b);
            stack.pop();
        }
    }
    order.reverse();
    let mut rpo_index = vec![usize::MAX; n];
    for (i, &b) in order.iter().enumerate() {
        rpo_index[b] = i;
    }
    let mut preds = vec![Vec::new(); n];
    for (b, ss) in succ.iter().enumerate() {
        if seen[b] {
            for &s in ss {
                preds[s].push(b);
            }
        }
    }
    let mut idom: Vec<Option<usize>> = vec![None; n];
    idom[0] = Some(0);
    let mut changed = true;
    while changed {
        changed = false;
        for &b in order.iter().skip(1) {
            let mut new: Option<usize> = None;
            for &p in &preds[b] {
                if idom[p].is_none() {
                    continue;
                }
                new = Some(match new {
                    None => p,
                    Some(mut a) => {
                        let mut c = p;
                        while a != c {
                            while rpo_index[a] > rpo_index[c] {
                                a = idom[a].unwrap_or(0);
                            }
                            while rpo_index[c] > rpo_index[a] {
                                c = idom[c].unwrap_or(0);
                            }
                        }
                        a
                    }
                });
            }
            if new.is_some() && idom[b] != new {
                idom[b] = new;
                changed = true;
            }
        }
    }
    idom
}

fn dominates(idom: &[Option<usize>], a: usize, mut b: usize) -> bool {
    loop {
        if a == b {
            return true;
        }
        match idom[b] {
            Some(p) if p != b => b = p,
            _ => return false,
        }
    }
}

fn check_dominance(
    st: &State,
    insts: &[Inst<'_>],
    parsed: &[Parsed],
) -> Result<(), ValidationError> {
    let mut i = 0;
    while i < insts.len() {
        if insts[i].opcode != op::FUNCTION {
            i += 1;
            continue;
        }
        // Collect the blocks of this function.
        let mut blocks: Vec<Block> = Vec::new();
        let mut j = i + 1;
        while j < insts.len() && insts[j].opcode != op::FUNCTION_END {
            if insts[j].opcode == op::LABEL {
                blocks.push(Block {
                    label: insts[j].operands[0],
                    first: j,
                    last: j,
                });
            } else if let Some(b) = blocks.last_mut() {
                b.last = j;
            }
            j += 1;
        }
        let index: HashMap<u32, usize> = blocks
            .iter()
            .enumerate()
            .map(|(k, b)| (b.label, k))
            .collect();
        let succ: Vec<Vec<usize>> = blocks
            .iter()
            .map(|b| {
                successors(&insts[b.last])
                    .iter()
                    .filter_map(|t| index.get(t).copied())
                    .collect()
            })
            .collect();
        let idom = dominators(&succ);
        let mut def_block: HashMap<u32, usize> = HashMap::new();
        for (k, b) in blocks.iter().enumerate() {
            for p in &parsed[b.first..=b.last] {
                if let Some(r) = p.res {
                    def_block.insert(r, k);
                }
            }
        }
        for (k, b) in blocks.iter().enumerate() {
            if idom[k].is_none() {
                continue; // unreachable: no dominance requirement
            }
            for n in b.first..=b.last {
                let inst = &insts[n];
                for &(id, kind) in &parsed[n].uses {
                    if kind != Use::Before || st.labels.contains_key(&id) {
                        continue;
                    }
                    if let Some(&d) = def_block.get(&id)
                        && d != k
                        && !dominates(&idom, d, k)
                    {
                        return err(
                            inst.offset,
                            format!("%{id} is used in a block its definition does not dominate"),
                        );
                    }
                }
            }
            let term = &insts[b.last];
            for &s in &succ[k] {
                if dominates(&idom, s, k) {
                    // Back edge: must be continue target -> loop header.
                    let header = &blocks[s];
                    let merge_inst = &insts[header.last.saturating_sub(1)];
                    let ok =
                        merge_inst.opcode == op::LOOP_MERGE && merge_inst.operands[1] == b.label;
                    if !ok {
                        return err(
                            term.offset,
                            "back edge that is not continue target -> loop header",
                        );
                    }
                }
            }
            if b.last > b.first {
                let m = &insts[b.last - 1];
                if matches!(m.opcode, op::SELECTION_MERGE | op::LOOP_MERGE)
                    && let Some(&mb) = index.get(&m.operands[0])
                    && idom[mb].is_some()
                    && !dominates(&idom, k, mb)
                {
                    return err(
                        m.offset,
                        "construct header does not dominate its merge block",
                    );
                }
            }
        }
        i = j;
    }
    Ok(())
}
