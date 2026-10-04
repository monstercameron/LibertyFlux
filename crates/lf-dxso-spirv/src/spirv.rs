//! A small SPIR-V 1.0 word emitter.
//!
//! Numbers below are from the public Khronos SPIR-V 1.0 specification and
//! the `GLSL.std.450` extended instruction set (written from those
//! documents, not cross-checked against a header in this environment;
//! Inferred, and pinned by the structural validator and the tests).
//!
//! The [`Builder`] keeps one word buffer per section of the SPIR-V logical
//! layout, so types, constants and global variables can be created lazily
//! while function code is being emitted, and still land before the code in
//! the final module. Types and constants are deduplicated, except arrays
//! and structures, which may carry layout decorations and are created
//! fresh each time (SPIR-V allows duplicate aggregate types).

use std::collections::HashMap;

/// SPIR-V magic number.
pub const MAGIC: u32 = 0x0723_0203;
/// SPIR-V version 1.0.
pub const VERSION_1_0: u32 = 0x0001_0000;
/// Generator magic. Zero is the documented value for an unregistered tool.
pub const GENERATOR: u32 = 0;

/// SPIR-V opcodes used by this crate.
#[allow(missing_docs)]
pub mod op {
    pub const NAME: u16 = 5;
    pub const MEMBER_NAME: u16 = 6;
    pub const EXT_INST_IMPORT: u16 = 11;
    pub const EXT_INST: u16 = 12;
    pub const MEMORY_MODEL: u16 = 14;
    pub const ENTRY_POINT: u16 = 15;
    pub const EXECUTION_MODE: u16 = 16;
    pub const CAPABILITY: u16 = 17;
    pub const TYPE_VOID: u16 = 19;
    pub const TYPE_BOOL: u16 = 20;
    pub const TYPE_INT: u16 = 21;
    pub const TYPE_FLOAT: u16 = 22;
    pub const TYPE_VECTOR: u16 = 23;
    pub const TYPE_IMAGE: u16 = 25;
    pub const TYPE_SAMPLED_IMAGE: u16 = 27;
    pub const TYPE_ARRAY: u16 = 28;
    pub const TYPE_STRUCT: u16 = 30;
    pub const TYPE_POINTER: u16 = 32;
    pub const TYPE_FUNCTION: u16 = 33;
    pub const CONSTANT_TRUE: u16 = 41;
    pub const CONSTANT_FALSE: u16 = 42;
    pub const CONSTANT: u16 = 43;
    pub const CONSTANT_COMPOSITE: u16 = 44;
    pub const FUNCTION: u16 = 54;
    pub const FUNCTION_END: u16 = 56;
    pub const FUNCTION_CALL: u16 = 57;
    pub const VARIABLE: u16 = 59;
    pub const LOAD: u16 = 61;
    pub const STORE: u16 = 62;
    pub const ACCESS_CHAIN: u16 = 65;
    pub const DECORATE: u16 = 71;
    pub const MEMBER_DECORATE: u16 = 72;
    pub const VECTOR_SHUFFLE: u16 = 79;
    pub const COMPOSITE_CONSTRUCT: u16 = 80;
    pub const COMPOSITE_EXTRACT: u16 = 81;
    pub const IMAGE_SAMPLE_IMPLICIT_LOD: u16 = 87;
    pub const IMAGE_SAMPLE_EXPLICIT_LOD: u16 = 88;
    pub const IMAGE_SAMPLE_DREF_IMPLICIT_LOD: u16 = 89;
    pub const IMAGE_SAMPLE_DREF_EXPLICIT_LOD: u16 = 90;
    pub const CONVERT_F_TO_S: u16 = 110;
    pub const F_NEGATE: u16 = 127;
    pub const I_ADD: u16 = 128;
    pub const F_ADD: u16 = 129;
    pub const F_SUB: u16 = 131;
    pub const F_MUL: u16 = 133;
    pub const F_DIV: u16 = 136;
    pub const DOT: u16 = 148;
    pub const ANY: u16 = 154;
    pub const LOGICAL_OR: u16 = 166;
    pub const LOGICAL_AND: u16 = 167;
    pub const LOGICAL_NOT: u16 = 168;
    pub const SELECT: u16 = 169;
    pub const I_EQUAL: u16 = 170;
    pub const I_NOT_EQUAL: u16 = 171;
    pub const S_GREATER_THAN_EQUAL: u16 = 175;
    pub const S_LESS_THAN: u16 = 177;
    pub const F_ORD_EQUAL: u16 = 180;
    pub const F_UNORD_NOT_EQUAL: u16 = 183;
    pub const F_ORD_LESS_THAN: u16 = 184;
    pub const F_ORD_GREATER_THAN: u16 = 186;
    pub const F_ORD_LESS_THAN_EQUAL: u16 = 188;
    pub const F_ORD_GREATER_THAN_EQUAL: u16 = 190;
    pub const SHIFT_RIGHT_LOGICAL: u16 = 194;
    pub const BITWISE_AND: u16 = 199;
    pub const DPDX: u16 = 207;
    pub const DPDY: u16 = 208;
    pub const LOOP_MERGE: u16 = 246;
    pub const SELECTION_MERGE: u16 = 247;
    pub const LABEL: u16 = 248;
    pub const BRANCH: u16 = 249;
    pub const BRANCH_CONDITIONAL: u16 = 250;
    pub const KILL: u16 = 252;
    pub const RETURN: u16 = 253;
    pub const UNREACHABLE: u16 = 255;
}

/// `GLSL.std.450` extended instruction numbers used by this crate.
#[allow(missing_docs)]
pub mod glsl {
    pub const FABS: u32 = 4;
    pub const FSIGN: u32 = 6;
    pub const FLOOR: u32 = 8;
    pub const FRACT: u32 = 10;
    pub const SIN: u32 = 13;
    pub const COS: u32 = 14;
    pub const EXP2: u32 = 29;
    pub const LOG2: u32 = 30;
    pub const SQRT: u32 = 31;
    pub const FMIN: u32 = 37;
    pub const FMAX: u32 = 40;
    pub const SCLAMP: u32 = 45;
    pub const CROSS: u32 = 68;
}

/// Capability numbers.
#[allow(missing_docs)]
pub mod capability {
    pub const SHADER: u32 = 1;
}

/// Execution models.
#[allow(missing_docs)]
pub mod model {
    pub const VERTEX: u32 = 0;
    pub const FRAGMENT: u32 = 4;
}

/// Execution modes.
#[allow(missing_docs)]
pub mod mode {
    pub const ORIGIN_UPPER_LEFT: u32 = 7;
    pub const DEPTH_REPLACING: u32 = 12;
}

/// Storage classes.
#[allow(missing_docs)]
pub mod storage {
    pub const UNIFORM_CONSTANT: u32 = 0;
    pub const INPUT: u32 = 1;
    pub const UNIFORM: u32 = 2;
    pub const OUTPUT: u32 = 3;
    pub const PRIVATE: u32 = 6;
    pub const FUNCTION: u32 = 7;
    pub const PUSH_CONSTANT: u32 = 9;
}

/// Decorations.
#[allow(missing_docs)]
pub mod decoration {
    pub const BLOCK: u32 = 2;
    pub const ARRAY_STRIDE: u32 = 6;
    pub const BUILT_IN: u32 = 11;
    pub const CENTROID: u32 = 16;
    pub const LOCATION: u32 = 30;
    pub const BINDING: u32 = 33;
    pub const DESCRIPTOR_SET: u32 = 34;
    pub const OFFSET: u32 = 35;
}

/// Built-in variables.
#[allow(missing_docs)]
pub mod builtin {
    pub const POSITION: u32 = 0;
    pub const POINT_SIZE: u32 = 1;
    pub const FRAG_COORD: u32 = 15;
    pub const FRONT_FACING: u32 = 17;
    pub const FRAG_DEPTH: u32 = 22;
}

/// Image dimensions.
#[allow(missing_docs)]
pub mod dim {
    pub const D2: u32 = 1;
    pub const D3: u32 = 2;
    pub const CUBE: u32 = 3;
}

/// Image operand mask bits.
#[allow(missing_docs)]
pub mod image_operands {
    pub const BIAS: u32 = 0x1;
    pub const LOD: u32 = 0x2;
    pub const GRAD: u32 = 0x4;
}

/// Addressing model `Logical`.
pub const ADDRESSING_LOGICAL: u32 = 0;
/// Memory model `GLSL450`.
pub const MEMORY_GLSL450: u32 = 1;

/// A SPIR-V result id.
pub type Id = u32;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum TypeKey {
    Void,
    Bool,
    Int(bool),
    Float,
    Vector(Id, u32),
    Pointer(u32, Id),
    Function(Id, Vec<Id>),
    Image(Id, u32, u32),
    SampledImage(Id),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum ConstKey {
    Scalar(Id, u32),
    Bool(bool),
    Composite(Id, Vec<Id>),
}

/// Append one instruction to a word buffer.
pub fn push_inst(buf: &mut Vec<u32>, opcode: u16, operands: &[u32]) {
    let count = u32::try_from(operands.len() + 1)
        .unwrap_or(u32::MAX)
        .min(0xFFFF);
    buf.push((count << 16) | u32::from(opcode));
    buf.extend_from_slice(operands);
}

/// Encode a string literal: UTF-8, NUL-terminated, padded to whole words.
#[must_use]
pub fn string_words(s: &str) -> Vec<u32> {
    let mut bytes = s.as_bytes().to_vec();
    bytes.push(0);
    while !bytes.len().is_multiple_of(4) {
        bytes.push(0);
    }
    bytes
        .chunks_exact(4)
        .map(|c| u32::from_le_bytes([c[0], c[1], c[2], c[3]]))
        .collect()
}

/// Section buffers plus id allocation and type/constant deduplication.
#[derive(Debug, Default)]
pub struct Builder {
    next_id: u32,
    capabilities: Vec<u32>,
    ext_imports: Vec<u32>,
    entry_points: Vec<u32>,
    execution_modes: Vec<u32>,
    debug: Vec<u32>,
    annotations: Vec<u32>,
    globals: Vec<u32>,
    code: Vec<u32>,
    types: HashMap<TypeKey, Id>,
    consts: HashMap<ConstKey, Id>,
    block_open: bool,
    glsl: Id,
}

impl Builder {
    /// A builder with the `Shader` capability and `GLSL.std.450` imported.
    #[must_use]
    pub fn new() -> Builder {
        let mut b = Builder {
            next_id: 1,
            ..Builder::default()
        };
        push_inst(&mut b.capabilities, op::CAPABILITY, &[capability::SHADER]);
        let glsl = b.id();
        let mut operands = vec![glsl];
        operands.extend(string_words("GLSL.std.450"));
        push_inst(&mut b.ext_imports, op::EXT_INST_IMPORT, &operands);
        b.glsl = glsl;
        b
    }

    /// Allocate a fresh id.
    pub fn id(&mut self) -> Id {
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    /// The id bound (one past the largest id).
    #[must_use]
    pub fn bound(&self) -> u32 {
        self.next_id
    }

    /// Add an entry point.
    pub fn entry_point(&mut self, model: u32, function: Id, name: &str, interface: &[Id]) {
        let mut operands = vec![model, function];
        operands.extend(string_words(name));
        operands.extend_from_slice(interface);
        push_inst(&mut self.entry_points, op::ENTRY_POINT, &operands);
    }

    /// Add an execution mode.
    pub fn execution_mode(&mut self, function: Id, mode: u32) {
        push_inst(
            &mut self.execution_modes,
            op::EXECUTION_MODE,
            &[function, mode],
        );
    }

    /// Name an id (debug information).
    pub fn name(&mut self, target: Id, name: &str) {
        let mut operands = vec![target];
        operands.extend(string_words(name));
        push_inst(&mut self.debug, op::NAME, &operands);
    }

    /// Name a structure member (debug information).
    pub fn member_name(&mut self, target: Id, member: u32, name: &str) {
        let mut operands = vec![target, member];
        operands.extend(string_words(name));
        push_inst(&mut self.debug, op::MEMBER_NAME, &operands);
    }

    /// Decorate an id.
    pub fn decorate(&mut self, target: Id, decoration: u32, literals: &[u32]) {
        let mut operands = vec![target, decoration];
        operands.extend_from_slice(literals);
        push_inst(&mut self.annotations, op::DECORATE, &operands);
    }

    /// Decorate a structure member.
    pub fn member_decorate(&mut self, target: Id, member: u32, decoration: u32, literals: &[u32]) {
        let mut operands = vec![target, member, decoration];
        operands.extend_from_slice(literals);
        push_inst(&mut self.annotations, op::MEMBER_DECORATE, &operands);
    }

    fn cached_type(&mut self, key: TypeKey, opcode: u16, tail: &[u32]) -> Id {
        if let Some(&id) = self.types.get(&key) {
            return id;
        }
        let id = self.id();
        let mut operands = vec![id];
        operands.extend_from_slice(tail);
        push_inst(&mut self.globals, opcode, &operands);
        self.types.insert(key, id);
        id
    }

    /// `void`
    pub fn t_void(&mut self) -> Id {
        self.cached_type(TypeKey::Void, op::TYPE_VOID, &[])
    }
    /// `bool`
    pub fn t_bool(&mut self) -> Id {
        self.cached_type(TypeKey::Bool, op::TYPE_BOOL, &[])
    }
    /// 32-bit signed integer.
    pub fn t_int(&mut self) -> Id {
        self.cached_type(TypeKey::Int(true), op::TYPE_INT, &[32, 1])
    }
    /// 32-bit unsigned integer.
    pub fn t_uint(&mut self) -> Id {
        self.cached_type(TypeKey::Int(false), op::TYPE_INT, &[32, 0])
    }
    /// 32-bit float.
    pub fn t_float(&mut self) -> Id {
        self.cached_type(TypeKey::Float, op::TYPE_FLOAT, &[32])
    }
    /// Vector of `n` components.
    pub fn t_vector(&mut self, component: Id, n: u32) -> Id {
        self.cached_type(
            TypeKey::Vector(component, n),
            op::TYPE_VECTOR,
            &[component, n],
        )
    }
    /// `vec4`
    pub fn t_vec4(&mut self) -> Id {
        let f = self.t_float();
        self.t_vector(f, 4)
    }
    /// `ivec4`
    pub fn t_ivec4(&mut self) -> Id {
        let i = self.t_int();
        self.t_vector(i, 4)
    }
    /// `bvec4`
    pub fn t_bvec4(&mut self) -> Id {
        let b = self.t_bool();
        self.t_vector(b, 4)
    }
    /// Pointer type.
    pub fn t_pointer(&mut self, class: u32, pointee: Id) -> Id {
        self.cached_type(
            TypeKey::Pointer(class, pointee),
            op::TYPE_POINTER,
            &[class, pointee],
        )
    }
    /// Function type.
    pub fn t_function(&mut self, ret: Id, params: &[Id]) -> Id {
        let mut tail = vec![ret];
        tail.extend_from_slice(params);
        self.cached_type(
            TypeKey::Function(ret, params.to_vec()),
            op::TYPE_FUNCTION,
            &tail,
        )
    }
    /// Sampled float image of a dimension, optionally a depth image.
    pub fn t_image(&mut self, dimension: u32, depth: bool) -> Id {
        let f = self.t_float();
        let depth = u32::from(depth);
        // Sampled type, Dim, Depth, Arrayed, MS, Sampled = 1, Format Unknown.
        self.cached_type(
            TypeKey::Image(f, dimension, depth),
            op::TYPE_IMAGE,
            &[f, dimension, depth, 0, 0, 1, 0],
        )
    }
    /// Sampled-image type.
    pub fn t_sampled_image(&mut self, image: Id) -> Id {
        self.cached_type(
            TypeKey::SampledImage(image),
            op::TYPE_SAMPLED_IMAGE,
            &[image],
        )
    }
    /// A fresh (not deduplicated) array type.
    pub fn t_array_unique(&mut self, element: Id, length: u32) -> Id {
        let len = self.c_uint(length);
        let id = self.id();
        push_inst(&mut self.globals, op::TYPE_ARRAY, &[id, element, len]);
        id
    }
    /// A fresh (not deduplicated) structure type.
    pub fn t_struct_unique(&mut self, members: &[Id]) -> Id {
        let id = self.id();
        let mut operands = vec![id];
        operands.extend_from_slice(members);
        push_inst(&mut self.globals, op::TYPE_STRUCT, &operands);
        id
    }

    fn scalar_const(&mut self, ty: Id, bits: u32) -> Id {
        let key = ConstKey::Scalar(ty, bits);
        if let Some(&id) = self.consts.get(&key) {
            return id;
        }
        let id = self.id();
        push_inst(&mut self.globals, op::CONSTANT, &[ty, id, bits]);
        self.consts.insert(key, id);
        id
    }

    /// Float constant, deduplicated by bit pattern (keeps `-0.0` and NaN payloads).
    pub fn c_float_bits(&mut self, bits: u32) -> Id {
        let t = self.t_float();
        self.scalar_const(t, bits)
    }
    /// Float constant.
    pub fn c_float(&mut self, v: f32) -> Id {
        self.c_float_bits(v.to_bits())
    }
    /// Signed integer constant.
    pub fn c_int(&mut self, v: i32) -> Id {
        let t = self.t_int();
        self.scalar_const(t, v.cast_unsigned())
    }
    /// Unsigned integer constant.
    pub fn c_uint(&mut self, v: u32) -> Id {
        let t = self.t_uint();
        self.scalar_const(t, v)
    }
    /// Boolean constant.
    pub fn c_bool(&mut self, v: bool) -> Id {
        let key = ConstKey::Bool(v);
        if let Some(&id) = self.consts.get(&key) {
            return id;
        }
        let t = self.t_bool();
        let id = self.id();
        let opcode = if v {
            op::CONSTANT_TRUE
        } else {
            op::CONSTANT_FALSE
        };
        push_inst(&mut self.globals, opcode, &[t, id]);
        self.consts.insert(key, id);
        id
    }
    /// Composite constant.
    pub fn c_composite(&mut self, ty: Id, parts: &[Id]) -> Id {
        let key = ConstKey::Composite(ty, parts.to_vec());
        if let Some(&id) = self.consts.get(&key) {
            return id;
        }
        let id = self.id();
        let mut operands = vec![ty, id];
        operands.extend_from_slice(parts);
        push_inst(&mut self.globals, op::CONSTANT_COMPOSITE, &operands);
        self.consts.insert(key, id);
        id
    }
    /// `vec4` constant from raw float bits.
    pub fn c_vec4_bits(&mut self, bits: [u32; 4]) -> Id {
        let parts = bits.map(|b| self.c_float_bits(b));
        let t = self.t_vec4();
        self.c_composite(t, &parts)
    }
    /// `vec4` constant with every component `v`.
    pub fn c_vec4_splat(&mut self, v: f32) -> Id {
        self.c_vec4_bits([v.to_bits(); 4])
    }
    /// `ivec4` constant.
    pub fn c_ivec4(&mut self, v: [i32; 4]) -> Id {
        let parts = v.map(|x| self.c_int(x));
        let t = self.t_ivec4();
        self.c_composite(t, &parts)
    }

    /// A module-scope variable, with an optional initializer.
    pub fn global_variable(&mut self, pointer_type: Id, class: u32, init: Option<Id>) -> Id {
        let id = self.id();
        let mut operands = vec![pointer_type, id, class];
        if let Some(init) = init {
            operands.push(init);
        }
        push_inst(&mut self.globals, op::VARIABLE, &operands);
        id
    }

    /// Whether the current block is open (no terminator yet).
    #[must_use]
    pub fn block_open(&self) -> bool {
        self.block_open
    }

    /// Begin a function definition.
    pub fn begin_function(&mut self, function: Id, ret: Id, fn_type: Id) {
        push_inst(&mut self.code, op::FUNCTION, &[ret, function, 0, fn_type]);
        let entry = self.id();
        self.label(entry);
    }

    /// End a function definition. Closes a dangling block with `OpReturn`.
    pub fn end_function(&mut self) {
        if self.block_open {
            self.terminate(op::RETURN, &[]);
        }
        push_inst(&mut self.code, op::FUNCTION_END, &[]);
    }

    /// Start a block. If a block is still open it falls through by an
    /// explicit branch to the new one.
    pub fn label(&mut self, id: Id) {
        if self.block_open {
            push_inst(&mut self.code, op::BRANCH, &[id]);
        }
        push_inst(&mut self.code, op::LABEL, &[id]);
        self.block_open = true;
    }

    fn ensure_block(&mut self) {
        if !self.block_open {
            // Code after a terminator: give it an unreachable block.
            let id = self.id();
            push_inst(&mut self.code, op::LABEL, &[id]);
            self.block_open = true;
        }
    }

    /// Emit a non-terminating instruction with no result.
    pub fn emit(&mut self, opcode: u16, operands: &[u32]) {
        self.ensure_block();
        push_inst(&mut self.code, opcode, operands);
    }

    /// Emit an instruction with a result type and id; returns the id.
    pub fn result(&mut self, opcode: u16, ty: Id, operands: &[u32]) -> Id {
        self.ensure_block();
        let id = self.id();
        let mut all = vec![ty, id];
        all.extend_from_slice(operands);
        push_inst(&mut self.code, opcode, &all);
        id
    }

    /// Emit a `GLSL.std.450` instruction.
    pub fn ext(&mut self, ty: Id, instruction: u32, args: &[Id]) -> Id {
        let mut operands = vec![self.glsl, instruction];
        operands.extend_from_slice(args);
        self.result(op::EXT_INST, ty, &operands)
    }

    /// Emit a block terminator.
    pub fn terminate(&mut self, opcode: u16, operands: &[u32]) {
        self.ensure_block();
        push_inst(&mut self.code, opcode, operands);
        self.block_open = false;
    }

    /// Emit a merge instruction followed by its terminator.
    pub fn merge_and_terminate(
        &mut self,
        merge_op: u16,
        merge: &[u32],
        opcode: u16,
        operands: &[u32],
    ) {
        self.ensure_block();
        push_inst(&mut self.code, merge_op, merge);
        self.terminate(opcode, operands);
    }

    /// Branch to `target` if the current block is still open.
    pub fn branch_if_open(&mut self, target: Id) {
        if self.block_open {
            self.terminate(op::BRANCH, &[target]);
        }
    }

    /// Assemble the module words.
    #[must_use]
    pub fn assemble(self) -> Vec<u32> {
        let mut words = vec![MAGIC, VERSION_1_0, GENERATOR, self.next_id, 0];
        words.extend(self.capabilities);
        words.extend(self.ext_imports);
        push_inst(
            &mut words,
            op::MEMORY_MODEL,
            &[ADDRESSING_LOGICAL, MEMORY_GLSL450],
        );
        words.extend(self.entry_points);
        words.extend(self.execution_modes);
        words.extend(self.debug);
        words.extend(self.annotations);
        words.extend(self.globals);
        words.extend(self.code);
        words
    }
}
