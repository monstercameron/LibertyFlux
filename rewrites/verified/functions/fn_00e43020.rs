// original: 0x00E43020 aligned_frame_draw_c
//! Specification: fetch runtime bounds, clamp the right and bottom edges,
//! draw two rotated four-corner passes at four positions, conditionally
//! draw an inset border, then finalize the draw state.
#![allow(unsafe_code)]

use lf_checker_rt::{callee_cdecl, callee_thiscall, export, global};

const CALLEE_INITIALIZE: u32 = 1;
const CALLEE_FETCH_BOUNDS: u32 = 2;
const CALLEE_REFRESH: u32 = 3;
const CALLEE_OPTION: u32 = 4;
const CALLEE_ANGLE_A: u32 = 5;
const CALLEE_ANGLE_B: u32 = 6;
const CALLEE_DRAW_STATE: u32 = 7;
const CALLEE_DRAW_MODE: u32 = 8;
const CALLEE_DRAW_VERTEX: u32 = 9;
const CALLEE_POST_DRAW_A: u32 = 10;
const CALLEE_POST_DRAW_B: u32 = 11;
const CALLEE_FINALIZE: u32 = 12;
const CALLEE_ZERO_OPTION: u32 = 13;
const CALLEE_COOKIE: u32 = 14;

const HIGH_LIMIT: u32 = 0x00E82784;
const LOW_LIMIT: u32 = 0x00E82780;
const SCALE_FACTOR: u32 = 0x00FE8830;
const ANGLE_STEP: u32 = 0x00FE8978;
const INNER_MARGIN: u32 = 0x00FE8A24;

const DIRECTIONS: [(u32, u32); 4] = [
    (0x3F80_0000, 0xBF80_0000),
    (0x3F80_0000, 0x3F80_0000),
    (0xBF80_0000, 0x3F80_0000),
    (0xBF80_0000, 0xBF80_0000),
];

const MAIN_TEXCOORDS: [u32; 8] = [
    0x3F7A_E148, 0x3F7A_E148,
    0x3CA3_D70A, 0x3F7A_E148,
    0x3CA3_D70A, 0x3CA3_D70A,
    0x3F7A_E148, 0x3CA3_D70A,
];

const BORDER_TEXCOORDS: [u32; 8] = [
    0x3CA3_D70A, 0x3F7A_E148,
    0x3CA3_D70A, 0x3CA3_D70A,
    0x3F7A_E148, 0x3F7A_E148,
    0x3F7A_E148, 0x3CA3_D70A,
];

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
unsafe fn read_game_float(file_va: u32) -> f32 {
    unsafe { global::<f32>(file_va).read_unaligned() }
}

#[inline(always)]
fn clamp_lower(value: f32, lower_bound: f32) -> f32 {
    if lower_bound > value {
        lower_bound
    } else {
        value
    }
}

/// Apply the two runtime angle helpers to one origin and its four signed corners.
unsafe fn rotated_quad(
    angle: f32,
    origin: (f32, f32),
    scale_x: f32,
    scale_y: f32,
) -> [(f32, f32); 4] {
    let helper_a_bits = callee_cdecl!(CALLEE_ANGLE_A, u32, angle.to_bits());
    let helper_b_bits = callee_cdecl!(CALLEE_ANGLE_B, u32, angle.to_bits());
    let helper_a = f32::from_bits(helper_a_bits);
    let helper_b = f32::from_bits(helper_b_bits);
    let mut points = [(0.0_f32, 0.0_f32); 4];

    for index in 0..4 {
        let (unit_x_bits, unit_y_bits) = DIRECTIONS[index];
        let unit_x = f32::from_bits(unit_x_bits);
        let unit_y = f32::from_bits(unit_y_bits);

        let x_first = mul_f32(unit_x, helper_a);
        let x_second = mul_f32(helper_b, unit_y);
        let x_delta = sub_f32(x_first, x_second);
        let scaled_x = mul_f32(x_delta, scale_x);
        let point_x = add_f32(scaled_x, origin.0);

        let y_first = mul_f32(helper_b, unit_x);
        let y_second = mul_f32(helper_a, unit_y);
        let y_delta = add_f32(y_first, y_second);
        let scaled_y = mul_f32(y_delta, scale_y);
        let point_y = add_f32(scaled_y, origin.1);
        points[index] = (point_x, point_y);
    }
    points
}

/// Emit one four-point draw with the requested texture coordinates.
unsafe fn emit_quad(
    this: u32,
    state_slot: u32,
    draw_mode: (u32, u32),
    points: [(f32, f32); 4],
    texture: [u32; 8],
    selector: u32,
    wrong_first_vertex: bool,
    mutation_pending: &mut bool,
) {
    let state = this.wrapping_add(0x38).wrapping_add(state_slot.wrapping_mul(4));
    let _ = callee_thiscall!(CALLEE_DRAW_STATE, u32, state);
    let _ = callee_cdecl!(CALLEE_DRAW_MODE, u32, draw_mode.0, draw_mode.1);

    for index in 0..4 {
        let (mut x, y) = points[index];
        if wrong_first_vertex && *mutation_pending {
            x = add_f32(x, 1.0);
            *mutation_pending = false;
        }
        let uv = index * 2;
        let _ = callee_cdecl!(
            CALLEE_DRAW_VERTEX,
            u32,
            x.to_bits(),
            y.to_bits(),
            0,
            0,
            0,
            0xBF80_0000,
            selector,
            texture[uv],
            texture[uv + 1]
        );
    }
    let _ = callee_cdecl!(CALLEE_POST_DRAW_A, u32,);
    let _ = callee_cdecl!(CALLEE_POST_DRAW_B, u32,);
}

/// Draw a clipped rectangle through two rotating passes and an optional inset border.
unsafe fn run_draw(
    this: u32,
    left_arg: f32,
    top_arg: f32,
    mut right_arg: f32,
    mut bottom_arg: f32,
    selector: u32,
    wrong_first_vertex: bool,
) {
    let _ = callee_cdecl!(CALLEE_INITIALIZE, u32,);

    let mut bounds_words = [0x3C03_126F_u32; 2];
    let _ = callee_cdecl!(
        CALLEE_FETCH_BOUNDS,
        u32,
        7,
        0,
        bounds_words.as_mut_ptr() as u32,
        0
    );
    let half_width = f32::from_bits(bounds_words[0]);
    let half_height = f32::from_bits(bounds_words[1]);

    let high_limit = unsafe { read_game_float(HIGH_LIMIT) };
    let low_limit = unsafe { read_game_float(LOW_LIMIT) };
    right_arg = clamp_lower(right_arg, high_limit);
    bottom_arg = clamp_lower(bottom_arg, low_limit);

    let top_origin = add_f32(half_height, top_arg);
    let bottom_delta = sub_f32(bottom_arg, half_height);
    let left_origin = add_f32(half_width, left_arg);
    let right_delta = sub_f32(right_arg, half_width);

    let _ = callee_cdecl!(CALLEE_REFRESH, u32,);
    let _ = callee_cdecl!(CALLEE_OPTION, u32, 7, 1);
    let _ = callee_cdecl!(CALLEE_OPTION, u32, 2, 0);
    let _ = callee_cdecl!(CALLEE_OPTION, u32, 15, 15);

    let right_edge = add_f32(left_origin, right_delta);
    let bottom_edge = add_f32(top_origin, bottom_delta);
    let right_origin = sub_f32(right_edge, half_width);
    let bottom_origin = sub_f32(bottom_edge, half_height);

    let scale_factor = unsafe { read_game_float(SCALE_FACTOR) };
    let scaled_right_delta = mul_f32(right_delta, scale_factor);
    let scaled_bottom_delta = mul_f32(bottom_delta, scale_factor);
    let x_orbit = sub_f32(
        add_f32(scaled_right_delta, left_origin),
        mul_f32(half_width, scale_factor),
    );
    let y_orbit = sub_f32(
        add_f32(scaled_bottom_delta, top_origin),
        mul_f32(half_height, scale_factor),
    );
    let x_scale = sub_f32(
        sub_f32(scaled_right_delta, half_width),
        mul_f32(half_width, scale_factor),
    );
    let y_scale = sub_f32(
        sub_f32(scaled_bottom_delta, half_height),
        mul_f32(half_height, scale_factor),
    );

    let primary_origins = [
        (left_origin, top_origin),
        (right_origin, top_origin),
        (right_origin, bottom_origin),
        (left_origin, bottom_origin),
    ];
    let secondary_origins = [
        (x_orbit, top_origin),
        (right_origin, y_orbit),
        (x_orbit, bottom_origin),
        (left_origin, y_orbit),
    ];
    let secondary_scales = [
        (x_scale, half_height),
        (half_width, y_scale),
        (x_scale, half_height),
        (half_width, y_scale),
    ];

    let angle_step = unsafe { read_game_float(ANGLE_STEP) };
    let mut angle = angle_step;
    let mut mutation_pending = true;
    for row in 0..4 {
        for mode in 0..2 {
            let (origin, sx, sy) = if mode == 0 {
                (primary_origins[row], half_width, half_height)
            } else {
                let (sx, sy) = secondary_scales[row];
                (secondary_origins[row], sx, sy)
            };
            let points = unsafe { rotated_quad(angle, origin, sx, sy) };
            unsafe {
                emit_quad(
                    this,
                    mode as u32,
                    (5, 4),
                    points,
                    MAIN_TEXCOORDS,
                    selector,
                    wrong_first_vertex,
                    &mut mutation_pending,
                );
            }
        }
        angle = add_f32(angle, angle_step);
    }

    let margin = unsafe { read_game_float(INNER_MARGIN) };
    let inner_left = add_f32(left_origin, half_width);
    let inner_top = add_f32(top_origin, half_height);
    let inner_right = sub_f32(right_edge, mul_f32(half_width, margin));
    let inner_bottom = sub_f32(bottom_edge, mul_f32(half_height, margin));
    if inner_left < inner_right && inner_top < inner_bottom {
        let border_points = [
            (inner_left, inner_top),
            (inner_left, inner_bottom),
            (inner_right, inner_top),
            (inner_right, inner_bottom),
        ];
        let _ = callee_thiscall!(
            CALLEE_DRAW_STATE,
            u32,
            this.wrapping_add(0x40)
        );
        let _ = callee_cdecl!(CALLEE_DRAW_MODE, u32, 4, 4);
        for index in 0..4 {
            let uv = index * 2;
            let _ = callee_cdecl!(
                CALLEE_DRAW_VERTEX,
                u32,
                border_points[index].0.to_bits(),
                border_points[index].1.to_bits(),
                0,
                0,
                0,
                0xBF80_0000,
                selector,
                BORDER_TEXCOORDS[uv],
                BORDER_TEXCOORDS[uv + 1]
            );
        }
        let _ = callee_cdecl!(CALLEE_POST_DRAW_A, u32,);
        let _ = callee_cdecl!(CALLEE_POST_DRAW_B, u32,);
    }

    let _ = callee_cdecl!(CALLEE_FINALIZE, u32,);
    let _ = callee_cdecl!(CALLEE_ZERO_OPTION, u32, 0);
    let _ = callee_cdecl!(CALLEE_COOKIE, (),);
}

/// Fetches runtime bounds, clamps two edges, emits two rotated four-corner passes
/// for each of four positions, and draws an inset border when the remaining area
/// has positive width and height.
export!(thiscall, rw_a_u15(
    this: u32,
    left: f32,
    top: f32,
    right: f32,
    bottom: f32,
    selector: u32
) -> () {
    unsafe { run_draw(this, left, top, right, bottom, selector, false); }
});

// Mutant: offset the first emitted vertex's x coordinate by one.
export!(thiscall, mut_a_u15(
    this: u32,
    left: f32,
    top: f32,
    right: f32,
    bottom: f32,
    selector: u32
) -> () {
    unsafe { run_draw(this, left, top, right, bottom, selector, true); }
});
