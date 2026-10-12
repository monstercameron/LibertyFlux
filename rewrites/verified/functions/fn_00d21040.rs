// original: 0x00D21040 angle_solver_atan2
//! Flips the first scalar input's sign mask, requests a double precision
//! angle, converts it to degrees, advances negative results by the configured
//! turn until they are nonnegative, then returns the classifier result.

use core::hint::black_box;

const SIGN_MASK_VA: u32 = 0x00FE8FA0;
const DEGREES_FACTOR_VA: u32 = 0x00E7C2A8;
const TURN_VA: u32 = 0x00FE8C1C;

lf_checker_rt::export!(cdecl, rw_fn_00d21040(first_bits: u32, second_bits: u32) -> u32 {
    let sign_mask = unsafe { lf_checker_rt::global::<u32>(SIGN_MASK_VA).read() };
    let first = f32::from_bits(first_bits ^ sign_mask);
    let second = f32::from_bits(second_bits);
    let first64 = f64::from(first).to_bits();
    let second64 = f64::from(second).to_bits();
    let angle64_bits = lf_checker_rt::callee_cdecl!(
        1,
        u64,
        first64 as u32,
        (first64 >> 32) as u32,
        second64 as u32,
        (second64 >> 32) as u32,
    );
    let radians = f64::from_bits(angle64_bits) as f32;
    let factor = unsafe { lf_checker_rt::global::<f32>(DEGREES_FACTOR_VA).read() };
    let mut degrees = black_box(radians) * black_box(factor);

    if degrees < 0.0 {
        let turn = unsafe { lf_checker_rt::global::<f32>(TURN_VA).read() };
        while degrees < 0.0 {
            degrees = black_box(degrees) + black_box(turn);
        }
    }

    lf_checker_rt::callee_cdecl!(2, u32, degrees.to_bits())
});
