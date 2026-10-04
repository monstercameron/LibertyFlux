//! Two hand-written programs of realistic shape: a skinned, lit vertex
//! shader and a textured, alpha-tested pixel shader. They are written
//! here from scratch in the public token format; they do not come from
//! the game.

mod common;

use common::programs::{skinned_lit_vertex_shader, textured_pixel_shader};

type UsageLocation = (Option<(u8, u8)>, Option<u32>);
use common::*;
use lf_dxso_spirv::spirv::{builtin, decoration, glsl, mode, op as sop};
use lf_dxso_spirv::{BuiltIn, TextureDim};

#[test]
fn skinned_lit_vertex_shader_translates() {
    let words = skinned_lit_vertex_shader();
    let shader = lf_dxso_spirv::decode(&words).unwrap();
    assert_eq!(shader.instructions.len(), 9 + 2 + 20); // dcl, def, code
    let (m, rep) = check(&words);
    assert_eq!(rep.functions, 1);

    // Interface: five vertex inputs at the convention's locations.
    let ins: Vec<UsageLocation> = m.inputs.iter().map(|x| (x.usage, x.location)).collect();
    assert_eq!(
        ins,
        vec![
            (Some((0, 0)), Some(0)),
            (Some((3, 0)), Some(3)),
            (Some((5, 0)), Some(6)),
            (Some((2, 0)), Some(2)),
        ]
    );
    let outs: Vec<(Option<u32>, Option<BuiltIn>)> =
        m.outputs.iter().map(|x| (x.location, x.builtin)).collect();
    assert_eq!(
        outs,
        vec![
            (None, Some(BuiltIn::Position)),
            (Some(0), None),
            (Some(1), None),
            (Some(10), None),
            (Some(12), None),
        ]
    );
    assert_eq!(
        decoration_of(&m, "o0_position0", decoration::BUILT_IN),
        Some(vec![builtin::POSITION])
    );
    // Constants: relative reads (skinning, lights), int from defi, one float def.
    assert!(m.constants.float_relative);
    assert_eq!(m.constants.float_defined, vec![95]);
    assert_eq!(m.constants.int_defined, vec![0]);
    assert!(m.constants.int_read.is_empty());
    assert_eq!(m.float_constant_binding, Some(0));
    assert_eq!(m.int_bool_constant_binding, None);
    assert_eq!(m.push_constant_size, 16);
    // Shape of the code.
    assert_eq!(count(&m, sop::LOOP_MERGE), 1);
    // m4x3 (3) + m4x4 (4) + m3x3 (3) + nrm (1) + dp3 (1) + dp4 (1).
    assert_eq!(count(&m, sop::DOT), 13);
    // Relative reads: 3 rows of skinning + 2 per light iteration body.
    assert_eq!(count_ext(&m, glsl::SCLAMP), 5);
    assert_eq!(count(&m, sop::CONVERT_F_TO_S), 1);
    // Predicated move: one select on the bvec4 predicate.
    assert!(count(&m, sop::SELECT) >= 1);
    assert_eq!(count(&m, sop::KILL), 0);
}

#[test]
fn textured_pixel_shader_translates() {
    let words = textured_pixel_shader();
    let (m, rep) = check(&words);
    assert_eq!(rep.functions, 1);
    let samplers: Vec<(u16, TextureDim, u32, u32)> = m
        .samplers
        .iter()
        .map(|s| (s.register, s.dim, s.set, s.binding))
        .collect();
    assert_eq!(
        samplers,
        vec![(0, TextureDim::D2, 1, 0), (1, TextureDim::Cube, 1, 1)]
    );
    let ins: Vec<Option<u32>> = m.inputs.iter().map(|x| x.location).collect();
    assert_eq!(ins, vec![Some(0), Some(1), Some(10), Some(12), None]);
    assert_eq!(m.inputs[4].builtin, Some(BuiltIn::FrontFacing));
    assert_eq!(m.outputs.len(), 1);
    assert_eq!(m.outputs[0].location, Some(0));
    assert_eq!(count(&m, sop::IMAGE_SAMPLE_IMPLICIT_LOD), 2);
    assert_eq!(count(&m, sop::KILL), 1);
    assert_eq!(count(&m, sop::SELECTION_MERGE), 2); // texkill + if
    assert!(execution_modes(&m).contains(&mode::ORIGIN_UPPER_LEFT));
    assert!(!m.depth_replacing);
    assert_eq!(m.constants.float_read, vec![5, 6]);
    assert_eq!(m.float_constant_binding, Some(1));
}
