// original: 0x008D6920 paired_heading_blend
//! Computes two polar angles from a three-float direction, passes each angle
//! through the angle wrapper, and stores the wrapped first angle and wrapped
//! difference in the two caller-provided output words.

use core::hint::black_box;

const ATAN2_CALLEE: u32 = 1;
const WRAP_CALLEE: u32 = 2;

unsafe fn angle_from_vector(vector: *const f32) -> f32 {
    let x = unsafe { vector.read() };
    let y = unsafe { vector.add(1).read() };
    let z = unsafe { vector.add(2).read() };

    let x_squared = black_box(x) * black_box(x);
    let y_squared = black_box(y) * black_box(y);
    let squared_radius = black_box(x_squared) + black_box(y_squared);
    let radius = black_box(squared_radius).sqrt();

    let vertical = f64::from(z).to_bits();
    let horizontal = f64::from(radius).to_bits();
    let angle_bits = lf_checker_rt::callee_cdecl!(
        ATAN2_CALLEE,
        u64,
        vertical as u32,
        (vertical >> 32) as u32,
        horizontal as u32,
        (horizontal >> 32) as u32,
    );
    f64::from_bits(angle_bits) as f32
}

lf_checker_rt::export!(cdecl, rw_fn_008d6920(
    first_vector: *const f32,
    second_vector: *const f32,
    first_out: *mut f32,
    second_out: *mut f32,
) -> () {
    unsafe {
        first_out.write(angle_from_vector(first_vector));
        second_out.write(angle_from_vector(second_vector));

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
        second_out.write(wrapped_delta);
    }
});
