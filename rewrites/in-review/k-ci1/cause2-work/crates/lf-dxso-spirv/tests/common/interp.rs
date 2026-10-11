//! A small SPIR-V interpreter for the subset the translator emits, used to
//! check translated programs by their results. It runs the entry point on
//! values the test places in named variables (names come from `OpName`)
//! and reads named outputs back.
//!
//! Images are fake: a sample returns `(u, v, w', binding)` where `w'` is
//! the coordinate's third component (0 for 2D) plus any bias, lod or
//! gradient `x` components, so tests can see which coordinate and operand
//! reached the sampler. A depth-compare sample returns
//! `1.0 if dref <= u else 0.0`. Derivatives return zero.

// The interpreter reinterprets bit patterns on purpose (SPIR-V scalars are
// raw 32-bit words) and compares floats exactly, as SPIR-V does; short
// names follow operand roles.
#![allow(
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss,
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss,
    clippy::float_cmp,
    clippy::many_single_char_names
)]

use lf_dxso_spirv::spirv::{decoration, glsl, op};
use lf_dxso_spirv::validate::{Inst, instructions};
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub enum V {
    /// Scalar bits (float bits, integer bits, or 0/1 for booleans).
    S(u32),
    /// Composite.
    C(Vec<V>),
    /// Pointer: variable and access path.
    P(u32, Vec<usize>),
    /// Sampled image from a variable.
    Img(u32),
}

#[derive(Clone, Debug, PartialEq)]
enum Ty {
    Void,
    Bool,
    Int,
    Float,
    Vector(u32, u32),
    Array(u32, u32),
    Struct(Vec<u32>),
    Pointer(u32),
    Other,
}

pub struct Machine<'a> {
    insts: Vec<Inst<'a>>,
    types: HashMap<u32, Ty>,
    vtype: HashMap<u32, u32>,
    vals: HashMap<u32, V>,
    labels: HashMap<u32, usize>,
    funcs: HashMap<u32, usize>,
    names: HashMap<String, u32>,
    bindings: HashMap<u32, u32>,
    pub mem: HashMap<u32, V>,
    pub killed: bool,
    entry: u32,
    steps: usize,
    glsl: u32,
}

fn f(v: &V) -> f32 {
    match v {
        V::S(b) => f32::from_bits(*b),
        _ => panic!("not a scalar: {v:?}"),
    }
}

fn bits(v: &V) -> u32 {
    match v {
        V::S(b) => *b,
        _ => panic!("not a scalar: {v:?}"),
    }
}

fn fs(x: f32) -> V {
    V::S(x.to_bits())
}

fn bs(x: bool) -> V {
    V::S(u32::from(x))
}

fn map1(a: &V, g: &dyn Fn(&V) -> V) -> V {
    match a {
        V::C(xs) => V::C(xs.iter().map(g).collect()),
        s => g(s),
    }
}

fn map2(a: &V, b: &V, g: &dyn Fn(&V, &V) -> V) -> V {
    match (a, b) {
        (V::C(xs), V::C(ys)) => V::C(xs.iter().zip(ys).map(|(x, y)| g(x, y)).collect()),
        (x, y) => g(x, y),
    }
}

fn map3(a: &V, b: &V, c: &V, g: &dyn Fn(&V, &V, &V) -> V) -> V {
    match (a, b, c) {
        (V::C(xs), V::C(ys), V::C(zs)) => V::C(
            xs.iter()
                .zip(ys)
                .zip(zs)
                .map(|((x, y), z)| g(x, y, z))
                .collect(),
        ),
        (x, y, z) => g(x, y, z),
    }
}

fn comps(v: &V) -> Vec<V> {
    match v {
        V::C(xs) => xs.clone(),
        s => vec![s.clone()],
    }
}

impl<'a> Machine<'a> {
    pub fn new(words: &'a [u32]) -> Machine<'a> {
        let insts = instructions(words).expect("instructions");
        let mut m = Machine {
            insts,
            types: HashMap::new(),
            vtype: HashMap::new(),
            vals: HashMap::new(),
            labels: HashMap::new(),
            funcs: HashMap::new(),
            names: HashMap::new(),
            bindings: HashMap::new(),
            mem: HashMap::new(),
            killed: false,
            entry: 0,
            steps: 0,
            glsl: 0,
        };
        let insts = m.insts.clone();
        for (n, i) in insts.iter().enumerate() {
            let o = i.operands;
            match i.opcode {
                op::EXT_INST_IMPORT => m.glsl = o[0],
                op::ENTRY_POINT => m.entry = o[1],
                op::NAME => {
                    let (s, _) = lf_dxso_spirv::validate::read_string(&o[1..]).unwrap();
                    m.names.insert(s, o[0]);
                }
                op::DECORATE if o[1] == decoration::BINDING => {
                    m.bindings.insert(o[0], o[2]);
                }
                op::TYPE_VOID => {
                    m.types.insert(o[0], Ty::Void);
                }
                op::TYPE_BOOL => {
                    m.types.insert(o[0], Ty::Bool);
                }
                op::TYPE_INT => {
                    m.types.insert(o[0], Ty::Int);
                }
                op::TYPE_FLOAT => {
                    m.types.insert(o[0], Ty::Float);
                }
                op::TYPE_VECTOR => {
                    m.types.insert(o[0], Ty::Vector(o[1], o[2]));
                }
                op::TYPE_ARRAY => {
                    let len = bits(&m.vals[&o[2]]);
                    m.types.insert(o[0], Ty::Array(o[1], len));
                }
                op::TYPE_STRUCT => {
                    m.types.insert(o[0], Ty::Struct(o[1..].to_vec()));
                }
                op::TYPE_POINTER => {
                    m.types.insert(o[0], Ty::Pointer(o[2]));
                }
                op::TYPE_FUNCTION | op::TYPE_IMAGE | op::TYPE_SAMPLED_IMAGE => {
                    m.types.insert(o[0], Ty::Other);
                }
                op::CONSTANT => {
                    m.vtype.insert(o[1], o[0]);
                    m.vals.insert(o[1], V::S(o[2]));
                }
                op::CONSTANT_TRUE | op::CONSTANT_FALSE => {
                    m.vtype.insert(o[1], o[0]);
                    m.vals.insert(o[1], bs(i.opcode == op::CONSTANT_TRUE));
                }
                op::CONSTANT_COMPOSITE => {
                    m.vtype.insert(o[1], o[0]);
                    let parts = o[2..].iter().map(|x| m.vals[x].clone()).collect();
                    m.vals.insert(o[1], V::C(parts));
                }
                op::VARIABLE if m.funcs.is_empty() => {
                    let Ty::Pointer(pointee) = m.types[&o[0]].clone() else {
                        panic!()
                    };
                    let init = match o.get(3) {
                        Some(x) => m.vals[x].clone(),
                        None => m.zero(pointee),
                    };
                    m.mem.insert(o[1], init);
                    m.vals.insert(o[1], V::P(o[1], Vec::new()));
                }
                op::FUNCTION => {
                    m.funcs.insert(o[1], n);
                }
                op::LABEL => {
                    m.labels.insert(o[0], n);
                }
                _ => {}
            }
        }
        m
    }

    fn zero(&self, ty: u32) -> V {
        match &self.types[&ty] {
            Ty::Vector(_, n) => V::C(vec![V::S(0); *n as usize]),
            Ty::Array(e, n) => V::C(vec![self.zero(*e); *n as usize]),
            Ty::Struct(ms) => V::C(ms.iter().map(|m| self.zero(*m)).collect()),
            _ => V::S(0),
        }
    }

    fn var(&self, name: &str) -> u32 {
        *self
            .names
            .get(name)
            .unwrap_or_else(|| panic!("no variable named {name}"))
    }

    pub fn has(&self, name: &str) -> bool {
        self.names.contains_key(name)
    }

    pub fn set(&mut self, name: &str, v: V) {
        let id = self.var(name);
        self.mem.insert(id, v);
    }

    pub fn set_vec4(&mut self, name: &str, x: [f32; 4]) {
        self.set(name, V::C(x.iter().map(|&a| fs(a)).collect()));
    }

    pub fn set_bool(&mut self, name: &str, x: bool) {
        self.set(name, bs(x));
    }

    /// Float constant registers.
    pub fn set_c(&mut self, regs: &[(usize, [f32; 4])]) {
        if !self.has("c") {
            return;
        }
        let id = self.var("c");
        let V::C(mut st) = self.mem[&id].clone() else {
            panic!()
        };
        let V::C(arr) = &mut st[0] else { panic!() };
        for (n, x) in regs {
            arr[*n] = V::C(x.iter().map(|&a| fs(a)).collect());
        }
        self.mem.insert(id, V::C(st));
    }

    /// Integer constant registers and the boolean bit mask.
    pub fn set_ib(&mut self, ints: &[(usize, [i32; 4])], bools: u32) {
        if !self.has("ib") {
            return;
        }
        let id = self.var("ib");
        let V::C(mut st) = self.mem[&id].clone() else {
            panic!()
        };
        let V::C(arr) = &mut st[0] else { panic!() };
        for (n, x) in ints {
            arr[*n] = V::C(x.iter().map(|&a| V::S(a as u32)).collect());
        }
        st[1] = V::S(bools);
        self.mem.insert(id, V::C(st));
    }

    pub fn set_push(&mut self, x: [f32; 4]) {
        if self.has("push") {
            self.set("push", V::C(vec![V::C(x.iter().map(|&a| fs(a)).collect())]));
        }
    }

    pub fn get_vec4(&self, name: &str) -> [f32; 4] {
        let v = &self.mem[&self.var(name)];
        let c = comps(v);
        [f(&c[0]), f(&c[1]), f(&c[2]), f(&c[3])]
    }

    pub fn get_f32(&self, name: &str) -> f32 {
        f(&self.mem[&self.var(name)])
    }

    pub fn run(&mut self) {
        let e = self.entry;
        self.call(e);
    }

    fn call(&mut self, fid: u32) {
        let mut pc = self.funcs[&fid] + 1;
        loop {
            self.steps += 1;
            assert!(self.steps < 1_000_000, "interpreter step limit");
            let i = self.insts[pc];
            let o = i.operands;
            match i.opcode {
                op::LABEL | op::SELECTION_MERGE | op::LOOP_MERGE => pc += 1,
                op::BRANCH => pc = self.labels[&o[0]],
                op::BRANCH_CONDITIONAL => {
                    let c = bits(&self.vals[&o[0]]) != 0;
                    pc = self.labels[&if c { o[1] } else { o[2] }];
                }
                op::RETURN | op::FUNCTION_END => return,
                op::KILL => {
                    self.killed = true;
                    return;
                }
                op::UNREACHABLE => panic!("reached OpUnreachable"),
                op::FUNCTION_CALL => {
                    self.call(o[2]);
                    if self.killed {
                        return;
                    }
                    self.vals.insert(o[1], V::S(0));
                    pc += 1;
                }
                _ => {
                    self.exec(&i);
                    pc += 1;
                }
            }
        }
    }

    fn get(&self, id: u32) -> V {
        self.vals
            .get(&id)
            .cloned()
            .unwrap_or_else(|| panic!("value %{id} not computed"))
    }

    fn load_path(&self, var: u32, path: &[usize]) -> V {
        let mut v = &self.mem[&var];
        for &k in path {
            let V::C(xs) = v else {
                panic!("path into scalar")
            };
            v = &xs[k];
        }
        v.clone()
    }

    fn store_path(&mut self, var: u32, path: &[usize], value: V) {
        let mut v = self.mem.get_mut(&var).unwrap();
        for &k in path {
            let V::C(xs) = v else {
                panic!("path into scalar")
            };
            v = &mut xs[k];
        }
        *v = value;
    }

    fn scalar_kind(&self, ty: u32) -> Ty {
        match &self.types[&ty] {
            Ty::Vector(c, _) => self.types[c].clone(),
            t => t.clone(),
        }
    }

    #[allow(clippy::too_many_lines)]
    fn exec(&mut self, i: &Inst<'_>) {
        let o = i.operands;
        if i.opcode == op::STORE {
            let V::P(var, path) = self.get(o[0]) else {
                panic!()
            };
            let v = self.get(o[1]);
            self.store_path(var, &path, v);
            return;
        }
        let ty = o[0];
        let res = o[1];
        let a = |k: usize| self.get(o[k]);
        let v = match i.opcode {
            op::VARIABLE => panic!("function-local variable not expected"),
            op::LOAD => {
                let V::P(var, path) = a(2) else { panic!() };
                if matches!(self.types.get(&ty), Some(Ty::Other)) {
                    V::Img(var)
                } else {
                    self.load_path(var, &path)
                }
            }
            op::ACCESS_CHAIN => {
                let V::P(var, mut path) = a(2) else { panic!() };
                for &k in &o[3..] {
                    path.push(bits(&self.get(k)) as usize);
                }
                V::P(var, path)
            }
            op::COMPOSITE_CONSTRUCT => {
                let mut parts = Vec::new();
                for &x in &o[2..] {
                    parts.extend(comps(&self.get(x)));
                }
                V::C(parts)
            }
            op::COMPOSITE_EXTRACT => {
                let mut v = a(2);
                for &k in &o[3..] {
                    v = comps(&v)[k as usize].clone();
                }
                v
            }
            op::VECTOR_SHUFFLE => {
                let mut all = comps(&a(2));
                all.extend(comps(&a(3)));
                V::C(o[4..].iter().map(|&k| all[k as usize].clone()).collect())
            }
            op::F_ADD => map2(&a(2), &a(3), &|x, y| fs(f(x) + f(y))),
            op::F_SUB => map2(&a(2), &a(3), &|x, y| fs(f(x) - f(y))),
            op::F_MUL => map2(&a(2), &a(3), &|x, y| fs(f(x) * f(y))),
            op::F_DIV => map2(&a(2), &a(3), &|x, y| fs(f(x) / f(y))),
            op::F_NEGATE => map1(&a(2), &|x| fs(-f(x))),
            op::I_ADD => map2(&a(2), &a(3), &|x, y| V::S(bits(x).wrapping_add(bits(y)))),
            op::SHIFT_RIGHT_LOGICAL => map2(&a(2), &a(3), &|x, y| V::S(bits(x) >> bits(y))),
            op::BITWISE_AND => map2(&a(2), &a(3), &|x, y| V::S(bits(x) & bits(y))),
            op::CONVERT_F_TO_S => map1(&a(2), &|x| V::S((f(x) as i32) as u32)),
            op::DOT => {
                let (x, y) = (comps(&a(2)), comps(&a(3)));
                fs(x.iter().zip(&y).map(|(p, q)| f(p) * f(q)).sum())
            }
            op::LOGICAL_AND => map2(&a(2), &a(3), &|x, y| bs(bits(x) != 0 && bits(y) != 0)),
            op::LOGICAL_OR => map2(&a(2), &a(3), &|x, y| bs(bits(x) != 0 || bits(y) != 0)),
            op::LOGICAL_NOT => map1(&a(2), &|x| bs(bits(x) == 0)),
            op::ANY => bs(comps(&a(2)).iter().any(|x| bits(x) != 0)),
            op::SELECT => map3(&a(2), &a(3), &a(4), &|c, x, y| {
                if bits(c) != 0 { x.clone() } else { y.clone() }
            }),
            op::I_EQUAL => map2(&a(2), &a(3), &|x, y| bs(bits(x) == bits(y))),
            op::I_NOT_EQUAL => map2(&a(2), &a(3), &|x, y| bs(bits(x) != bits(y))),
            op::S_LESS_THAN => map2(
                &a(2),
                &a(3),
                &|x, y| bs((bits(x) as i32) < (bits(y) as i32)),
            ),
            op::S_GREATER_THAN_EQUAL => map2(&a(2), &a(3), &|x, y| {
                bs((bits(x) as i32) >= (bits(y) as i32))
            }),
            op::F_ORD_EQUAL => map2(&a(2), &a(3), &|x, y| bs(f(x) == f(y))),
            op::F_UNORD_NOT_EQUAL => map2(&a(2), &a(3), &|x, y| bs(f(x) != f(y))),
            op::F_ORD_LESS_THAN => map2(&a(2), &a(3), &|x, y| bs(f(x) < f(y))),
            op::F_ORD_GREATER_THAN => map2(&a(2), &a(3), &|x, y| bs(f(x) > f(y))),
            op::F_ORD_LESS_THAN_EQUAL => map2(&a(2), &a(3), &|x, y| bs(f(x) <= f(y))),
            op::F_ORD_GREATER_THAN_EQUAL => map2(&a(2), &a(3), &|x, y| bs(f(x) >= f(y))),
            op::DPDX | op::DPDY => map1(&a(2), &|_| fs(0.0)),
            op::EXT_INST => {
                assert_eq!(o[2], self.glsl);
                let args: Vec<V> = o[4..].iter().map(|&x| self.get(x)).collect();
                let int = self.scalar_kind(ty) == Ty::Int;
                match o[3] {
                    glsl::FABS => map1(&args[0], &|x| fs(f(x).abs())),
                    glsl::FSIGN => map1(&args[0], &|x| {
                        let v = f(x);
                        fs(if v > 0.0 {
                            1.0
                        } else if v < 0.0 {
                            -1.0
                        } else {
                            v
                        })
                    }),
                    glsl::FLOOR => map1(&args[0], &|x| fs(f(x).floor())),
                    glsl::FRACT => map1(&args[0], &|x| fs(f(x) - f(x).floor())),
                    glsl::SIN => map1(&args[0], &|x| fs(f(x).sin())),
                    glsl::COS => map1(&args[0], &|x| fs(f(x).cos())),
                    glsl::EXP2 => map1(&args[0], &|x| fs(f(x).exp2())),
                    glsl::LOG2 => map1(&args[0], &|x| fs(f(x).log2())),
                    glsl::SQRT => map1(&args[0], &|x| fs(f(x).sqrt())),
                    glsl::FMIN => map2(&args[0], &args[1], &|x, y| fs(f(x).min(f(y)))),
                    glsl::FMAX => map2(&args[0], &args[1], &|x, y| fs(f(x).max(f(y)))),
                    glsl::SCLAMP if int => map3(&args[0], &args[1], &args[2], &|x, lo, hi| {
                        V::S((bits(x) as i32).clamp(bits(lo) as i32, bits(hi) as i32) as u32)
                    }),
                    glsl::CROSS => {
                        let (p, q) = (comps(&args[0]), comps(&args[1]));
                        let (p, q): (Vec<f32>, Vec<f32>) =
                            (p.iter().map(f).collect(), q.iter().map(f).collect());
                        V::C(vec![
                            fs(p[1] * q[2] - p[2] * q[1]),
                            fs(p[2] * q[0] - p[0] * q[2]),
                            fs(p[0] * q[1] - p[1] * q[0]),
                        ])
                    }
                    other => panic!("GLSL.std.450 {other} not interpreted"),
                }
            }
            op::IMAGE_SAMPLE_IMPLICIT_LOD | op::IMAGE_SAMPLE_EXPLICIT_LOD => {
                let V::Img(var) = a(2) else { panic!() };
                let c: Vec<f32> = comps(&a(3)).iter().map(f).collect();
                let extra: f32 = o
                    .get(5..)
                    .unwrap_or(&[])
                    .iter()
                    .map(|&x| f(&comps(&self.get(x))[0]))
                    .sum();
                let third = c.get(2).copied().unwrap_or(0.0);
                let binding = self.bindings[&var] as f32;
                V::C(vec![fs(c[0]), fs(c[1]), fs(third + extra), fs(binding)])
            }
            op::IMAGE_SAMPLE_DREF_IMPLICIT_LOD | op::IMAGE_SAMPLE_DREF_EXPLICIT_LOD => {
                let c: Vec<f32> = comps(&a(3)).iter().map(f).collect();
                let dref = f(&a(4));
                fs(if dref <= c[0] { 1.0 } else { 0.0 })
            }
            other => panic!("opcode {other} not interpreted"),
        };
        self.vtype.insert(res, ty);
        self.vals.insert(res, v);
    }
}
