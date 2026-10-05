// original: 0x00c3ef40 train_clamped_ratio_or_one (proposed)
/// Return 1.0 for a zero input, else a ratio clamped above by 1.0.
///
/// `this` (ECX) points to the car. When the float at `+0x10` compares
/// equal to +0.0 (either zero; NaN takes the other path), returns 1.0 on
/// the x87 stack. Otherwise computes `f1 = (float)(G - [this+0x1c]) /
/// ((float)[this+0x14] * [this+0x10])` with G a global, in the original's
/// operand order (pinned), and returns f1 when it is below 1.0, else 1.0,
/// likewise on the x87 stack (a float widened exactly to double).
/// Integer-to-float conversions and division by zero follow x86/SSE
/// semantics (no faults). Bit-exact for all inputs including NaN.
///
/// Original: 0x00c3ef40 (thiscall, no stack words; x87 double result).
lf_checker_rt::export!(thiscall, rw_00c3ef40(this: u32) -> f64 {
    unsafe {
        const X: u32 = 0x10;
        const A: u32 = 0x14;
        const B: u32 = 0x1c;
        const GLOB: u32 = 0x11735c4;
        #[inline(always)]
        fn mul(x: f32, y: f32) -> f32 {
            core::hint::black_box(x) * core::hint::black_box(y)
        }
        #[inline(always)]
        fn div(x: f32, y: f32) -> f32 {
            core::hint::black_box(x) / core::hint::black_box(y)
        }
        let x = f32::from_bits(((this + X) as *const u32).read_unaligned());
        if x == 0.0 {
            return 1.0;
        }
        let g = lf_checker_rt::global::<u32>(GLOB).read_unaligned();
        let b = ((this + B) as *const u32).read_unaligned();
        let f1 = ((g.wrapping_sub(b) as i32) as f32);
        let f0 = ((((this + A) as *const u32).read_unaligned() as i32) as f32);
        let r = div(f1, mul(f0, x));
        // jbe after comiss is taken for below-or-equal AND unordered,
        // so a NaN ratio also returns 1.0.
        if r.is_nan() || 1.0f32 <= r {
            1.0
        } else {
            r as f64
        }
    }
});
