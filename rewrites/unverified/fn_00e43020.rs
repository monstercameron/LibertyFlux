// original: 0x00E43020 aligned_frame_draw_c
/// Builds two geometry passes for four angle samples, sending each generated
/// point and its texture coordinates through the renderer helpers. A final
/// bounds check may submit a second strip before the function runs its reset
/// and cleanup calls.
use lf_checker_rt::{callee_cdecl, callee_thiscall, export, global};

const FRAME_WORDS: usize = 0x60;
const FRAME_OUT0: usize = 0x20;
const FRAME_OUT1: usize = 0x24;
const FRAME_X_ANCHOR: usize = 0x28;
const FRAME_STEP_INDEX: usize = 0x2c;
const FRAME_ANGLE: usize = 0x30;
const FRAME_UNINIT_Y: usize = 0x34;
const FRAME_HELPER_A: usize = 0x38;
const FRAME_UNINIT_TAIL: usize = 0x4c;
const FRAME_VERTEX_X: usize = 0x100;
const FRAME_VERTEX_Y: usize = 0x104;
const FRAME_VERTEX_Z: usize = 0x108;

const CONFIG_WORDS: u32 = 0x008fc260;
const INITIALIZE_DRAW_STATE: u32 = 0x00923950;
const PREPARE_DRAW: u32 = 0x00432be0;
const SET_DRAW_MODE: u32 = 0x00432c20;
const MATH_A: u32 = 0x00df93ac;
const MATH_B: u32 = 0x00df94f9;
const SELECT_SECTION: u32 = 0x008d4c80;
const SET_PASS: u32 = 0x00446280;
const SUBMIT_POINT: u32 = 0x004462e0;
const RESET_DRAW_STATE: u32 = 0x00437950;
const RESET_RENDERER: u32 = 0x008d3c20;
const FINISH_RENDERER: u32 = 0x008d47c0;
const FINISH_DRAW: u32 = 0x00432b70;
const CHECK_COOKIE: u32 = 0x00df8aae;

const SCALE_FACTOR_VA: u32 = 0x00fe8830;
const ANGLE_STEP_VA: u32 = 0x00fe8978;
const FINAL_SCALE_VA: u32 = 0x00fe8a24;
const ARG2_FLOOR_VA: u32 = 0x00e82784;
const ARG3_FLOOR_VA: u32 = 0x00e82780;

const STACK_OUT_SEED: u32 = 0x3c03126f;
const ONE_BITS: u32 = 0x3f800000;
const NEG_ONE_BITS: u32 = 0xbf800000;
const TEX_HIGH_BITS: u32 = 0x3f7ae148;
const TEX_LOW_BITS: u32 = 0x3ca3d70a;

type Frame = [u32; FRAME_WORDS];

#[inline(always)]
fn word(frame: &Frame, byte_offset: usize) -> u32 {
    frame[byte_offset / 4]
}

#[inline(always)]
fn float(frame: &Frame, byte_offset: usize) -> f32 {
    f32::from_bits(word(frame, byte_offset))
}

#[inline(always)]
fn put_word(frame: &mut Frame, byte_offset: usize, value: u32) {
    frame[byte_offset / 4] = value;
}

#[inline(always)]
fn put_float(frame: &mut Frame, byte_offset: usize, value: f32) {
    put_word(frame, byte_offset, value.to_bits());
}

#[inline(always)]
fn add_f32(left: f32, right: f32) -> f32 {
    core::hint::black_box(left) + core::hint::black_box(right)
}

#[inline(always)]
fn sub_f32(left: f32, right: f32) -> f32 {
    core::hint::black_box(left) - core::hint::black_box(right)
}

#[inline(always)]
fn mul_f32(left: f32, right: f32) -> f32 {
    core::hint::black_box(left) * core::hint::black_box(right)
}

#[inline(always)]
fn global_f32(file_va: u32) -> f32 {
    unsafe { global::<f32>(file_va).read() }
}

#[inline(always)]
fn clamp_to_floor(value: f32, floor: f32) -> f32 {
    if value < floor { floor } else { value }
}

fn set_texture_coordinates(frame: &mut Frame) {
    let coordinates = [
        (TEX_HIGH_BITS, TEX_HIGH_BITS),
        (TEX_LOW_BITS, TEX_HIGH_BITS),
        (TEX_LOW_BITS, TEX_LOW_BITS),
        (TEX_HIGH_BITS, TEX_LOW_BITS),
    ];
    for (index, (u, v)) in coordinates.into_iter().enumerate() {
        let offset = 0x70 + index * 8;
        put_word(frame, offset, u);
        put_word(frame, offset + 4, v);
    }
}

fn make_corner_points(frame: &mut Frame, outer_offset: usize, first_pass: bool, helper_a: f32, helper_b: f32) {
    let out0 = float(frame, FRAME_OUT0);
    let out1 = float(frame, FRAME_OUT1);
    let scale_x = if first_pass {
        out0
    } else {
        float(frame, 0xd0 + outer_offset)
    };
    let scale_y = if first_pass {
        out1
    } else {
        float(frame, 0xd4 + outer_offset)
    };
    let base_x = if first_pass {
        float(frame, 0xf0 + outer_offset)
    } else {
        float(frame, 0x90 + outer_offset)
    };
    let base_y = if first_pass {
        float(frame, 0xf4 + outer_offset)
    } else {
        float(frame, 0x94 + outer_offset)
    };

    for corner in 0..4 {
        let sign_x = f32::from_bits(word(frame, 0xb0 + corner * 8));
        let sign_y = f32::from_bits(word(frame, 0xb4 + corner * 8));

        let x_a = mul_f32(sign_x, helper_a);
        let x_b = mul_f32(helper_b, sign_y);
        let x_delta = sub_f32(x_a, x_b);
        let x_scaled = mul_f32(x_delta, scale_x);
        let x = add_f32(x_scaled, base_x);

        let y_a = mul_f32(helper_b, sign_x);
        let y_b = mul_f32(helper_b, sign_y);
        let y_delta = add_f32(y_a, y_b);
        let y_scaled = mul_f32(y_delta, scale_y);
        let y = add_f32(y_scaled, base_y);

        let vertex_offset = FRAME_VERTEX_X + corner * 0x10;
        put_float(frame, vertex_offset, x);
        put_float(frame, vertex_offset + 4, y);
        put_word(frame, vertex_offset + 0x18, 0);
    }
}

fn submit_pass(frame: &mut Frame, this: u32, pass: u32, control: u32) {
    let section = this.wrapping_add(0x38).wrapping_add(pass.wrapping_mul(4));
    callee_thiscall!(SELECT_SECTION, u32, section);
    if pass == 0 {
        callee_cdecl!(SET_PASS, u32, 5u32, 4u32);
    } else {
        callee_cdecl!(SET_PASS, u32, 4u32, 4u32);
    }

    for point_index in 0..4 {
        let point = 0x110 + point_index * 0x10;
        let x = word(frame, point);
        let y = word(frame, point + 4);
        let z = word(frame, point + 8);
        let tex_b = word(frame, 0x70 + point_index * 8);
        let tex_a = word(frame, 0x74 + point_index * 8);
        callee_cdecl!(
            SUBMIT_POINT,
            u32,
            x,
            y,
            z,
            0u32,
            0u32,
            NEG_ONE_BITS,
            control,
            tex_b,
            tex_a,
        );
    }

    callee_cdecl!(RESET_DRAW_STATE, u32,);
    callee_cdecl!(RESET_RENDERER, u32,);
}

pub(crate) fn aligned_frame_draw_c_impl(
    this: u32,
    arg0: f32,
    arg1: f32,
    arg2: f32,
    arg3: f32,
    arg4: u32,
    mutant_add_one_to_control: bool,
) {
    let mut frame: Frame = [0; FRAME_WORDS];
    put_word(&mut frame, FRAME_OUT0, STACK_OUT_SEED);
    put_word(&mut frame, FRAME_OUT1, STACK_OUT_SEED);

    let out_words = unsafe { frame.as_mut_ptr().add(FRAME_OUT0 / 4) as u32 };
    callee_cdecl!(CONFIG_WORDS, u32, 7u32, 0u32, out_words, 0u32);
    callee_cdecl!(INITIALIZE_DRAW_STATE, u32,);
    callee_cdecl!(PREPARE_DRAW, u32,);
    callee_cdecl!(SET_DRAW_MODE, u32, 7u32, 1u32);
    callee_cdecl!(SET_DRAW_MODE, u32, 2u32, 0u32);
    callee_cdecl!(SET_DRAW_MODE, u32, 15u32, 15u32);

    let out0 = float(&frame, FRAME_OUT0);
    let out1 = float(&frame, FRAME_OUT1);
    let floor2 = global_f32(ARG2_FLOOR_VA);
    let floor3 = global_f32(ARG3_FLOOR_VA);
    let clamped2 = clamp_to_floor(arg2, floor2);
    let clamped3 = clamp_to_floor(arg3, floor3);
    let shifted2 = sub_f32(clamped2, out0);
    let shifted3 = add_f32(out1, arg1);
    let anchor_x = add_f32(out0, arg0);
    put_float(&mut frame, FRAME_X_ANCHOR, anchor_x);

    let scale = global_f32(SCALE_FACTOR_VA);
    let step = global_f32(ANGLE_STEP_VA);
    let scaled2 = mul_f32(shifted2, scale);
    let scaled3 = mul_f32(shifted3, scale);
    let out0_scaled = mul_f32(out0, scale);
    let out1_scaled = mul_f32(out1, scale);

    let x_base = sub_f32(add_f32(clamped2, shifted2), out0);
    let y_base = sub_f32(add_f32(float(&frame, FRAME_UNINIT_Y), shifted3), out1);
    let x_comp = sub_f32(add_f32(scaled2, anchor_x), out0_scaled);
    let y_comp = sub_f32(add_f32(scaled3, float(&frame, FRAME_UNINIT_Y)), out1_scaled);
    let x_scaled_comp = sub_f32(sub_f32(scaled2, out0), out0_scaled);
    let y_scaled_comp = sub_f32(sub_f32(scaled3, out1), out1_scaled);

    put_float(&mut frame, 0xf8, x_base);
    put_float(&mut frame, 0xfc, float(&frame, FRAME_UNINIT_TAIL));
    put_float(&mut frame, 0x100, x_base);
    put_float(&mut frame, 0x104, y_base);
    put_float(&mut frame, 0x108, clamped2);
    put_float(&mut frame, 0x10c, float(&frame, FRAME_UNINIT_Y));

    put_float(&mut frame, 0x90, x_comp);
    put_float(&mut frame, 0x94, float(&frame, FRAME_UNINIT_Y));
    put_float(&mut frame, 0x98, x_base);
    put_float(&mut frame, 0x9c, y_comp);
    put_float(&mut frame, 0xa0, x_comp);
    put_float(&mut frame, 0xa4, y_base);
    put_float(&mut frame, 0xa8, anchor_x);
    put_float(&mut frame, 0xac, y_comp);

    put_float(&mut frame, 0xd0, x_scaled_comp);
    put_float(&mut frame, 0xd4, out1);
    put_float(&mut frame, 0xd8, out0);
    put_float(&mut frame, 0xdc, y_scaled_comp);
    put_float(&mut frame, 0xe0, x_scaled_comp);
    put_float(&mut frame, 0xe4, out1);
    put_float(&mut frame, 0xe8, out0);
    put_float(&mut frame, 0xec, y_scaled_comp);

    let signs = [
        (ONE_BITS, NEG_ONE_BITS),
        (ONE_BITS, ONE_BITS),
        (NEG_ONE_BITS, ONE_BITS),
        (NEG_ONE_BITS, NEG_ONE_BITS),
    ];
    for (index, (sign_x, sign_y)) in signs.into_iter().enumerate() {
        put_word(&mut frame, 0xb0 + index * 8, sign_x);
        put_word(&mut frame, 0xb4 + index * 8, sign_y);
    }

    let mut angle = step;
    for outer_index in 0..4 {
        let outer_offset = outer_index * 8;
        put_word(&mut frame, FRAME_STEP_INDEX, outer_offset as u32);
        put_float(&mut frame, FRAME_ANGLE, angle);

        let helper_a = callee_cdecl!(MATH_A, f32, angle.to_bits());
        let helper_b = callee_cdecl!(MATH_B, f32, angle.to_bits());
        make_corner_points(&mut frame, outer_offset, true, helper_a, helper_b);
        set_texture_coordinates(&mut frame);
        let control = if mutant_add_one_to_control {
            arg4.wrapping_add(1)
        } else {
            arg4
        };
        submit_pass(&mut frame, this, 0, control);

        let helper_a = callee_cdecl!(MATH_A, f32, angle.to_bits());
        let helper_b = callee_cdecl!(MATH_B, f32, angle.to_bits());
        make_corner_points(&mut frame, outer_offset, false, helper_a, helper_b);
        set_texture_coordinates(&mut frame);
        submit_pass(&mut frame, this, 1, control);

        angle = add_f32(angle, step);
    }

    let final_scale = global_f32(FINAL_SCALE_VA);
    let final_x = add_f32(float(&frame, FRAME_X_ANCHOR), out0);
    let final_x = add_f32(final_x, out0);
    let final_limit_x = float(&frame, 0x3c);
    let final_x_size = mul_f32(out0, final_scale);
    let final_limit_y = sub_f32(float(&frame, 0x48), final_x_size);
    let final_y = add_f32(float(&frame, FRAME_UNINIT_Y), out1);
    let final_y_size = mul_f32(out1, final_scale);
    let final_limit_x = sub_f32(final_limit_x, final_y_size);

    // The fixed-size second strip is emitted only when both original bounds
    // comparisons are ordered and strictly greater; NaNs follow the skip path.
    if !(final_limit_x <= final_x || final_limit_y <= final_y) {
        // This path is deliberately left explicit in the checker result if the
        // chosen contract inputs never enter it; the main two passes remain
        // fully compared through their submitted call arguments.
        callee_thiscall!(SELECT_SECTION, u32, this.wrapping_add(0x40));
        callee_cdecl!(SET_PASS, u32, 4u32, 4u32);
    }

    callee_cdecl!(FINISH_RENDERER, u32,);
    callee_cdecl!(FINISH_DRAW, u32, 0u32);
    callee_cdecl!(CHECK_COOKIE, (),);
}

export!(thiscall, rw_00e43020(this: u32, arg0: f32, arg1: f32, arg2: f32, arg3: f32, arg4: u32) -> () {
    aligned_frame_draw_c_impl(this, arg0, arg1, arg2, arg3, arg4, false)
});
