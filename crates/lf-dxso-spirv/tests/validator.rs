//! The structural validator must reject broken modules. Each test builds
//! a correct module and a deliberately wrong twin that differs in one
//! thing; the correct one must pass and the wrong one must fail with the
//! expected complaint. Modules are built with the crate's own `Builder`
//! or by editing translator output.

// Short names follow SPIR-V operand roles (`b` builder, `c` context, `m`
// merge, `h` header, `t` condition).
#![allow(clippy::many_single_char_names)]

mod common;

use common::*;
use lf_dxso_spirv::spirv::{
    Builder, Id, decoration, dim, image_operands, mode, model, op as sop, push_inst, storage,
};
use lf_dxso_spirv::validate::{instructions, validate};

struct Ctx {
    f: Id,
    v4: Id,
    one: Id,
    out: Id,
}

/// A fragment module writing `vec4` output location 0; `body` emits the
/// main function's code after the entry label and before the final store.
fn fragment(body: impl FnOnce(&mut Builder, &Ctx) -> Id) -> Vec<u32> {
    let mut b = Builder::new();
    let void = b.t_void();
    let f = b.t_float();
    let v4 = b.t_vec4();
    let fnty = b.t_function(void, &[]);
    let ptr = b.t_pointer(storage::OUTPUT, v4);
    let out = b.global_variable(ptr, storage::OUTPUT, None);
    b.decorate(out, decoration::LOCATION, &[0]);
    let one = b.c_float(1.0);
    let main = b.id();
    let ctx = Ctx { f, v4, one, out };
    b.begin_function(main, void, fnty);
    let value = body(&mut b, &ctx);
    b.emit(sop::STORE, &[out, value]);
    b.end_function();
    b.entry_point(model::FRAGMENT, main, "main", &[out]);
    b.execution_mode(main, mode::ORIGIN_UPPER_LEFT);
    b.assemble()
}

fn splat(b: &mut Builder, c: &Ctx, x: Id) -> Id {
    b.result(sop::COMPOSITE_CONSTRUCT, c.v4, &[x, x, x, x])
}

fn expect_err(words: &[u32], needle: &str) {
    match validate(words) {
        Ok(_) => panic!(
            "validator accepted a broken module (expected: {needle})\n{}",
            dump(words)
        ),
        Err(e) => assert!(
            e.message.contains(needle),
            "wrong complaint: {e} (expected: {needle})"
        ),
    }
}

/// Rebuild a module with instruction `n` (by index) replaced by `f(words)`.
fn edit(
    words: &[u32],
    pick: impl Fn(u16, &[u32]) -> bool,
    f: impl Fn(&[u32]) -> Option<Vec<u32>>,
) -> Vec<u32> {
    let insts = instructions(words).unwrap();
    let mut out = words[..5].to_vec();
    let mut done = false;
    for i in insts {
        let whole = &words[i.offset..i.offset + 1 + i.operands.len()];
        if !done && pick(i.opcode, i.operands) {
            done = true;
            if let Some(w) = f(whole) {
                out.extend(w);
            }
        } else {
            out.extend_from_slice(whole);
        }
    }
    assert!(done, "edit target not found");
    out
}

#[test]
fn the_baseline_module_passes() {
    let w = fragment(|b, c| splat(b, c, c.one));
    validate(&w).unwrap();
}

#[test]
fn header_checks() {
    let w = fragment(|b, c| splat(b, c, c.one));
    let mut bad = w.clone();
    bad[0] = 0x0302_2307;
    expect_err(&bad, "magic");
    let mut bad = w.clone();
    bad[1] = 0x0001_0300;
    expect_err(&bad, "version");
    let mut bad = w.clone();
    bad[3] = 3;
    expect_err(&bad, "outside the bound");
    let mut bad = w;
    bad[4] = 1;
    expect_err(&bad, "schema");
    expect_err(&[0x0723_0203, 0x0001_0000], "header");
}

#[test]
fn word_count_and_unknown_opcode() {
    let w = fragment(|b, c| splat(b, c, c.one));
    let mut bad = w.clone();
    let last = bad.len() - 1;
    bad[last] = (9 << 16) | u32::from(sop::FUNCTION_END);
    expect_err(&bad, "runs past the end");
    let bad = edit(&w, |o, _| o == sop::RETURN, |_| Some(vec![1 << 16])); // OpNop
    expect_err(&bad, "not one this crate emits");
}

#[test]
fn layout_order() {
    let w = fragment(|b, c| splat(b, c, c.one));
    // Move the capability to the end of the module.
    let cap = edit(&w, |o, _| o == sop::CAPABILITY, |_| None);
    let mut bad = cap;
    push_inst(&mut bad, sop::CAPABILITY, &[1]);
    expect_err(&bad, "layout order");
    let bad = edit(&w, |o, _| o == sop::MEMORY_MODEL, |_| None);
    expect_err(&bad, "OpMemoryModel");
}

#[test]
fn ids_defined_once_and_before_use() {
    let good = fragment(|b, c| {
        let x = b.result(sop::F_ADD, c.f, &[c.one, c.one]);
        splat(b, c, x)
    });
    validate(&good).unwrap();
    // Use an id that is defined later in the same block.
    let bad = fragment(|b, c| {
        let later = b.id();
        let x = b.result(sop::F_ADD, c.f, &[c.one, later + 1]);
        let _ = later;
        splat(b, c, x)
    });
    expect_err(&bad, "before its definition");
    // Define an id twice: rewrite the OpCompositeConstruct result to the FAdd's.
    let bad = edit(
        &good,
        |o, _| o == sop::COMPOSITE_CONSTRUCT,
        |w| {
            let mut w = w.to_vec();
            w[2] -= 1;
            Some(w)
        },
    );
    expect_err(&bad, "defined twice");
}

#[test]
fn type_checks() {
    // FAdd with a vec4 result type and float operands.
    let bad = fragment(|b, c| b.result(sop::F_ADD, c.v4, &[c.one, c.one]));
    expect_err(&bad, "operand type differs");
    // Store a float into a vec4 output.
    let bad = fragment(|_, c| c.one);
    expect_err(&bad, "stored value type");
    // Shuffle with the wrong literal count.
    let bad = fragment(|b, c| {
        let v = splat(b, c, c.one);
        b.result(sop::VECTOR_SHUFFLE, c.v4, &[v, v, 0, 1, 2])
    });
    expect_err(&bad, "literal count");
    // Select with a scalar condition on a vector (not allowed in SPIR-V 1.0).
    let bad = fragment(|b, c| {
        let t = b.c_bool(true);
        let v = splat(b, c, c.one);
        b.result(sop::SELECT, c.v4, &[t, v, v])
    });
    expect_err(&bad, "select condition");
    // Load through a pointer of the wrong pointee.
    let bad = fragment(|b, c| b.result(sop::LOAD, c.f, &[c.out]));
    expect_err(&bad, "load result type");
}

#[test]
fn stores_to_read_only_storage() {
    let mut b = Builder::new();
    let void = b.t_void();
    let v4 = b.t_vec4();
    let fnty = b.t_function(void, &[]);
    let ptr = b.t_pointer(storage::INPUT, v4);
    let input = b.global_variable(ptr, storage::INPUT, None);
    b.decorate(input, decoration::LOCATION, &[0]);
    let zero = b.c_vec4_splat(0.0);
    let main = b.id();
    b.begin_function(main, void, fnty);
    b.emit(sop::STORE, &[input, zero]);
    b.end_function();
    b.entry_point(model::FRAGMENT, main, "main", &[input]);
    b.execution_mode(main, mode::ORIGIN_UPPER_LEFT);
    expect_err(&b.assemble(), "read-only");
}

#[test]
fn blocks_and_terminators() {
    let good = fragment(|b, c| splat(b, c, c.one));
    let bad = edit(&good, |o, _| o == sop::RETURN, |_| None);
    expect_err(&bad, "terminator");
    // A conditional branch without a merge instruction.
    let bad = fragment(|b, c| {
        let t = b.c_bool(true);
        let (x, y) = (b.id(), b.id());
        b.terminate(sop::BRANCH_CONDITIONAL, &[t, x, y]);
        b.label(x);
        b.terminate(sop::BRANCH, &[y]);
        b.label(y);
        splat(b, c, c.one)
    });
    expect_err(&bad, "without a structured merge");
    // Two constructs sharing a merge block.
    let bad = fragment(|b, c| {
        let t = b.c_bool(true);
        let (m, x, y) = (b.id(), b.id(), b.id());
        b.merge_and_terminate(
            sop::SELECTION_MERGE,
            &[m, 0],
            sop::BRANCH_CONDITIONAL,
            &[t, x, m],
        );
        b.label(x);
        b.merge_and_terminate(
            sop::SELECTION_MERGE,
            &[m, 0],
            sop::BRANCH_CONDITIONAL,
            &[t, y, m],
        );
        b.label(y);
        b.terminate(sop::BRANCH, &[m]);
        b.label(m);
        splat(b, c, c.one)
    });
    expect_err(&bad, "merge block of two constructs");
    // Branch to something that is not a label.
    let bad = fragment(|b, c| {
        b.terminate(sop::BRANCH, &[c.one]);
        splat(b, c, c.one)
    });
    expect_err(&bad, "not a label or function");
}

#[test]
fn dominance() {
    let build = |use_in_else: bool| {
        fragment(move |b, c| {
            let t = b.c_bool(true);
            let (m, x, y) = (b.id(), b.id(), b.id());
            b.merge_and_terminate(
                sop::SELECTION_MERGE,
                &[m, 0],
                sop::BRANCH_CONDITIONAL,
                &[t, x, y],
            );
            b.label(x);
            let v = b.result(sop::F_ADD, c.f, &[c.one, c.one]);
            if !use_in_else {
                b.result(sop::F_ADD, c.f, &[v, c.one]);
            }
            b.terminate(sop::BRANCH, &[m]);
            b.label(y);
            if use_in_else {
                // `v` is defined in the then block, which does not dominate this one.
                b.result(sop::F_ADD, c.f, &[v, c.one]);
            }
            b.terminate(sop::BRANCH, &[m]);
            b.label(m);
            splat(b, c, c.one)
        })
    };
    validate(&build(false)).unwrap();
    expect_err(&build(true), "does not dominate");
}

#[test]
fn back_edges_must_come_from_a_loop_continue_target() {
    let build = |structured: bool| {
        fragment(move |b, c| {
            let t = b.c_bool(false);
            let (h, body, cont, m) = (b.id(), b.id(), b.id(), b.id());
            b.label(h);
            if structured {
                b.merge_and_terminate(
                    sop::LOOP_MERGE,
                    &[m, cont, 0],
                    sop::BRANCH_CONDITIONAL,
                    &[t, body, m],
                );
            } else {
                b.merge_and_terminate(
                    sop::SELECTION_MERGE,
                    &[m, 0],
                    sop::BRANCH_CONDITIONAL,
                    &[t, body, m],
                );
            }
            b.label(body);
            b.terminate(sop::BRANCH, &[cont]);
            b.label(cont);
            b.terminate(sop::BRANCH, &[h]);
            b.label(m);
            splat(b, c, c.one)
        })
    };
    validate(&build(true)).unwrap();
    expect_err(&build(false), "back edge");
}

#[test]
fn entry_point_and_interface_rules() {
    // Fragment without OriginUpperLeft.
    let mut b = Builder::new();
    let void = b.t_void();
    let fnty = b.t_function(void, &[]);
    let main = b.id();
    b.begin_function(main, void, fnty);
    b.end_function();
    b.entry_point(model::FRAGMENT, main, "main", &[]);
    expect_err(&b.assemble(), "OriginUpperLeft");

    // Output without Location or BuiltIn.
    let w = fragment(|b, c| splat(b, c, c.one));
    let bad = edit(
        &w,
        |o, ops| o == sop::DECORATE && ops[1] == decoration::LOCATION,
        |_| None,
    );
    expect_err(&bad, "exactly one of BuiltIn and Location");

    // Output missing from the entry point interface.
    let bad = edit(
        &w,
        |o, _| o == sop::ENTRY_POINT,
        |words| {
            let mut words = words[..words.len() - 1].to_vec();
            words[0] -= 1 << 16;
            Some(words)
        },
    );
    expect_err(&bad, "missing from the entry point");
}

#[test]
fn resource_rules() {
    let build = |with_binding: bool, with_block: bool| {
        fragment(move |b, c| {
            let arr = b.t_array_unique(c.v4, 4);
            b.decorate(arr, decoration::ARRAY_STRIDE, &[16]);
            let st = b.t_struct_unique(&[arr]);
            if with_block {
                b.decorate(st, decoration::BLOCK, &[]);
            }
            b.member_decorate(st, 0, decoration::OFFSET, &[0]);
            let ptr = b.t_pointer(storage::UNIFORM, st);
            let var = b.global_variable(ptr, storage::UNIFORM, None);
            b.decorate(var, decoration::DESCRIPTOR_SET, &[0]);
            if with_binding {
                b.decorate(var, decoration::BINDING, &[0]);
            }
            let pv = b.t_pointer(storage::UNIFORM, c.v4);
            let z = b.c_int(0);
            let p = b.result(sop::ACCESS_CHAIN, pv, &[var, z, z]);
            b.result(sop::LOAD, c.v4, &[p])
        })
    };
    validate(&build(true, true)).unwrap();
    expect_err(&build(false, true), "DescriptorSet/Binding");
    expect_err(&build(true, false), "Block");
}

#[test]
fn fragment_only_instructions_in_vertex_shaders() {
    let build = |execution: u32| {
        let mut b = Builder::new();
        let void = b.t_void();
        let v4 = b.t_vec4();
        let f = b.t_float();
        let v2 = b.t_vector(f, 2);
        let fnty = b.t_function(void, &[]);
        let img = b.t_image(dim::D2, false);
        let si = b.t_sampled_image(img);
        let ptr = b.t_pointer(storage::UNIFORM_CONSTANT, si);
        let var = b.global_variable(ptr, storage::UNIFORM_CONSTANT, None);
        b.decorate(var, decoration::DESCRIPTOR_SET, &[1]);
        b.decorate(var, decoration::BINDING, &[0]);
        let half = b.c_float(0.5);
        let coord = b.c_composite(v2, &[half, half]);
        let main = b.id();
        b.begin_function(main, void, fnty);
        let s = b.result(sop::LOAD, si, &[var]);
        b.result(sop::IMAGE_SAMPLE_IMPLICIT_LOD, v4, &[s, coord]);
        b.result(
            sop::IMAGE_SAMPLE_EXPLICIT_LOD,
            v4,
            &[s, coord, image_operands::LOD, half],
        );
        b.end_function();
        b.entry_point(execution, main, "main", &[]);
        if execution == model::FRAGMENT {
            b.execution_mode(main, mode::ORIGIN_UPPER_LEFT);
        }
        b.assemble()
    };
    validate(&build(model::FRAGMENT)).unwrap();
    expect_err(&build(model::VERTEX), "fragment-only");
}

#[test]
fn image_operand_rules() {
    let build = |operands: Vec<u32>, explicit: bool| {
        fragment(move |b, c| {
            let v2 = b.t_vector(c.f, 2);
            let img = b.t_image(dim::D2, false);
            let si = b.t_sampled_image(img);
            let ptr = b.t_pointer(storage::UNIFORM_CONSTANT, si);
            let var = b.global_variable(ptr, storage::UNIFORM_CONSTANT, None);
            b.decorate(var, decoration::DESCRIPTOR_SET, &[1]);
            b.decorate(var, decoration::BINDING, &[0]);
            let coord = b.c_composite(v2, &[c.one, c.one]);
            let s = b.result(sop::LOAD, si, &[var]);
            let mut all = vec![s, coord];
            all.extend(
                operands
                    .iter()
                    .map(|&x| if x == u32::MAX { c.one } else { x }),
            );
            let opc = if explicit {
                sop::IMAGE_SAMPLE_EXPLICIT_LOD
            } else {
                sop::IMAGE_SAMPLE_IMPLICIT_LOD
            };
            b.result(opc, c.v4, &all)
        })
    };
    validate(&build(vec![image_operands::BIAS, u32::MAX], false)).unwrap();
    validate(&build(vec![image_operands::LOD, u32::MAX], true)).unwrap();
    expect_err(&build(vec![], true), "without Lod or Grad");
    expect_err(
        &build(vec![image_operands::LOD, u32::MAX], false),
        "implicit-LOD sample with Lod",
    );
    expect_err(
        &build(vec![image_operands::GRAD, u32::MAX], true),
        "count does not match",
    );
}

#[test]
fn edits_to_translator_output_are_caught() {
    let words = {
        let mut a = Asm::ps_basic();
        a.flow(op::IF, 0, &[b(0)]);
        a.mov(oc(0), v(0));
        a.flow(op::ENDIF, 0, &[]);
        a.end()
    };
    let (m, _) = check(&words);
    let bad = edit(&m.words, |o, _| o == sop::SELECTION_MERGE, |_| None);
    expect_err(&bad, "without a structured merge");
    let bad = edit(&m.words, |o, _| o == sop::EXECUTION_MODE, |_| None);
    expect_err(&bad, "OriginUpperLeft");
    let bad = edit(
        &m.words,
        |o, ops| o == sop::DECORATE && ops[1] == decoration::BINDING,
        |_| None,
    );
    expect_err(&bad, "DescriptorSet/Binding");
}
