// original: 0x00b05d80 angle_between_vectors
/// Returns the angle between two 3-vectors, or zero for degenerate inputs.
///
/// The squared lengths multiply to a product; a zero product yields a zero
/// reciprocal length, otherwise the reciprocal of its square root. The dot
/// product times that reciprocal is clamped to the unit interval carried in
/// the image (a not-a-number passes the clamp through unchanged). When the
/// product is below a tiny epsilon, or unordered, the result is zero without
/// calling the arc-cosine helper; otherwise the helper maps the clamped value
/// and its float answer is returned on the x87 stack.
use core::hint::black_box;
use lf_checker_rt::{callee_cdecl, export, relocated};

export!(thiscall, rw_00b05d80(this_ptr: u32, arg_ptr: u32) -> f32 {
    unsafe {
        /// Unit value: clamp bound and reciprocal-length numerator.
        const UNIT: u32 = 0x00FE_88E8;
        /// Lower clamp bound.
        const CLAMP_LO: u32 = 0x00FE_8D94;
        /// Length-product gate: below this the angle is zero.
        const GATE: u32 = 0x00FE_863C;
        /// Arc-cosine helper callee id.
        const ACOS: u32 = 1;

        let x1 = *((this_ptr.wrapping_add(0)) as *const f32);
        let y1 = *((this_ptr.wrapping_add(4)) as *const f32);
        let z1 = *((this_ptr.wrapping_add(8)) as *const f32);
        let x2 = *((arg_ptr.wrapping_add(0)) as *const f32);
        let y2 = *((arg_ptr.wrapping_add(4)) as *const f32);
        let z2 = *((arg_ptr.wrapping_add(8)) as *const f32);

        let xx1 = black_box(x1) * black_box(x1);
        let yy1 = black_box(y1) * black_box(y1);
        let n1 = black_box(black_box(xx1) + black_box(yy1)) + black_box(black_box(z1) * black_box(z1));
        let xx2 = black_box(x2) * black_box(x2);
        let yy2 = black_box(y2) * black_box(y2);
        let n2 = black_box(black_box(xx2) + black_box(yy2)) + black_box(black_box(z2) * black_box(z2));
        let prod = black_box(n1) * black_box(n2);

        let one = *(relocated(UNIT) as *const f32);
        // The original takes the square-root path for any product that is not
        // ordered-equal to zero, including not-a-number.
        let inv = if !(prod == 0.0) {
            black_box(one) / black_box(black_box(prod).sqrt())
        } else {
            0.0
        };

        let dx = black_box(x2) * black_box(x1);
        let dy = black_box(y2) * black_box(y1);
        let dz = black_box(z2) * black_box(z1);
        let dot = black_box(black_box(dx) + black_box(dy)) + black_box(dz);
        let scaled = black_box(dot) * black_box(inv);

        let lo = *(relocated(CLAMP_LO) as *const f32);
        let hi = *(relocated(UNIT) as *const f32);
        let clamped = if black_box(lo) > black_box(scaled) {
            lo
        } else if !(black_box(scaled) > black_box(hi)) {
            scaled
        } else {
            hi
        };

        let gate = *(relocated(GATE) as *const f32);
        let gated = black_box(prod) - black_box(gate);
        // Below-or-unordered takes the zero path.
        if !(gated >= 0.0) {
            0.0
        } else {
            let answer: u32 = callee_cdecl!(ACOS, u32, clamped.to_bits());
            f32::from_bits(answer)
        }
    }
});
