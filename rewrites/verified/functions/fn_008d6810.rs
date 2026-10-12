// original: 0x008D6810 paired_polar_angles
//! Computes a masked polar angle for each non-null input vector, preserves a
//! prior output when its vector is null, then wraps both outputs and their
//! difference through the angle helper.

use core::hint::black_box;

const SIGN_MASK_VA: u32 = 0x00FE8FA0;
const ATAN2_CALLEE: u32 = 1;
const WRAP_CALLEE: u32 = 2;

unsafe fn angle_from_vector(vector: *const f32, sign_mask: u32) -> f32 {
    let x_bits = unsafe { (vector as *const u32).read() } ^ sign_mask;
    let y = unsafe { vector.add(1).read() };
    let x = f32::from_bits(x_bits);
    let x64 = f64::from(x).to_bits();
    let y64 = f64::from(y).to_bits();
    let angle_bits = lf_checker_rt::callee_cdecl!(
        ATAN2_CALLEE,
        u64,
        x64 as u32,
        (x64 >> 32) as u32,
        y64 as u32,
        (y64 >> 32) as u32,
    );
    f64::from_bits(angle_bits) as f32
}

lf_checker_rt::export!(cdecl, rw_fn_008d6810(
    first_vector: *const f32,
    second_vector: *const f32,
    first_out: *mut f32,
    second_out: *mut f32,
) -> f32 {
    unsafe {
        let sign_mask = lf_checker_rt::global::<u32>(SIGN_MASK_VA).read();
        if !first_vector.is_null() {
            first_out.write(angle_from_vector(first_vector, sign_mask));
        }
        if !second_vector.is_null() {
            second_out.write(angle_from_vector(second_vector, sign_mask));
        }

        let first = lf_checker_rt::callee_cdecl!(
            WRAP_CALLEE,
            f32,
            first_out.read().to_bits(),
        );
        first_out.write(first);

        let second = lf_checker_rt::callee_cdecl!(
            WRAP_CALLEE,
            f32,
            second_out.read().to_bits(),
        );
        second_out.write(second);

        let delta = black_box(second_out.read()) - black_box(first_out.read());
        let wrapped_delta = lf_checker_rt::callee_cdecl!(
            WRAP_CALLEE,
            f32,
            delta.to_bits(),
        );
        wrapped_delta
    }
});
