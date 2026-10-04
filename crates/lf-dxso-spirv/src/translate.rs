//! Translation of a decoded [`Shader`] into a SPIR-V [`Module`].
//!
//! The crate-level documentation is the specification of what each
//! instruction becomes; this file implements it. Register files are
//! module-scope `Private` variables, so values cross structured blocks and
//! subroutines (SPIR-V functions) without phi nodes.

// Single-letter bindings mirror the operand names of the instruction
// descriptions (`a`, `b`, `c` for src0..src2, `x`, `y`, `w` for components).
#![allow(clippy::many_single_char_names)]

use std::collections::{BTreeMap, BTreeSet};

use crate::binding::{self, usage};
use crate::decode::{
    Dcl, DstParam, IDENTITY_SWIZZLE, Instruction, Opcode, Payload, RegType, Register, RelAddr,
    Shader, SrcMod, SrcParam, Stage, TEXLD_BIAS, TEXLD_PROJECT, TextureType,
};
use crate::spirv::{
    Builder, Id, builtin, decoration, dim, glsl, image_operands, mode, model, op, storage,
};
use crate::{
    BuiltIn, ConstantUse, Error, InterfaceVar, Module, Options, SamplerBinding, TextureDim,
};

const TEMP_REGISTERS: u16 = 32;
const VS_INPUT_REGISTERS: u16 = 16;
const PS_INPUT_REGISTERS: u16 = 10;
const VS_OUTPUT_REGISTERS: u16 = 12;
const MISC_POSITION: u16 = 0;
const MISC_FACE: u16 = 1;
/// `lit` clamps its exponent to this magnitude (Direct3D 9 documentation).
const LIT_EXPONENT_LIMIT: f32 = 127.996_1;

#[derive(Debug, Clone, Copy)]
struct InputDecl {
    reg: Register,
    mask: u8,
    var: Id,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum OutputKind {
    Position,
    PointSize,
    Varying,
}

#[derive(Debug, Clone, Copy)]
struct OutputDecl {
    reg: u16,
    mask: u8,
    var: Id,
    kind: OutputKind,
}

#[derive(Debug, Clone, Copy)]
struct SamplerDecl {
    dim: TextureDim,
    depth: bool,
    var: Id,
}

#[derive(Debug, Clone, Copy)]
enum Flow {
    If {
        merge: Id,
        else_label: Id,
        has_else: bool,
    },
    Loop {
        header: Id,
        merge: Id,
        cont: Id,
        counter: Id,
        /// `(step, saved aL)` for `loop`; `None` for `rep`.
        loop_reg: Option<(Id, Id)>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Current {
    Main,
    Sub,
    None,
}

/// Cached common type ids.
#[derive(Debug, Clone, Copy)]
struct Types {
    void: Id,
    f32: Id,
    v2: Id,
    v3: Id,
    v4: Id,
    i32: Id,
    iv4: Id,
    u32: Id,
    bool: Id,
    bv4: Id,
    fn_void: Id,
}

struct Translator<'s> {
    b: Builder,
    sh: &'s Shader,
    opts: Options,
    t: Types,
    main: Id,
    temps: BTreeMap<u16, Id>,
    addr: Option<Id>,
    loop_reg: Option<Id>,
    pred: Option<Id>,
    inputs: Vec<InputDecl>,
    input_array: Option<Id>,
    frag_coord: Option<Id>,
    front_facing: Option<Id>,
    outputs: Vec<OutputDecl>,
    output_stage: BTreeMap<u16, Id>,
    output_array: Option<Id>,
    color_stage: BTreeMap<u16, (Id, Id)>,
    depth_stage: Option<(Id, Id)>,
    float_ubo: Option<Id>,
    int_bool_ubo: Option<Id>,
    push: Option<Id>,
    samplers: BTreeMap<u16, SamplerDecl>,
    defs_f: BTreeMap<u16, [u32; 4]>,
    defs_i: BTreeMap<u16, [i32; 4]>,
    defs_b: BTreeMap<u16, bool>,
    labels: BTreeMap<u16, Id>,
    defined_labels: BTreeSet<u16>,
    calls: BTreeMap<Option<u16>, BTreeSet<u16>>,
    current_label: Option<u16>,
    flow: Vec<Flow>,
    interface: Vec<Id>,
    refl_inputs: Vec<InterfaceVar>,
    refl_outputs: Vec<InterfaceVar>,
    float_read: BTreeSet<u16>,
    float_relative: bool,
    int_read: BTreeSet<u16>,
    bool_read: BTreeSet<u16>,
}

fn invalid(offset: usize, reason: impl Into<String>) -> Error {
    Error::Invalid {
        offset,
        reason: reason.into(),
    }
}

fn unsupported(offset: usize, what: impl Into<String>) -> Error {
    Error::Unsupported {
        offset,
        what: what.into(),
    }
}

fn mask_components(mask: u8) -> impl Iterator<Item = u32> {
    (0..4u32).filter(move |i| mask & (1 << i) != 0)
}

/// Translate a decoded shader.
pub(crate) fn translate(sh: &Shader, opts: Options) -> Result<Module, Error> {
    let mut b = Builder::new();
    let void = b.t_void();
    let f32_ = b.t_float();
    let v2 = b.t_vector(f32_, 2);
    let v3 = b.t_vector(f32_, 3);
    let v4 = b.t_vector(f32_, 4);
    let i32_ = b.t_int();
    let iv4 = b.t_vector(i32_, 4);
    let u32_ = b.t_uint();
    let bool_ = b.t_bool();
    let bv4 = b.t_vector(bool_, 4);
    let fn_void = b.t_function(void, &[]);
    let main = b.id();
    b.name(main, "main");
    let mut tr = Translator {
        b,
        sh,
        opts,
        t: Types {
            void,
            f32: f32_,
            v2,
            v3,
            v4,
            i32: i32_,
            iv4,
            u32: u32_,
            bool: bool_,
            bv4,
            fn_void,
        },
        main,
        temps: BTreeMap::new(),
        addr: None,
        loop_reg: None,
        pred: None,
        inputs: Vec::new(),
        input_array: None,
        frag_coord: None,
        front_facing: None,
        outputs: Vec::new(),
        output_stage: BTreeMap::new(),
        output_array: None,
        color_stage: BTreeMap::new(),
        depth_stage: None,
        float_ubo: None,
        int_bool_ubo: None,
        push: None,
        samplers: BTreeMap::new(),
        defs_f: BTreeMap::new(),
        defs_i: BTreeMap::new(),
        defs_b: BTreeMap::new(),
        labels: BTreeMap::new(),
        defined_labels: BTreeSet::new(),
        calls: BTreeMap::new(),
        current_label: None,
        flow: Vec::new(),
        interface: Vec::new(),
        refl_inputs: Vec::new(),
        refl_outputs: Vec::new(),
        float_read: BTreeSet::new(),
        float_relative: false,
        int_read: BTreeSet::new(),
        bool_read: BTreeSet::new(),
    };
    tr.prescan()?;
    tr.body()?;
    Ok(tr.finish())
}

impl Translator<'_> {
    fn stage(&self) -> Stage {
        self.sh.stage
    }

    // ----------------------------------------------------------------- prescan

    fn prescan(&mut self) -> Result<(), Error> {
        let sh = self.sh;
        let mut input_relative = false;
        let mut output_relative = false;
        for ins in &sh.instructions {
            match ins.payload {
                Payload::Dcl(dcl) => self.declare(ins.offset, &dcl)?,
                Payload::DefF(v) => {
                    let dst = self.def_target(ins, RegType::Const)?;
                    self.defs_f.insert(dst, v);
                }
                Payload::DefI(v) => {
                    let dst = self.def_target(ins, RegType::ConstInt)?;
                    self.defs_i.insert(dst, v);
                }
                Payload::DefB(v) => {
                    let dst = self.def_target(ins, RegType::ConstBool)?;
                    self.defs_b.insert(dst, v);
                }
                Payload::None => {}
            }
            for s in ins.src.iter().chain(ins.predicate.iter()) {
                if s.reg.kind == RegType::Input && s.relative.is_some() {
                    input_relative = true;
                }
            }
            if let Some(d) = ins.dst
                && ins.payload == Payload::None
            {
                {
                    self.check_range(d.reg, ins.offset)?;
                    match d.reg.kind {
                        RegType::Output if d.relative.is_some() => output_relative = true,
                        RegType::ColorOut => {
                            self.color_output(d.reg.num);
                        }
                        RegType::DepthOut => {
                            self.depth_output();
                        }
                        _ => {}
                    }
                }
            }
        }
        if input_relative {
            self.make_input_array();
        }
        if self.stage() == Stage::Vertex {
            self.make_output_staging(output_relative);
        }
        Ok(())
    }

    fn def_target(&self, ins: &Instruction, kind: RegType) -> Result<u16, Error> {
        let d = ins
            .dst
            .ok_or_else(|| invalid(ins.offset, "def without register"))?;
        if d.reg.kind != kind {
            return Err(invalid(ins.offset, "def targets the wrong register file"));
        }
        self.check_range(d.reg, ins.offset)?;
        Ok(d.reg.num)
    }

    fn register_limit(&self, kind: RegType) -> Option<u16> {
        let vs = self.stage() == Stage::Vertex;
        Some(match kind {
            RegType::Temp => TEMP_REGISTERS,
            RegType::Input => {
                if vs {
                    VS_INPUT_REGISTERS
                } else {
                    PS_INPUT_REGISTERS
                }
            }
            RegType::Const => {
                let n = if vs {
                    binding::VS_FLOAT_CONSTANTS
                } else {
                    binding::PS_FLOAT_CONSTANTS
                };
                u16::try_from(n).unwrap_or(u16::MAX)
            }
            RegType::Output if vs => VS_OUTPUT_REGISTERS,
            RegType::ConstInt => u16::try_from(binding::INT_CONSTANTS).unwrap_or(u16::MAX),
            RegType::ConstBool => u16::try_from(binding::BOOL_CONSTANTS).unwrap_or(u16::MAX),
            RegType::Sampler => {
                if vs {
                    binding::VS_SAMPLERS
                } else {
                    binding::PS_SAMPLERS
                }
            }
            RegType::ColorOut if !vs => binding::COLOR_OUTPUTS,
            RegType::DepthOut if !vs => 1,
            RegType::Addr if vs => 1,
            RegType::Loop | RegType::Predicate => 1,
            RegType::MiscType if !vs => 2,
            RegType::Label => 2048,
            _ => return None,
        })
    }

    fn check_range(&self, reg: Register, offset: usize) -> Result<(), Error> {
        match self.register_limit(reg.kind) {
            Some(limit) if reg.num < limit => Ok(()),
            Some(_) => Err(invalid(
                offset,
                format!("register {:?}{} out of range", reg.kind, reg.num),
            )),
            None => Err(unsupported(
                offset,
                format!(
                    "register file {:?} in a {:?} shader (not valid in shader model 3)",
                    reg.kind,
                    self.stage()
                ),
            )),
        }
    }

    #[allow(clippy::too_many_lines)]
    fn declare(&mut self, offset: usize, dcl: &Dcl) -> Result<(), Error> {
        let reg = dcl.dst.reg;
        self.check_range(reg, offset)?;
        let mask = if dcl.dst.write_mask == 0 {
            0xF
        } else {
            dcl.dst.write_mask
        };
        let vs = self.stage() == Stage::Vertex;
        match reg.kind {
            RegType::Input => {
                let location = if vs {
                    binding::vertex_input_location(dcl.usage, dcl.usage_index)
                } else {
                    binding::varying_location(dcl.usage, dcl.usage_index)
                }
                .ok_or_else(|| {
                    unsupported(
                        offset,
                        format!(
                            "input usage {}{} has no location in the binding convention",
                            binding::usage_name(dcl.usage),
                            dcl.usage_index
                        ),
                    )
                })?;
                if self
                    .refl_inputs
                    .iter()
                    .any(|v| v.location == Some(location))
                {
                    return Err(invalid(offset, "two input declarations share a usage"));
                }
                let var = self.interface_var(storage::INPUT, self.t.v4);
                self.b.decorate(var, decoration::LOCATION, &[location]);
                if dcl.dst.centroid {
                    self.b.decorate(var, decoration::CENTROID, &[]);
                }
                let name = format!(
                    "v{}_{}{}",
                    reg.num,
                    binding::usage_name(dcl.usage),
                    dcl.usage_index
                );
                self.b.name(var, &name);
                self.inputs.push(InputDecl { reg, mask, var });
                self.refl_inputs.push(InterfaceVar {
                    reg,
                    usage: Some((dcl.usage, dcl.usage_index)),
                    write_mask: mask,
                    location: Some(location),
                    builtin: None,
                    centroid: dcl.dst.centroid,
                });
            }
            RegType::MiscType => {
                let builtin = if reg.num == MISC_POSITION {
                    self.frag_coord_var();
                    BuiltIn::FragCoord
                } else {
                    self.front_facing_var();
                    BuiltIn::FrontFacing
                };
                self.refl_inputs.push(InterfaceVar {
                    reg,
                    usage: None,
                    write_mask: mask,
                    location: None,
                    builtin: Some(builtin),
                    centroid: false,
                });
            }
            RegType::Output => {
                let (kind, location, ty) = match (dcl.usage, dcl.usage_index) {
                    (usage::POSITION, 0) => (OutputKind::Position, None, self.t.v4),
                    (usage::PSIZE, 0) => (OutputKind::PointSize, None, self.t.f32),
                    (u, i) => {
                        let l = binding::varying_location(u, i).ok_or_else(|| {
                            unsupported(
                                offset,
                                format!(
                                    "output usage {}{} has no location in the binding convention",
                                    binding::usage_name(u),
                                    i
                                ),
                            )
                        })?;
                        (OutputKind::Varying, Some(l), self.t.v4)
                    }
                };
                let duplicate = self.outputs.iter().any(|o| o.kind == kind)
                    && kind != OutputKind::Varying
                    || location.is_some()
                        && self.refl_outputs.iter().any(|v| v.location == location);
                if duplicate {
                    return Err(invalid(offset, "two output declarations share a usage"));
                }
                let var = self.interface_var(storage::OUTPUT, ty);
                let builtin = match kind {
                    OutputKind::Position => {
                        self.b
                            .decorate(var, decoration::BUILT_IN, &[builtin::POSITION]);
                        Some(BuiltIn::Position)
                    }
                    OutputKind::PointSize => {
                        self.b
                            .decorate(var, decoration::BUILT_IN, &[builtin::POINT_SIZE]);
                        Some(BuiltIn::PointSize)
                    }
                    OutputKind::Varying => {
                        if let Some(l) = location {
                            self.b.decorate(var, decoration::LOCATION, &[l]);
                        }
                        None
                    }
                };
                let name = format!(
                    "o{}_{}{}",
                    reg.num,
                    binding::usage_name(dcl.usage),
                    dcl.usage_index
                );
                self.b.name(var, &name);
                self.outputs.push(OutputDecl {
                    reg: reg.num,
                    mask,
                    var,
                    kind,
                });
                self.refl_outputs.push(InterfaceVar {
                    reg,
                    usage: Some((dcl.usage, dcl.usage_index)),
                    write_mask: mask,
                    location,
                    builtin,
                    centroid: false,
                });
            }
            RegType::Sampler => {
                let depth = self.opts.depth_compare_samplers & (1 << reg.num) != 0;
                let (dim_id, tdim) = match dcl.texture_type {
                    TextureType::Tex2d => (dim::D2, TextureDim::D2),
                    TextureType::Cube => (dim::CUBE, TextureDim::Cube),
                    TextureType::Volume => (dim::D3, TextureDim::D3),
                    TextureType::Unknown(t) => {
                        return Err(unsupported(
                            offset,
                            format!("sampler texture type {t} (only 2d, cube and volume)"),
                        ));
                    }
                };
                if depth && tdim != TextureDim::D2 {
                    return Err(unsupported(
                        offset,
                        "depth-compare sampler that is not two-dimensional",
                    ));
                }
                if self.samplers.contains_key(&reg.num) {
                    return Err(invalid(offset, "sampler declared twice"));
                }
                let image = self.b.t_image(dim_id, depth);
                let sampled = self.b.t_sampled_image(image);
                let ptr = self.b.t_pointer(storage::UNIFORM_CONSTANT, sampled);
                let var = self.b.global_variable(ptr, storage::UNIFORM_CONSTANT, None);
                let base = if self.stage() == Stage::Vertex {
                    binding::VS_SAMPLER_BINDING_BASE
                } else {
                    binding::PS_SAMPLER_BINDING_BASE
                };
                self.b
                    .decorate(var, decoration::DESCRIPTOR_SET, &[binding::SAMPLER_SET]);
                self.b
                    .decorate(var, decoration::BINDING, &[base + u32::from(reg.num)]);
                self.b.name(var, &format!("s{}", reg.num));
                self.samplers.insert(
                    reg.num,
                    SamplerDecl {
                        dim: tdim,
                        depth,
                        var,
                    },
                );
            }
            _ => {
                return Err(unsupported(
                    offset,
                    format!("dcl of register file {:?}", reg.kind),
                ));
            }
        }
        Ok(())
    }

    fn interface_var(&mut self, class: u32, ty: Id) -> Id {
        let ptr = self.b.t_pointer(class, ty);
        let var = self.b.global_variable(ptr, class, None);
        self.interface.push(var);
        var
    }

    fn frag_coord_var(&mut self) -> Id {
        if let Some(v) = self.frag_coord {
            return v;
        }
        let v = self.interface_var(storage::INPUT, self.t.v4);
        self.b
            .decorate(v, decoration::BUILT_IN, &[builtin::FRAG_COORD]);
        self.b.name(v, "vPos");
        self.frag_coord = Some(v);
        v
    }

    fn front_facing_var(&mut self) -> Id {
        if let Some(v) = self.front_facing {
            return v;
        }
        let v = self.interface_var(storage::INPUT, self.t.bool);
        self.b
            .decorate(v, decoration::BUILT_IN, &[builtin::FRONT_FACING]);
        self.b.name(v, "vFace");
        self.front_facing = Some(v);
        v
    }

    fn private_var(&mut self, ty: Id, init: Id, name: &str) -> Id {
        let ptr = self.b.t_pointer(storage::PRIVATE, ty);
        let v = self.b.global_variable(ptr, storage::PRIVATE, Some(init));
        self.b.name(v, name);
        v
    }

    fn zero4(&mut self) -> Id {
        self.b.c_vec4_splat(0.0)
    }

    fn color_output(&mut self, n: u16) -> (Id, Id) {
        if let Some(&p) = self.color_stage.get(&n) {
            return p;
        }
        let zero = self.zero4();
        let stage = self.private_var(self.t.v4, zero, &format!("oC{n}_stage"));
        let var = self.interface_var(storage::OUTPUT, self.t.v4);
        self.b.decorate(var, decoration::LOCATION, &[u32::from(n)]);
        self.b.name(var, &format!("oC{n}"));
        self.refl_outputs.push(InterfaceVar {
            reg: Register {
                kind: RegType::ColorOut,
                num: n,
            },
            usage: None,
            write_mask: 0xF,
            location: Some(u32::from(n)),
            builtin: None,
            centroid: false,
        });
        self.color_stage.insert(n, (stage, var));
        (stage, var)
    }

    fn depth_output(&mut self) -> (Id, Id) {
        if let Some(p) = self.depth_stage {
            return p;
        }
        let zero = self.zero4();
        let stage = self.private_var(self.t.v4, zero, "oDepth_stage");
        let var = self.interface_var(storage::OUTPUT, self.t.f32);
        self.b
            .decorate(var, decoration::BUILT_IN, &[builtin::FRAG_DEPTH]);
        self.b.name(var, "oDepth");
        self.refl_outputs.push(InterfaceVar {
            reg: Register {
                kind: RegType::DepthOut,
                num: 0,
            },
            usage: None,
            write_mask: 0x1,
            location: None,
            builtin: Some(BuiltIn::FragDepth),
            centroid: false,
        });
        self.depth_stage = Some((stage, var));
        (stage, var)
    }

    fn input_registers(&self) -> u16 {
        self.register_limit(RegType::Input)
            .unwrap_or(PS_INPUT_REGISTERS)
    }

    fn make_input_array(&mut self) {
        let len = self.input_registers();
        let arr = self.b.t_array_unique(self.t.v4, u32::from(len));
        let zero = self.zero4();
        let parts = vec![zero; usize::from(len)];
        let init = self.b.c_composite(arr, &parts);
        let v = self.private_var(arr, init, "v");
        self.input_array = Some(v);
    }

    fn make_output_staging(&mut self, relative: bool) {
        if relative {
            let arr = self
                .b
                .t_array_unique(self.t.v4, u32::from(VS_OUTPUT_REGISTERS));
            let zero = self.zero4();
            let parts = vec![zero; usize::from(VS_OUTPUT_REGISTERS)];
            let init = self.b.c_composite(arr, &parts);
            let v = self.private_var(arr, init, "o");
            self.output_array = Some(v);
        } else {
            let regs: BTreeSet<u16> = self.outputs.iter().map(|o| o.reg).collect();
            for r in regs {
                let zero = self.zero4();
                let v = self.private_var(self.t.v4, zero, &format!("o{r}"));
                self.output_stage.insert(r, v);
            }
        }
    }

    // -------------------------------------------------------------- resources

    fn float_ubo(&mut self) -> Id {
        if let Some(v) = self.float_ubo {
            return v;
        }
        let (count, bind) = if self.stage() == Stage::Vertex {
            (binding::VS_FLOAT_CONSTANTS, binding::VS_FLOAT_BINDING)
        } else {
            (binding::PS_FLOAT_CONSTANTS, binding::PS_FLOAT_BINDING)
        };
        let arr = self.b.t_array_unique(self.t.v4, count);
        self.b
            .decorate(arr, decoration::ARRAY_STRIDE, &[binding::CONSTANT_STRIDE]);
        let st = self.b.t_struct_unique(&[arr]);
        self.b.decorate(st, decoration::BLOCK, &[]);
        self.b.member_decorate(st, 0, decoration::OFFSET, &[0]);
        self.b.name(st, "FloatConstants");
        self.b.member_name(st, 0, "c");
        let ptr = self.b.t_pointer(storage::UNIFORM, st);
        let v = self.b.global_variable(ptr, storage::UNIFORM, None);
        self.b
            .decorate(v, decoration::DESCRIPTOR_SET, &[binding::CONSTANT_SET]);
        self.b.decorate(v, decoration::BINDING, &[bind]);
        self.b.name(v, "c");
        self.float_ubo = Some(v);
        v
    }

    fn int_bool_ubo(&mut self) -> Id {
        if let Some(v) = self.int_bool_ubo {
            return v;
        }
        let bind = if self.stage() == Stage::Vertex {
            binding::VS_INT_BOOL_BINDING
        } else {
            binding::PS_INT_BOOL_BINDING
        };
        let arr = self.b.t_array_unique(self.t.iv4, binding::INT_CONSTANTS);
        self.b
            .decorate(arr, decoration::ARRAY_STRIDE, &[binding::CONSTANT_STRIDE]);
        let st = self.b.t_struct_unique(&[arr, self.t.u32]);
        self.b.decorate(st, decoration::BLOCK, &[]);
        self.b.member_decorate(st, 0, decoration::OFFSET, &[0]);
        self.b
            .member_decorate(st, 1, decoration::OFFSET, &[binding::BOOL_MASK_OFFSET]);
        self.b.name(st, "IntBoolConstants");
        self.b.member_name(st, 0, "i");
        self.b.member_name(st, 1, "b");
        let ptr = self.b.t_pointer(storage::UNIFORM, st);
        let v = self.b.global_variable(ptr, storage::UNIFORM, None);
        self.b
            .decorate(v, decoration::DESCRIPTOR_SET, &[binding::CONSTANT_SET]);
        self.b.decorate(v, decoration::BINDING, &[bind]);
        self.b.name(v, "ib");
        self.int_bool_ubo = Some(v);
        v
    }

    fn push_block(&mut self) -> Id {
        if let Some(v) = self.push {
            return v;
        }
        let st = self.b.t_struct_unique(&[self.t.v4]);
        self.b.decorate(st, decoration::BLOCK, &[]);
        self.b
            .member_decorate(st, 0, decoration::OFFSET, &[binding::POSITION_FIXUP_OFFSET]);
        self.b.name(st, "PushConstants");
        self.b.member_name(st, 0, "position_fixup");
        let ptr = self.b.t_pointer(storage::PUSH_CONSTANT, st);
        let v = self.b.global_variable(ptr, storage::PUSH_CONSTANT, None);
        self.b.name(v, "push");
        self.push = Some(v);
        v
    }

    fn temp_var(&mut self, n: u16) -> Id {
        if let Some(&v) = self.temps.get(&n) {
            return v;
        }
        let zero = self.zero4();
        let v = self.private_var(self.t.v4, zero, &format!("r{n}"));
        self.temps.insert(n, v);
        v
    }

    fn addr_var(&mut self) -> Id {
        if let Some(v) = self.addr {
            return v;
        }
        let zero = self.b.c_ivec4([0; 4]);
        let v = self.private_var(self.t.iv4, zero, "a0");
        self.addr = Some(v);
        v
    }

    fn loop_var(&mut self) -> Id {
        if let Some(v) = self.loop_reg {
            return v;
        }
        let zero = self.b.c_int(0);
        let v = self.private_var(self.t.i32, zero, "aL");
        self.loop_reg = Some(v);
        v
    }

    fn pred_var(&mut self) -> Id {
        if let Some(v) = self.pred {
            return v;
        }
        let f = self.b.c_bool(false);
        let init = self.b.c_composite(self.t.bv4, &[f, f, f, f]);
        let v = self.private_var(self.t.bv4, init, "p0");
        self.pred = Some(v);
        v
    }

    // ------------------------------------------------------------ value helpers

    fn load(&mut self, ty: Id, ptr: Id) -> Id {
        self.b.result(op::LOAD, ty, &[ptr])
    }

    fn store(&mut self, ptr: Id, value: Id) {
        self.b.emit(op::STORE, &[ptr, value]);
    }

    fn extract(&mut self, ty: Id, v: Id, comp: u32) -> Id {
        self.b.result(op::COMPOSITE_EXTRACT, ty, &[v, comp])
    }

    fn fx(&mut self, v: Id, comp: u32) -> Id {
        self.extract(self.t.f32, v, comp)
    }

    fn splat(&mut self, scalar: Id) -> Id {
        self.b.result(
            op::COMPOSITE_CONSTRUCT,
            self.t.v4,
            &[scalar, scalar, scalar, scalar],
        )
    }

    fn bsplat(&mut self, scalar: Id) -> Id {
        self.b.result(
            op::COMPOSITE_CONSTRUCT,
            self.t.bv4,
            &[scalar, scalar, scalar, scalar],
        )
    }

    fn vec4_of(&mut self, parts: [Id; 4]) -> Id {
        self.b.result(op::COMPOSITE_CONSTRUCT, self.t.v4, &parts)
    }

    fn shuffle(&mut self, ty: Id, a: Id, b: Id, comps: &[u32]) -> Id {
        let mut operands = vec![a, b];
        operands.extend_from_slice(comps);
        self.b.result(op::VECTOR_SHUFFLE, ty, &operands)
    }

    fn truncate(&mut self, v: Id, n: u32) -> Id {
        match n {
            2 => self.shuffle(self.t.v2, v, v, &[0, 1]),
            3 => self.shuffle(self.t.v3, v, v, &[0, 1, 2]),
            _ => v,
        }
    }

    fn bin(&mut self, opcode: u16, ty: Id, a: Id, b: Id) -> Id {
        self.b.result(opcode, ty, &[a, b])
    }

    fn select(&mut self, ty: Id, cond: Id, a: Id, b: Id) -> Id {
        self.b.result(op::SELECT, ty, &[cond, a, b])
    }

    fn dot(&mut self, a: Id, b: Id, n: u32) -> Id {
        let a = self.truncate(a, n);
        let b = self.truncate(b, n);
        self.bin(op::DOT, self.t.f32, a, b)
    }

    /// `log2(|x|)` with `log2(0) = -inf` made explicit (`GLSL.std.450`
    /// leaves `Log2` of zero undefined).
    fn log2_abs(&mut self, x: Id) -> Id {
        let a = self.b.ext(self.t.f32, glsl::FABS, &[x]);
        let l = self.b.ext(self.t.f32, glsl::LOG2, &[a]);
        let zero = self.b.c_float(0.0);
        let is_zero = self.bin(op::F_ORD_EQUAL, self.t.bool, a, zero);
        let ninf = self.b.c_float(f32::NEG_INFINITY);
        self.select(self.t.f32, is_zero, ninf, l)
    }

    /// Direct3D 10-style saturate: NaN becomes 0.
    fn saturate(&mut self, v: Id) -> Id {
        let one = self.b.c_vec4_splat(1.0);
        let zero = self.zero4();
        let lt1 = self.bin(op::F_ORD_LESS_THAN, self.t.bv4, v, one);
        let m = self.select(self.t.v4, lt1, v, one);
        let gt0 = self.bin(op::F_ORD_GREATER_THAN, self.t.bv4, v, zero);
        self.select(self.t.v4, gt0, m, zero)
    }

    fn compare_op(control: u8, offset: usize) -> Result<u16, Error> {
        Ok(match control & 0x7 {
            1 => op::F_ORD_GREATER_THAN,
            2 => op::F_ORD_EQUAL,
            3 => op::F_ORD_GREATER_THAN_EQUAL,
            4 => op::F_ORD_LESS_THAN,
            5 => op::F_UNORD_NOT_EQUAL,
            6 => op::F_ORD_LESS_THAN_EQUAL,
            _ => return Err(invalid(offset, "comparison code 0 or 7")),
        })
    }

    // ------------------------------------------------------------- operand reads

    fn rel_index(&mut self, rel: RelAddr, offset: usize) -> Result<Id, Error> {
        match rel.reg.kind {
            RegType::Addr => {
                if self.stage() != Stage::Vertex {
                    return Err(invalid(offset, "a0 relative addressing in a pixel shader"));
                }
                let a = self.addr_var();
                let v = self.load(self.t.iv4, a);
                Ok(self.extract(self.t.i32, v, u32::from(rel.component)))
            }
            RegType::Loop => {
                let l = self.loop_var();
                Ok(self.load(self.t.i32, l))
            }
            _ => Err(invalid(offset, "bad relative address register")),
        }
    }

    fn compose_input(&mut self, num: u16, offset: usize) -> Result<Id, Error> {
        let decls: Vec<InputDecl> = self
            .inputs
            .iter()
            .filter(|d| d.reg.num == num)
            .copied()
            .collect();
        let Some(first) = decls.first() else {
            return Err(invalid(offset, format!("input v{num} read without a dcl")));
        };
        let mut value = self.load(self.t.v4, first.var);
        for d in &decls[1..] {
            let next = self.load(self.t.v4, d.var);
            let comps: Vec<u32> = (0..4u32)
                .map(|i| if d.mask & (1 << i) != 0 { 4 + i } else { i })
                .collect();
            value = self.shuffle(self.t.v4, value, next, &comps);
        }
        Ok(value)
    }

    fn read_float_const(
        &mut self,
        num: u16,
        rel: Option<RelAddr>,
        offset: usize,
    ) -> Result<Id, Error> {
        let Some(rel) = rel else {
            if let Some(&bits) = self.defs_f.get(&num) {
                return Ok(self.b.c_vec4_bits(bits));
            }
            self.float_read.insert(num);
            let ubo = self.float_ubo();
            let ptr_ty = self.b.t_pointer(storage::UNIFORM, self.t.v4);
            let zero_i = self.b.c_int(0);
            let idx = self.b.c_int(i32::from(num));
            let p = self.b.result(op::ACCESS_CHAIN, ptr_ty, &[ubo, zero_i, idx]);
            return Ok(self.load(self.t.v4, p));
        };
        self.float_relative = true;
        let limit = self.register_limit(RegType::Const).unwrap_or(0);
        let r = self.rel_index(rel, offset)?;
        let base = self.b.c_int(i32::from(num));
        let idx = self.bin(op::I_ADD, self.t.i32, base, r);
        let zero_i = self.b.c_int(0);
        let n = self.b.c_int(i32::from(limit));
        let last = self.b.c_int(i32::from(limit) - 1);
        let ge0 = self.bin(op::S_GREATER_THAN_EQUAL, self.t.bool, idx, zero_i);
        let ltn = self.bin(op::S_LESS_THAN, self.t.bool, idx, n);
        let in_range = self.bin(op::LOGICAL_AND, self.t.bool, ge0, ltn);
        let clamped = self.b.ext(self.t.i32, glsl::SCLAMP, &[idx, zero_i, last]);
        let ubo = self.float_ubo();
        let ptr_ty = self.b.t_pointer(storage::UNIFORM, self.t.v4);
        let p = self
            .b
            .result(op::ACCESS_CHAIN, ptr_ty, &[ubo, zero_i, clamped]);
        let loaded = self.load(self.t.v4, p);
        let cond = self.bsplat(in_range);
        let zero = self.zero4();
        let mut value = self.select(self.t.v4, cond, loaded, zero);
        let defs: Vec<(u16, [u32; 4])> = self.defs_f.iter().map(|(&k, &v)| (k, v)).collect();
        for (reg, bits) in defs {
            let k = self.b.c_int(i32::from(reg));
            let eq = self.bin(op::I_EQUAL, self.t.bool, idx, k);
            let c = self.bsplat(eq);
            let lit = self.b.c_vec4_bits(bits);
            value = self.select(self.t.v4, c, lit, value);
        }
        Ok(value)
    }

    /// Read a float register before swizzle and modifier.
    fn load_raw(
        &mut self,
        reg: Register,
        rel: Option<RelAddr>,
        add: u16,
        offset: usize,
    ) -> Result<Id, Error> {
        let reg = Register {
            kind: reg.kind,
            num: reg.num.saturating_add(add),
        };
        self.check_range(reg, offset)?;
        if rel.is_some() && !matches!(reg.kind, RegType::Const | RegType::Input) {
            return Err(invalid(
                offset,
                format!("relative addressing on register file {:?}", reg.kind),
            ));
        }
        match reg.kind {
            RegType::Temp => {
                let v = self.temp_var(reg.num);
                Ok(self.load(self.t.v4, v))
            }
            RegType::Input => {
                if let Some(arr) = self.input_array {
                    let base = self.b.c_int(i32::from(reg.num));
                    let idx = match rel {
                        Some(rel) => {
                            let r = self.rel_index(rel, offset)?;
                            let sum = self.bin(op::I_ADD, self.t.i32, base, r);
                            let zero = self.b.c_int(0);
                            let last = self.b.c_int(i32::from(self.input_registers()) - 1);
                            self.b.ext(self.t.i32, glsl::SCLAMP, &[sum, zero, last])
                        }
                        None => base,
                    };
                    let ptr_ty = self.b.t_pointer(storage::PRIVATE, self.t.v4);
                    let p = self.b.result(op::ACCESS_CHAIN, ptr_ty, &[arr, idx]);
                    Ok(self.load(self.t.v4, p))
                } else {
                    self.compose_input(reg.num, offset)
                }
            }
            RegType::Const => self.read_float_const(reg.num, rel, offset),
            RegType::MiscType => {
                if reg.num == MISC_POSITION {
                    if self.frag_coord.is_none() {
                        return Err(invalid(offset, "vPos read without a dcl"));
                    }
                    let fc = self.frag_coord_var();
                    let v = self.load(self.t.v4, fc);
                    // Direct3D 9 pixel centres are integers; Vulkan's are at .5.
                    let half = self
                        .b
                        .c_vec4_bits([0.5f32.to_bits(), 0.5f32.to_bits(), 0, 0]);
                    Ok(self.bin(op::F_SUB, self.t.v4, v, half))
                } else if reg.num == MISC_FACE {
                    if self.front_facing.is_none() {
                        return Err(invalid(offset, "vFace read without a dcl"));
                    }
                    let ff = self.front_facing_var();
                    let f = self.load(self.t.bool, ff);
                    let one = self.b.c_float(1.0);
                    let minus = self.b.c_float(-1.0);
                    let s = self.select(self.t.f32, f, one, minus);
                    Ok(self.splat(s))
                } else {
                    Err(invalid(offset, "unknown misc register"))
                }
            }
            other => Err(unsupported(
                offset,
                format!("reading register file {other:?} as a float source"),
            )),
        }
    }

    fn apply_swizzle(&mut self, v: Id, sw: [u8; 4]) -> Id {
        if sw == IDENTITY_SWIZZLE {
            v
        } else {
            let comps = sw.map(u32::from);
            self.shuffle(self.t.v4, v, v, &comps)
        }
    }

    fn apply_modifier(&mut self, v: Id, m: SrcMod, offset: usize) -> Result<Id, Error> {
        let t = self.t.v4;
        Ok(match m {
            SrcMod::None => v,
            SrcMod::Neg => self.b.result(op::F_NEGATE, t, &[v]),
            SrcMod::Abs => self.b.ext(t, glsl::FABS, &[v]),
            SrcMod::AbsNeg => {
                let a = self.b.ext(t, glsl::FABS, &[v]);
                self.b.result(op::F_NEGATE, t, &[a])
            }
            SrcMod::Bias | SrcMod::BiasNeg => {
                let half = self.b.c_vec4_splat(0.5);
                let r = self.bin(op::F_SUB, t, v, half);
                if m == SrcMod::BiasNeg {
                    self.b.result(op::F_NEGATE, t, &[r])
                } else {
                    r
                }
            }
            SrcMod::Sign | SrcMod::SignNeg => {
                let two = self.b.c_vec4_splat(2.0);
                let one = self.b.c_vec4_splat(1.0);
                let d = self.bin(op::F_MUL, t, v, two);
                let r = self.bin(op::F_SUB, t, d, one);
                if m == SrcMod::SignNeg {
                    self.b.result(op::F_NEGATE, t, &[r])
                } else {
                    r
                }
            }
            SrcMod::Comp => {
                let one = self.b.c_vec4_splat(1.0);
                self.bin(op::F_SUB, t, one, v)
            }
            SrcMod::X2 | SrcMod::X2Neg => {
                let two = self.b.c_vec4_splat(2.0);
                let r = self.bin(op::F_MUL, t, v, two);
                if m == SrcMod::X2Neg {
                    self.b.result(op::F_NEGATE, t, &[r])
                } else {
                    r
                }
            }
            SrcMod::Dz | SrcMod::Dw => {
                return Err(unsupported(
                    offset,
                    "source modifier _dz/_dw (pixel shader 1.4 only)",
                ));
            }
            SrcMod::Not => return Err(invalid(offset, "! modifier on a float source")),
        })
    }

    fn load_src_at(&mut self, s: &SrcParam, add: u16, offset: usize) -> Result<Id, Error> {
        let v = self.load_raw(s.reg, s.relative, add, offset)?;
        let v = self.apply_swizzle(v, s.swizzle);
        self.apply_modifier(v, s.modifier, offset)
    }

    fn src(&mut self, ins: &Instruction, i: usize) -> Result<Id, Error> {
        let s = *ins
            .src
            .get(i)
            .ok_or_else(|| invalid(ins.offset, "missing source operand"))?;
        self.load_src_at(&s, 0, ins.offset)
    }

    /// Scalar from the first selected component of a source.
    fn src_scalar(&mut self, ins: &Instruction, i: usize) -> Result<Id, Error> {
        let v = self.src(ins, i)?;
        Ok(self.fx(v, 0))
    }

    fn load_int_const(&mut self, num: u16) -> Id {
        if let Some(&v) = self.defs_i.get(&num) {
            return self.b.c_ivec4(v);
        }
        self.int_read.insert(num);
        let ubo = self.int_bool_ubo();
        let ptr_ty = self.b.t_pointer(storage::UNIFORM, self.t.iv4);
        let zero = self.b.c_int(0);
        let idx = self.b.c_int(i32::from(num));
        let p = self.b.result(op::ACCESS_CHAIN, ptr_ty, &[ubo, zero, idx]);
        self.load(self.t.iv4, p)
    }

    fn load_bool_const(&mut self, num: u16) -> Id {
        if let Some(&v) = self.defs_b.get(&num) {
            return self.b.c_bool(v);
        }
        self.bool_read.insert(num);
        let ubo = self.int_bool_ubo();
        let ptr_ty = self.b.t_pointer(storage::UNIFORM, self.t.u32);
        let one_i = self.b.c_int(1);
        let p = self.b.result(op::ACCESS_CHAIN, ptr_ty, &[ubo, one_i]);
        let mask = self.load(self.t.u32, p);
        let sh = self.b.c_uint(u32::from(num));
        let shifted = self.bin(op::SHIFT_RIGHT_LOGICAL, self.t.u32, mask, sh);
        let one = self.b.c_uint(1);
        let bit = self.bin(op::BITWISE_AND, self.t.u32, shifted, one);
        let zero = self.b.c_uint(0);
        self.bin(op::I_NOT_EQUAL, self.t.bool, bit, zero)
    }

    /// The predicate register as a swizzled, possibly negated `bvec4`.
    fn pred_vec(&mut self, p: &SrcParam, offset: usize) -> Result<Id, Error> {
        if p.reg.kind != RegType::Predicate || p.relative.is_some() {
            return Err(invalid(offset, "predicate operand is not p0"));
        }
        self.check_range(p.reg, offset)?;
        let var = self.pred_var();
        let v = self.load(self.t.bv4, var);
        let v = if p.swizzle == IDENTITY_SWIZZLE {
            v
        } else {
            let comps = p.swizzle.map(u32::from);
            self.shuffle(self.t.bv4, v, v, &comps)
        };
        match p.modifier {
            SrcMod::None => Ok(v),
            SrcMod::Not => Ok(self.b.result(op::LOGICAL_NOT, self.t.bv4, &[v])),
            _ => Err(invalid(offset, "bad modifier on the predicate")),
        }
    }

    /// A boolean condition from `b#` or `p0.c` (with optional `!`).
    fn bool_condition(&mut self, s: &SrcParam, offset: usize) -> Result<Id, Error> {
        match s.reg.kind {
            RegType::ConstBool => {
                self.check_range(s.reg, offset)?;
                let v = self.load_bool_const(s.reg.num);
                match s.modifier {
                    SrcMod::None => Ok(v),
                    SrcMod::Not => Ok(self.b.result(op::LOGICAL_NOT, self.t.bool, &[v])),
                    _ => Err(invalid(offset, "bad modifier on a boolean")),
                }
            }
            RegType::Predicate => {
                let v = self.pred_vec(s, offset)?;
                Ok(self.extract(self.t.bool, v, 0))
            }
            _ => Err(invalid(offset, "condition is not a boolean or predicate")),
        }
    }

    // ------------------------------------------------------------------ writes

    /// Store `value` into `ptr` under a write mask and an optional
    /// per-component predicate.
    fn store_masked(
        &mut self,
        ptr: Id,
        ty: Id,
        value: Id,
        mask: u8,
        pred: Option<&SrcParam>,
        offset: usize,
    ) -> Result<(), Error> {
        let mask = mask & 0xF;
        if mask == 0 {
            return Ok(());
        }
        if mask == 0xF && pred.is_none() {
            self.store(ptr, value);
            return Ok(());
        }
        let old = self.load(ty, ptr);
        let mut new = value;
        if let Some(p) = pred {
            let cond = self.pred_vec(p, offset)?;
            new = self.select(ty, cond, value, old);
        }
        let merged = if mask == 0xF {
            new
        } else {
            let comps: Vec<u32> = (0..4u32)
                .map(|i| if mask & (1 << i) != 0 { i } else { 4 + i })
                .collect();
            self.shuffle(ty, new, old, &comps)
        };
        self.store(ptr, merged);
        Ok(())
    }

    fn dst_pointer(&mut self, d: &DstParam, offset: usize) -> Result<Id, Error> {
        self.check_range(d.reg, offset)?;
        if d.relative.is_some() && d.reg.kind != RegType::Output {
            return Err(invalid(offset, "relative addressing on a destination"));
        }
        match d.reg.kind {
            RegType::Temp => Ok(self.temp_var(d.reg.num)),
            RegType::Output => {
                if let Some(arr) = self.output_array {
                    let base = self.b.c_int(i32::from(d.reg.num));
                    let idx = match d.relative {
                        Some(rel) => {
                            let r = self.rel_index(rel, offset)?;
                            let sum = self.bin(op::I_ADD, self.t.i32, base, r);
                            let zero = self.b.c_int(0);
                            let last = self.b.c_int(i32::from(VS_OUTPUT_REGISTERS) - 1);
                            self.b.ext(self.t.i32, glsl::SCLAMP, &[sum, zero, last])
                        }
                        None => base,
                    };
                    let ptr_ty = self.b.t_pointer(storage::PRIVATE, self.t.v4);
                    Ok(self.b.result(op::ACCESS_CHAIN, ptr_ty, &[arr, idx]))
                } else {
                    self.output_stage.get(&d.reg.num).copied().ok_or_else(|| {
                        invalid(offset, format!("o{} written without a dcl", d.reg.num))
                    })
                }
            }
            RegType::ColorOut => Ok(self.color_output(d.reg.num).0),
            RegType::DepthOut => Ok(self.depth_output().0),
            other => Err(unsupported(
                offset,
                format!("writing register file {other:?} with this instruction"),
            )),
        }
    }

    fn write_dst(&mut self, ins: &Instruction, value: Id, mask_limit: u8) -> Result<(), Error> {
        let d = ins
            .dst
            .ok_or_else(|| invalid(ins.offset, "missing destination"))?;
        if d.shift != 0 {
            return Err(unsupported(
                ins.offset,
                "destination shift scale (pixel shader 1.x only)",
            ));
        }
        let value = if d.saturate {
            self.saturate(value)
        } else {
            value
        };
        let ptr = self.dst_pointer(&d, ins.offset)?;
        self.store_masked(
            ptr,
            self.t.v4,
            value,
            d.write_mask & mask_limit,
            ins.predicate.as_ref(),
            ins.offset,
        )
    }

    // ---------------------------------------------------------- program walk

    fn label_fn(&mut self, n: u16) -> Id {
        if let Some(&f) = self.labels.get(&n) {
            return f;
        }
        let f = self.b.id();
        self.b.name(f, &format!("l{n}"));
        self.labels.insert(n, f);
        f
    }

    fn close_function(&mut self, cur: Current, offset: usize) -> Result<(), Error> {
        if !self.flow.is_empty() {
            return Err(invalid(offset, "function ends inside flow control"));
        }
        if cur == Current::Main {
            self.emit_main_epilogue();
        }
        self.b.end_function();
        Ok(())
    }

    fn body(&mut self) -> Result<(), Error> {
        let sh = self.sh;
        self.b
            .begin_function(self.main, self.t.void, self.t.fn_void);
        self.emit_main_prologue()?;
        let mut cur = Current::Main;
        for ins in &sh.instructions {
            match ins.opcode {
                Opcode::Dcl | Opcode::Def | Opcode::DefI | Opcode::DefB | Opcode::Nop => {}
                Opcode::Label => {
                    if !self.flow.is_empty() {
                        return Err(invalid(ins.offset, "label inside flow control"));
                    }
                    let s = ins
                        .src
                        .first()
                        .ok_or_else(|| invalid(ins.offset, "label without operand"))?;
                    if s.reg.kind != RegType::Label {
                        return Err(invalid(ins.offset, "label operand is not l#"));
                    }
                    if cur != Current::None {
                        self.close_function(cur, ins.offset)?;
                    }
                    if !self.defined_labels.insert(s.reg.num) {
                        return Err(invalid(ins.offset, "label defined twice"));
                    }
                    let f = self.label_fn(s.reg.num);
                    self.b.begin_function(f, self.t.void, self.t.fn_void);
                    self.current_label = Some(s.reg.num);
                    cur = Current::Sub;
                }
                Opcode::Ret => {
                    if cur == Current::None {
                        return Err(invalid(ins.offset, "ret outside a function"));
                    }
                    if self.flow.is_empty() {
                        self.close_function(cur, ins.offset)?;
                        cur = Current::None;
                    } else {
                        if cur == Current::Main {
                            self.emit_main_epilogue();
                        }
                        self.b.terminate(op::RETURN, &[]);
                    }
                }
                _ => {
                    if cur == Current::None {
                        return Err(invalid(ins.offset, "instruction outside any function"));
                    }
                    self.instruction(ins)?;
                }
            }
        }
        if cur != Current::None {
            self.close_function(cur, sh.end_offset)?;
        }
        if let Some(missing) = self
            .labels
            .keys()
            .find(|l| !self.defined_labels.contains(l))
        {
            return Err(invalid(
                sh.end_offset,
                format!("call to undefined label l{missing}"),
            ));
        }
        self.check_recursion()
    }

    fn check_recursion(&self) -> Result<(), Error> {
        // Depth-first search for a cycle in the call graph.
        fn visit(
            n: u16,
            calls: &BTreeMap<Option<u16>, BTreeSet<u16>>,
            state: &mut BTreeMap<u16, u8>,
        ) -> bool {
            match state.get(&n) {
                Some(1) => return true,
                Some(2) => return false,
                _ => {}
            }
            state.insert(n, 1);
            if let Some(next) = calls.get(&Some(n)) {
                for &m in next {
                    if visit(m, calls, state) {
                        return true;
                    }
                }
            }
            state.insert(n, 2);
            false
        }
        let mut state = BTreeMap::new();
        for &l in &self.defined_labels {
            if visit(l, &self.calls, &mut state) {
                return Err(invalid(self.sh.end_offset, "recursive subroutine call"));
            }
        }
        Ok(())
    }

    fn emit_main_prologue(&mut self) -> Result<(), Error> {
        if let Some(arr) = self.input_array {
            let regs: BTreeSet<u16> = self.inputs.iter().map(|d| d.reg.num).collect();
            for r in regs {
                let v = self.compose_input(r, 0)?;
                let idx = self.b.c_int(i32::from(r));
                let ptr_ty = self.b.t_pointer(storage::PRIVATE, self.t.v4);
                let p = self.b.result(op::ACCESS_CHAIN, ptr_ty, &[arr, idx]);
                self.store(p, v);
            }
        }
        Ok(())
    }

    fn output_value(&mut self, reg: u16) -> Id {
        if let Some(arr) = self.output_array {
            let idx = self.b.c_int(i32::from(reg));
            let ptr_ty = self.b.t_pointer(storage::PRIVATE, self.t.v4);
            let p = self.b.result(op::ACCESS_CHAIN, ptr_ty, &[arr, idx]);
            self.load(self.t.v4, p)
        } else {
            let var = self.output_stage[&reg];
            self.load(self.t.v4, var)
        }
    }

    /// Copy staged outputs to the interface variables.
    fn emit_main_epilogue(&mut self) {
        let outputs = self.outputs.clone();
        for o in outputs {
            let v = self.output_value(o.reg);
            match o.kind {
                OutputKind::Position => {
                    let pos = if self.opts.position_fixup {
                        let push = self.push_block();
                        let ptr_ty = self.b.t_pointer(storage::PUSH_CONSTANT, self.t.v4);
                        let zero = self.b.c_int(0);
                        let p = self.b.result(op::ACCESS_CHAIN, ptr_ty, &[push, zero]);
                        let fix = self.load(self.t.v4, p);
                        // xy * fix.xy + fix.zw * w; z and w unchanged.
                        let w = self.fx(v, 3);
                        let ws = self.splat(w);
                        let one = self.b.c_vec4_splat(1.0);
                        let scale = self.shuffle(self.t.v4, fix, one, &[0, 1, 4, 5]);
                        let zero4 = self.zero4();
                        let offs = self.shuffle(self.t.v4, fix, zero4, &[2, 3, 4, 5]);
                        let scaled = self.bin(op::F_MUL, self.t.v4, v, scale);
                        let shift = self.bin(op::F_MUL, self.t.v4, offs, ws);
                        self.bin(op::F_ADD, self.t.v4, scaled, shift)
                    } else {
                        v
                    };
                    self.store(o.var, pos);
                }
                OutputKind::PointSize => {
                    let comp = mask_components(o.mask).next().unwrap_or(0);
                    let s = self.fx(v, comp);
                    self.store(o.var, s);
                }
                OutputKind::Varying => self.store(o.var, v),
            }
        }
        let colors: Vec<(Id, Id)> = self.color_stage.values().copied().collect();
        for (stage, var) in colors {
            let v = self.load(self.t.v4, stage);
            self.store(var, v);
        }
        if let Some((stage, var)) = self.depth_stage {
            let v = self.load(self.t.v4, stage);
            let x = self.fx(v, 0);
            self.store(var, x);
        }
    }

    fn require_pixel(&self, ins: &Instruction) -> Result<(), Error> {
        if self.stage() == Stage::Pixel {
            Ok(())
        } else {
            Err(invalid(
                ins.offset,
                format!("{} is only valid in pixel shaders", ins.opcode.name()),
            ))
        }
    }

    fn instruction(&mut self, ins: &Instruction) -> Result<(), Error> {
        use Opcode as O;
        let flow_op = matches!(
            ins.opcode,
            O::If
                | O::Ifc
                | O::Else
                | O::EndIf
                | O::Loop
                | O::EndLoop
                | O::Rep
                | O::EndRep
                | O::Break
                | O::BreakC
                | O::BreakP
                | O::Call
                | O::CallNz
        );
        if flow_op && ins.predicate.is_some() {
            return Err(unsupported(
                ins.offset,
                "predicated flow-control instruction",
            ));
        }
        match ins.opcode {
            O::If => {
                let s = ins.src[0];
                let c = self.bool_condition(&s, ins.offset)?;
                self.begin_if(c);
                Ok(())
            }
            O::Ifc => {
                let a = self.src_scalar(ins, 0)?;
                let b = self.src_scalar(ins, 1)?;
                let opc = Self::compare_op(ins.control, ins.offset)?;
                let c = self.bin(opc, self.t.bool, a, b);
                self.begin_if(c);
                Ok(())
            }
            O::Else => self.else_(ins.offset),
            O::EndIf => self.endif(ins.offset),
            O::Loop | O::Rep => self.begin_loop(ins),
            O::EndLoop | O::EndRep => self.end_loop(ins),
            O::Break => {
                let merge = self.innermost_loop_merge(ins.offset)?;
                self.b.terminate(op::BRANCH, &[merge]);
                Ok(())
            }
            O::BreakC => {
                let a = self.src_scalar(ins, 0)?;
                let b = self.src_scalar(ins, 1)?;
                let opc = Self::compare_op(ins.control, ins.offset)?;
                let c = self.bin(opc, self.t.bool, a, b);
                self.break_if(c, ins.offset)
            }
            O::BreakP => {
                let s = ins.src[0];
                if s.reg.kind != RegType::Predicate {
                    return Err(invalid(ins.offset, "breakp operand is not p0"));
                }
                let c = self.bool_condition(&s, ins.offset)?;
                self.break_if(c, ins.offset)
            }
            O::Call => {
                let f = self.call_target(ins)?;
                self.b.result(op::FUNCTION_CALL, self.t.void, &[f]);
                Ok(())
            }
            O::CallNz => {
                let f = self.call_target(ins)?;
                let s = ins.src[1];
                let c = self.bool_condition(&s, ins.offset)?;
                let then = self.b.id();
                let after = self.b.id();
                self.b.merge_and_terminate(
                    op::SELECTION_MERGE,
                    &[after, 0],
                    op::BRANCH_CONDITIONAL,
                    &[c, then, after],
                );
                self.b.label(then);
                self.b.result(op::FUNCTION_CALL, self.t.void, &[f]);
                self.b.terminate(op::BRANCH, &[after]);
                self.b.label(after);
                Ok(())
            }
            O::TexKill => self.texkill(ins),
            O::Tex | O::TexLdl | O::TexLdd => self.texture(ins),
            O::Mova => self.mova(ins),
            O::Setp => self.setp(ins),
            _ => self.arithmetic(ins),
        }
    }

    // ------------------------------------------------------------ flow control

    fn begin_if(&mut self, cond: Id) {
        let merge = self.b.id();
        let then = self.b.id();
        let else_label = self.b.id();
        self.b.merge_and_terminate(
            op::SELECTION_MERGE,
            &[merge, 0],
            op::BRANCH_CONDITIONAL,
            &[cond, then, else_label],
        );
        self.b.label(then);
        self.flow.push(Flow::If {
            merge,
            else_label,
            has_else: false,
        });
    }

    fn else_(&mut self, offset: usize) -> Result<(), Error> {
        match self.flow.last_mut() {
            Some(Flow::If {
                merge,
                else_label,
                has_else,
            }) if !*has_else => {
                *has_else = true;
                let (merge, else_label) = (*merge, *else_label);
                self.b.branch_if_open(merge);
                self.b.label(else_label);
                Ok(())
            }
            _ => Err(invalid(offset, "else without a matching if")),
        }
    }

    fn endif(&mut self, offset: usize) -> Result<(), Error> {
        match self.flow.pop() {
            Some(Flow::If {
                merge,
                else_label,
                has_else,
            }) => {
                self.b.branch_if_open(merge);
                if !has_else {
                    self.b.label(else_label);
                    self.b.terminate(op::BRANCH, &[merge]);
                }
                self.b.label(merge);
                Ok(())
            }
            _ => Err(invalid(offset, "endif without a matching if")),
        }
    }

    fn begin_loop(&mut self, ins: &Instruction) -> Result<(), Error> {
        let (count, loop_reg) = if ins.opcode == Opcode::Loop {
            let al = ins.src[0];
            let i = ins.src[1];
            if al.reg.kind != RegType::Loop || i.reg.kind != RegType::ConstInt {
                return Err(invalid(ins.offset, "loop operands are not aL, i#"));
            }
            self.check_range(i.reg, ins.offset)?;
            let iv = self.load_int_const(i.reg.num);
            let count = self.extract(self.t.i32, iv, 0);
            let init = self.extract(self.t.i32, iv, 1);
            let step = self.extract(self.t.i32, iv, 2);
            let zero = self.b.c_int(0);
            let saved = self.private_var(self.t.i32, zero, "aL_saved");
            let l = self.loop_var();
            let old = self.load(self.t.i32, l);
            self.store(saved, old);
            self.store(l, init);
            (count, Some((step, saved)))
        } else {
            let i = ins.src[0];
            if i.reg.kind != RegType::ConstInt {
                return Err(invalid(ins.offset, "rep operand is not i#"));
            }
            self.check_range(i.reg, ins.offset)?;
            let iv = self.load_int_const(i.reg.num);
            (self.extract(self.t.i32, iv, 0), None)
        };
        let zero = self.b.c_int(0);
        let counter = self.private_var(self.t.i32, zero, "loop_counter");
        self.store(counter, zero);
        let header = self.b.id();
        let body_label = self.b.id();
        let continue_label = self.b.id();
        let merge = self.b.id();
        self.b.label(header);
        let c = self.load(self.t.i32, counter);
        let ok = self.bin(op::S_LESS_THAN, self.t.bool, c, count);
        self.b.merge_and_terminate(
            op::LOOP_MERGE,
            &[merge, continue_label, 0],
            op::BRANCH_CONDITIONAL,
            &[ok, body_label, merge],
        );
        self.b.label(body_label);
        self.flow.push(Flow::Loop {
            header,
            merge,
            cont: continue_label,
            counter,
            loop_reg,
        });
        Ok(())
    }

    fn end_loop(&mut self, ins: &Instruction) -> Result<(), Error> {
        let Some(Flow::Loop {
            header,
            merge,
            cont,
            counter,
            loop_reg,
        }) = self.flow.pop()
        else {
            return Err(invalid(ins.offset, "endloop/endrep without a loop"));
        };
        let is_loop = ins.opcode == Opcode::EndLoop;
        if is_loop != loop_reg.is_some() {
            return Err(invalid(
                ins.offset,
                "loop/rep closed by the wrong instruction",
            ));
        }
        self.b.branch_if_open(cont);
        self.b.label(cont);
        let c = self.load(self.t.i32, counter);
        let one = self.b.c_int(1);
        let next = self.bin(op::I_ADD, self.t.i32, c, one);
        self.store(counter, next);
        if let Some((step, _)) = loop_reg {
            let l = self.loop_var();
            let a = self.load(self.t.i32, l);
            let n = self.bin(op::I_ADD, self.t.i32, a, step);
            self.store(l, n);
        }
        self.b.terminate(op::BRANCH, &[header]);
        self.b.label(merge);
        if let Some((_, saved)) = loop_reg {
            let l = self.loop_var();
            let old = self.load(self.t.i32, saved);
            self.store(l, old);
        }
        Ok(())
    }

    fn innermost_loop_merge(&self, offset: usize) -> Result<Id, Error> {
        self.flow
            .iter()
            .rev()
            .find_map(|f| match f {
                Flow::Loop { merge, .. } => Some(*merge),
                Flow::If { .. } => None,
            })
            .ok_or_else(|| invalid(offset, "break outside a loop"))
    }

    fn break_if(&mut self, cond: Id, offset: usize) -> Result<(), Error> {
        let merge = self.innermost_loop_merge(offset)?;
        let brk = self.b.id();
        let after = self.b.id();
        self.b.merge_and_terminate(
            op::SELECTION_MERGE,
            &[after, 0],
            op::BRANCH_CONDITIONAL,
            &[cond, brk, after],
        );
        self.b.label(brk);
        self.b.terminate(op::BRANCH, &[merge]);
        self.b.label(after);
        Ok(())
    }

    fn call_target(&mut self, ins: &Instruction) -> Result<Id, Error> {
        let s = ins.src[0];
        if s.reg.kind != RegType::Label {
            return Err(invalid(ins.offset, "call operand is not l#"));
        }
        self.check_range(s.reg, ins.offset)?;
        self.calls
            .entry(self.current_label)
            .or_default()
            .insert(s.reg.num);
        Ok(self.label_fn(s.reg.num))
    }

    // ---------------------------------------------------------------- texture

    fn texkill(&mut self, ins: &Instruction) -> Result<(), Error> {
        self.require_pixel(ins)?;
        let d = ins
            .dst
            .ok_or_else(|| invalid(ins.offset, "texkill without operand"))?;
        let v = self.load_raw(d.reg, d.relative, 0, ins.offset)?;
        let zero = self.zero4();
        let mut lt = self.bin(op::F_ORD_LESS_THAN, self.t.bv4, v, zero);
        if let Some(p) = ins.predicate {
            let pv = self.pred_vec(&p, ins.offset)?;
            lt = self.bin(op::LOGICAL_AND, self.t.bv4, lt, pv);
        }
        let mask = if d.write_mask == 0 { 0xF } else { d.write_mask };
        let mut cond = None;
        for c in mask_components(mask) {
            let x = self.extract(self.t.bool, lt, c);
            cond = Some(match cond {
                None => x,
                Some(prev) => self.bin(op::LOGICAL_OR, self.t.bool, prev, x),
            });
        }
        let Some(cond) = cond else { return Ok(()) };
        let kill = self.b.id();
        let after = self.b.id();
        self.b.merge_and_terminate(
            op::SELECTION_MERGE,
            &[after, 0],
            op::BRANCH_CONDITIONAL,
            &[cond, kill, after],
        );
        self.b.label(kill);
        self.b.terminate(op::KILL, &[]);
        self.b.label(after);
        Ok(())
    }

    fn texture(&mut self, ins: &Instruction) -> Result<(), Error> {
        let s = *ins
            .src
            .get(1)
            .ok_or_else(|| invalid(ins.offset, "texture instruction without sampler"))?;
        if s.reg.kind != RegType::Sampler || s.relative.is_some() {
            return Err(invalid(ins.offset, "second operand is not a sampler"));
        }
        self.check_range(s.reg, ins.offset)?;
        if s.modifier != SrcMod::None {
            return Err(invalid(ins.offset, "modifier on a sampler operand"));
        }
        let decl = *self.samplers.get(&s.reg.num).ok_or_else(|| {
            invalid(
                ins.offset,
                format!("sampler s{} used without a dcl", s.reg.num),
            )
        })?;
        let n = match decl.dim {
            TextureDim::D2 => 2,
            TextureDim::Cube | TextureDim::D3 => 3,
        };
        let coord = self.src(ins, 0)?;
        let project = ins.opcode == Opcode::Tex && ins.control & TEXLD_PROJECT != 0;
        let bias = ins.opcode == Opcode::Tex && ins.control & TEXLD_BIAS != 0;
        if project && bias {
            return Err(invalid(
                ins.offset,
                "texld with both project and bias flags",
            ));
        }
        if ins.opcode != Opcode::Tex && ins.control & (TEXLD_PROJECT | TEXLD_BIAS) != 0 {
            return Err(invalid(ins.offset, "texldl/texldd with texld flags"));
        }
        let implicit = ins.opcode == Opcode::Tex;
        if implicit && self.stage() == Stage::Vertex {
            return Err(unsupported(
                ins.offset,
                "texld with implicit level of detail in a vertex shader (use texldl)",
            ));
        }
        let coord = if project {
            let w = self.fx(coord, 3);
            let ws = self.splat(w);
            self.bin(op::F_DIV, self.t.v4, coord, ws)
        } else {
            coord
        };
        let coord_n = self.truncate(coord, n);
        let mut operands: Vec<u32> = Vec::new();
        match ins.opcode {
            Opcode::Tex if bias => {
                let w = self.fx(coord, 3);
                operands.extend([image_operands::BIAS, w]);
            }
            Opcode::TexLdl => {
                let w = self.fx(coord, 3);
                operands.extend([image_operands::LOD, w]);
            }
            Opcode::TexLdd => {
                let dx = self.src(ins, 2)?;
                let dy = self.src(ins, 3)?;
                let dx = self.truncate(dx, n);
                let dy = self.truncate(dy, n);
                operands.extend([image_operands::GRAD, dx, dy]);
            }
            _ => {}
        }
        let image = self.b.t_image(
            match decl.dim {
                TextureDim::D2 => dim::D2,
                TextureDim::Cube => dim::CUBE,
                TextureDim::D3 => dim::D3,
            },
            decl.depth,
        );
        let sampled_ty = self.b.t_sampled_image(image);
        let si = self.load(sampled_ty, decl.var);
        let result = if decl.depth {
            let dref = self.fx(coord, 2);
            let opc = if implicit {
                op::IMAGE_SAMPLE_DREF_IMPLICIT_LOD
            } else {
                op::IMAGE_SAMPLE_DREF_EXPLICIT_LOD
            };
            let mut all = vec![si, coord_n, dref];
            all.extend(operands);
            let r = self.b.result(opc, self.t.f32, &all);
            self.splat(r)
        } else {
            let opc = if implicit {
                op::IMAGE_SAMPLE_IMPLICIT_LOD
            } else {
                op::IMAGE_SAMPLE_EXPLICIT_LOD
            };
            let mut all = vec![si, coord_n];
            all.extend(operands);
            self.b.result(opc, self.t.v4, &all)
        };
        let result = self.apply_swizzle(result, s.swizzle);
        self.write_dst(ins, result, 0xF)
    }

    // ----------------------------------------------------------- p0, a0 writes

    fn mova(&mut self, ins: &Instruction) -> Result<(), Error> {
        let d = ins
            .dst
            .ok_or_else(|| invalid(ins.offset, "mova without destination"))?;
        if d.reg.kind != RegType::Addr || self.stage() != Stage::Vertex {
            return Err(invalid(ins.offset, "mova destination is not a0"));
        }
        self.check_range(d.reg, ins.offset)?;
        if d.saturate || d.shift != 0 {
            return Err(invalid(ins.offset, "modifier on a mova destination"));
        }
        let v = self.src(ins, 0)?;
        let half = self.b.c_vec4_splat(0.5);
        let sum = self.bin(op::F_ADD, self.t.v4, v, half);
        let rounded = self.b.ext(self.t.v4, glsl::FLOOR, &[sum]);
        let i = self.b.result(op::CONVERT_F_TO_S, self.t.iv4, &[rounded]);
        let a = self.addr_var();
        self.store_masked(
            a,
            self.t.iv4,
            i,
            d.write_mask,
            ins.predicate.as_ref(),
            ins.offset,
        )
    }

    fn setp(&mut self, ins: &Instruction) -> Result<(), Error> {
        let d = ins
            .dst
            .ok_or_else(|| invalid(ins.offset, "setp without destination"))?;
        if d.reg.kind != RegType::Predicate {
            return Err(invalid(ins.offset, "setp destination is not p0"));
        }
        self.check_range(d.reg, ins.offset)?;
        let a = self.src(ins, 0)?;
        let b = self.src(ins, 1)?;
        let opc = Self::compare_op(ins.control, ins.offset)?;
        let c = self.bin(opc, self.t.bv4, a, b);
        let p = self.pred_var();
        self.store_masked(
            p,
            self.t.bv4,
            c,
            d.write_mask,
            ins.predicate.as_ref(),
            ins.offset,
        )
    }

    // ------------------------------------------------------------- arithmetic

    #[allow(clippy::too_many_lines)]
    fn arithmetic(&mut self, ins: &Instruction) -> Result<(), Error> {
        use Opcode as O;
        let v4 = self.t.v4;
        let f = self.t.f32;
        let mut mask_limit = 0xF;
        let value = match ins.opcode {
            O::Mov => self.src(ins, 0)?,
            O::Add | O::Sub | O::Mul => {
                let a = self.src(ins, 0)?;
                let b = self.src(ins, 1)?;
                let opc = match ins.opcode {
                    O::Add => op::F_ADD,
                    O::Sub => op::F_SUB,
                    _ => op::F_MUL,
                };
                self.bin(opc, v4, a, b)
            }
            O::Mad => {
                let a = self.src(ins, 0)?;
                let b = self.src(ins, 1)?;
                let c = self.src(ins, 2)?;
                let m = self.bin(op::F_MUL, v4, a, b);
                self.bin(op::F_ADD, v4, m, c)
            }
            O::Min | O::Max => {
                let a = self.src(ins, 0)?;
                let b = self.src(ins, 1)?;
                let e = if ins.opcode == O::Min {
                    glsl::FMIN
                } else {
                    glsl::FMAX
                };
                self.b.ext(v4, e, &[a, b])
            }
            O::Slt | O::Sge => {
                let a = self.src(ins, 0)?;
                let b = self.src(ins, 1)?;
                let opc = if ins.opcode == O::Slt {
                    op::F_ORD_LESS_THAN
                } else {
                    op::F_ORD_GREATER_THAN_EQUAL
                };
                let c = self.bin(opc, self.t.bv4, a, b);
                let one = self.b.c_vec4_splat(1.0);
                let zero = self.zero4();
                self.select(v4, c, one, zero)
            }
            O::Rcp => {
                let x = self.src_scalar(ins, 0)?;
                let one = self.b.c_float(1.0);
                let r = self.bin(op::F_DIV, f, one, x);
                self.splat(r)
            }
            O::Rsq => {
                let x = self.src_scalar(ins, 0)?;
                let a = self.b.ext(f, glsl::FABS, &[x]);
                let s = self.b.ext(f, glsl::SQRT, &[a]);
                let one = self.b.c_float(1.0);
                let r = self.bin(op::F_DIV, f, one, s);
                self.splat(r)
            }
            O::Exp | O::ExpP => {
                let x = self.src_scalar(ins, 0)?;
                let r = self.b.ext(f, glsl::EXP2, &[x]);
                self.splat(r)
            }
            O::Log | O::LogP => {
                let x = self.src_scalar(ins, 0)?;
                let r = self.log2_abs(x);
                self.splat(r)
            }
            O::Pow => {
                let x = self.src_scalar(ins, 0)?;
                let y = self.src_scalar(ins, 1)?;
                let l = self.log2_abs(x);
                let m = self.bin(op::F_MUL, f, y, l);
                let r = self.b.ext(f, glsl::EXP2, &[m]);
                self.splat(r)
            }
            O::Frc | O::Abs | O::Sgn => {
                let a = self.src(ins, 0)?;
                let e = match ins.opcode {
                    O::Frc => glsl::FRACT,
                    O::Abs => glsl::FABS,
                    _ => glsl::FSIGN,
                };
                self.b.ext(v4, e, &[a])
            }
            O::Dp3 | O::Dp4 => {
                let a = self.src(ins, 0)?;
                let b = self.src(ins, 1)?;
                let n = if ins.opcode == O::Dp3 { 3 } else { 4 };
                let d = self.dot(a, b, n);
                self.splat(d)
            }
            O::Dp2Add => {
                let a = self.src(ins, 0)?;
                let b = self.src(ins, 1)?;
                let c = self.src_scalar(ins, 2)?;
                let d = self.dot(a, b, 2);
                let r = self.bin(op::F_ADD, f, d, c);
                self.splat(r)
            }
            O::Crs => {
                let a = self.src(ins, 0)?;
                let b = self.src(ins, 1)?;
                let a3 = self.truncate(a, 3);
                let b3 = self.truncate(b, 3);
                let c = self.b.ext(self.t.v3, glsl::CROSS, &[a3, b3]);
                let zero = self.zero4();
                mask_limit = 0x7;
                self.shuffle(v4, c, zero, &[0, 1, 2, 3])
            }
            O::Nrm => {
                let a = self.src(ins, 0)?;
                let d = self.dot(a, a, 3);
                let s = self.b.ext(f, glsl::SQRT, &[d]);
                let one = self.b.c_float(1.0);
                let r = self.bin(op::F_DIV, f, one, s);
                let rs = self.splat(r);
                self.bin(op::F_MUL, v4, a, rs)
            }
            O::Lrp => {
                let a = self.src(ins, 0)?;
                let b = self.src(ins, 1)?;
                let c = self.src(ins, 2)?;
                let d = self.bin(op::F_SUB, v4, b, c);
                let m = self.bin(op::F_MUL, v4, a, d);
                self.bin(op::F_ADD, v4, m, c)
            }
            O::Cmp | O::Cnd => {
                let a = self.src(ins, 0)?;
                let b = self.src(ins, 1)?;
                let c = self.src(ins, 2)?;
                let (opc, k) = if ins.opcode == O::Cmp {
                    (op::F_ORD_GREATER_THAN_EQUAL, 0.0)
                } else {
                    (op::F_ORD_GREATER_THAN, 0.5)
                };
                let kv = self.b.c_vec4_splat(k);
                let cond = self.bin(opc, self.t.bv4, a, kv);
                self.select(v4, cond, b, c)
            }
            O::Dsx | O::Dsy => {
                self.require_pixel(ins)?;
                let a = self.src(ins, 0)?;
                let opc = if ins.opcode == O::Dsx {
                    op::DPDX
                } else {
                    op::DPDY
                };
                self.b.result(opc, v4, &[a])
            }
            O::SinCos => {
                let x = self.src_scalar(ins, 0)?;
                let c = self.b.ext(f, glsl::COS, &[x]);
                let s = self.b.ext(f, glsl::SIN, &[x]);
                let zero = self.b.c_float(0.0);
                mask_limit = 0x3;
                self.vec4_of([c, s, zero, zero])
            }
            O::Lit => self.lit(ins)?,
            O::Dst => {
                let a = self.src(ins, 0)?;
                let b = self.src(ins, 1)?;
                let ay = self.fx(a, 1);
                let by = self.fx(b, 1);
                let y = self.bin(op::F_MUL, f, ay, by);
                let z = self.fx(a, 2);
                let w = self.fx(b, 3);
                let one = self.b.c_float(1.0);
                self.vec4_of([one, y, z, w])
            }
            O::M4x4 | O::M4x3 | O::M3x4 | O::M3x3 | O::M3x2 => {
                let (n, rows) = match ins.opcode {
                    O::M4x4 => (4, 4u16),
                    O::M4x3 => (4, 3),
                    O::M3x4 => (3, 4),
                    O::M3x3 => (3, 3),
                    _ => (3, 2),
                };
                let a = self.src(ins, 0)?;
                let m = ins.src[1];
                let zero = self.b.c_float(0.0);
                let mut parts = [zero; 4];
                for (k, part) in (0..rows).zip(parts.iter_mut()) {
                    let row = self.load_src_at(&m, k, ins.offset)?;
                    *part = self.dot(a, row, n);
                }
                mask_limit = (1u8 << rows) - 1;
                self.vec4_of(parts)
            }
            other => {
                return Err(unsupported(
                    ins.offset,
                    format!("instruction {}", other.name()),
                ));
            }
        };
        self.write_dst(ins, value, mask_limit)
    }

    fn lit(&mut self, ins: &Instruction) -> Result<Id, Error> {
        let f = self.t.f32;
        let s = self.src(ins, 0)?;
        let x = self.fx(s, 0);
        let y = self.fx(s, 1);
        let w = self.fx(s, 3);
        let zero = self.b.c_float(0.0);
        let one = self.b.c_float(1.0);
        let lim = self.b.c_float(LIT_EXPONENT_LIMIT);
        let nlim = self.b.c_float(-LIT_EXPONENT_LIMIT);
        let p = self.b.ext(f, glsl::FMIN, &[w, lim]);
        let p = self.b.ext(f, glsl::FMAX, &[p, nlim]);
        let l = self.log2_abs(y);
        let m = self.bin(op::F_MUL, f, p, l);
        let powv = self.b.ext(f, glsl::EXP2, &[m]);
        let xpos = self.bin(op::F_ORD_GREATER_THAN, self.t.bool, x, zero);
        let ypos = self.bin(op::F_ORD_GREATER_THAN, self.t.bool, y, zero);
        let both = self.bin(op::LOGICAL_AND, self.t.bool, xpos, ypos);
        let dy = self.select(f, xpos, x, zero);
        let dz = self.select(f, both, powv, zero);
        Ok(self.vec4_of([one, dy, dz, one]))
    }

    // ------------------------------------------------------------------ finish

    fn finish(mut self) -> Module {
        let (model_id, stage) = match self.stage() {
            Stage::Vertex => (model::VERTEX, Stage::Vertex),
            Stage::Pixel => (model::FRAGMENT, Stage::Pixel),
        };
        let interface = self.interface.clone();
        self.b.entry_point(model_id, self.main, "main", &interface);
        let depth_replacing = self.depth_stage.is_some();
        if stage == Stage::Pixel {
            self.b.execution_mode(self.main, mode::ORIGIN_UPPER_LEFT);
            if depth_replacing {
                self.b.execution_mode(self.main, mode::DEPTH_REPLACING);
            }
        }
        let mut samplers: Vec<SamplerBinding> = self
            .samplers
            .iter()
            .map(|(&n, d)| SamplerBinding {
                register: n,
                set: binding::SAMPLER_SET,
                binding: if stage == Stage::Vertex {
                    binding::VS_SAMPLER_BINDING_BASE
                } else {
                    binding::PS_SAMPLER_BINDING_BASE
                } + u32::from(n),
                dim: d.dim,
                depth_compare: d.depth,
            })
            .collect();
        samplers.sort_by_key(|s| s.register);
        let constants = ConstantUse {
            float_read: self.float_read.iter().copied().collect(),
            float_relative: self.float_relative,
            float_defined: self.defs_f.keys().copied().collect(),
            int_read: self.int_read.iter().copied().collect(),
            int_defined: self.defs_i.keys().copied().collect(),
            bool_read: self.bool_read.iter().copied().collect(),
            bool_defined: self.defs_b.keys().copied().collect(),
        };
        let float_constant_binding = self.float_ubo.map(|_| {
            if stage == Stage::Vertex {
                binding::VS_FLOAT_BINDING
            } else {
                binding::PS_FLOAT_BINDING
            }
        });
        let int_bool_constant_binding = self.int_bool_ubo.map(|_| {
            if stage == Stage::Vertex {
                binding::VS_INT_BOOL_BINDING
            } else {
                binding::PS_INT_BOOL_BINDING
            }
        });
        let push_constant_size = if self.push.is_some() {
            binding::PUSH_CONSTANT_SIZE
        } else {
            0
        };
        Module {
            words: self.b.assemble(),
            stage,
            inputs: self.refl_inputs,
            outputs: self.refl_outputs,
            samplers,
            constants,
            float_constant_binding,
            int_bool_constant_binding,
            push_constant_size,
            depth_replacing,
        }
    }
}
