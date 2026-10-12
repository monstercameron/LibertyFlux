// original: 0x008D6270 shaped_index_ratio
//! Uses two scripted double helpers to shape an index-scaled input, applies
//! the sign-dependent adjustment and image factors, then returns the scaled
//! helper output as a single-precision x87 result.

use core::hint::black_box;

const BASE_DOUBLE_VA: u32 = 0x00FE8A78;
const ADJUST_FLOAT_VA: u32 = 0x00FE8AD8;
const SCALE_FLOAT_VA: u32 = 0x00FE879C;
const DOUBLE_CALLEE: u32 = 1;
const TRANSFORM_CALLEE: u32 = 2;

lf_checker_rt::export!(cdecl, rw_fn_008d6270(input_bits: u32, index: u32) -> f32 {
    let input = f32::from_bits(input_bits);
    let base = unsafe { lf_checker_rt::global::<f64>(BASE_DOUBLE_VA).read() };

    let incremented = (index.wrapping_add(1) as i32) as f32;
    let first_arg = f64::from(incremented).to_bits();
    let base_bits = base.to_bits();
    let first_answer = lf_checker_rt::callee_cdecl!(
        DOUBLE_CALLEE,
        u64,
        base_bits as u32,
        (base_bits >> 32) as u32,
        first_arg as u32,
        (first_arg >> 32) as u32,
    );
    let shaped = f64::from_bits(first_answer) as f32;
    let product = black_box(shaped) * black_box(input);

    let adjust = unsafe { lf_checker_rt::global::<f32>(ADJUST_FLOAT_VA).read() };
    let adjusted = if !(input < 0.0) {
        black_box(product) + black_box(adjust)
    } else {
        black_box(product) - black_box(adjust)
    };
    let scale = unsafe { lf_checker_rt::global::<f32>(SCALE_FLOAT_VA).read() };
    let scaled = black_box(adjusted) * black_box(scale);
    let transformed_bits = f64::from(scaled).to_bits();

    let mut transformed_double = 0u64;
    let output_pointer = (&mut transformed_double as *mut u64) as u32;
    let _discarded = lf_checker_rt::callee_cdecl!(
        TRANSFORM_CALLEE,
        f64,
        transformed_bits as u32,
        (transformed_bits >> 32) as u32,
        output_pointer,
    );

    let denominator_index = (index as i32) as f32;
    let denominator_arg = f64::from(denominator_index).to_bits();
    let denominator_base = unsafe { lf_checker_rt::global::<f64>(BASE_DOUBLE_VA).read() };
    let denominator_base_bits = denominator_base.to_bits();
    let denominator_answer = lf_checker_rt::callee_cdecl!(
        DOUBLE_CALLEE,
        u64,
        denominator_base_bits as u32,
        (denominator_base_bits >> 32) as u32,
        denominator_arg as u32,
        (denominator_arg >> 32) as u32,
    );

    let numerator = f64::from_bits(transformed_double) as f32;
    let denominator = f64::from_bits(denominator_answer) as f32;
    black_box(numerator) / black_box(denominator)
});
