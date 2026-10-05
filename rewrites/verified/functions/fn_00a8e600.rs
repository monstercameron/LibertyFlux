// original: 0x00A8E600 distance_fade_factor (proposed)

/// Fade factor from a distance: 1 when forced, else `1 - x/4` cut at 4.
///
/// Returns 1.0 when the force byte at `this+0x72` is set. Otherwise loads
/// `x` from `this+0x114`: values above 4.0 (from the read-only constant)
/// yield 0.0, anything else yields `1.0 - x * 0.25` in that operand order,
/// NaN included (an unordered comparison falls through to the multiply
/// path). The result leaves in ST0. No calls.
///
/// Original: thiscall, no stack arguments, f32 in ST0.
lf_checker_rt::export!(thiscall, rw_00A8E600(this: u32) -> f32 {
    unsafe {
        const FORCE_OFF: u32 = 0x72;
        const X_OFF: u32 = 0x114;
        const LIMIT_BITS: u32 = 0xfe8ab8;
        const SLOPE_BITS: u32 = 0xfe87e4;
        const ONE_BITS: u32 = 0xfe88e8;
        if ((this + FORCE_OFF) as *const u8).read() != 0 {
            return 1.0;
        }
        let x = ((this + X_OFF) as *const f32).read_unaligned();
        let limit = f32::from_bits(
            (lf_checker_rt::relocated(LIMIT_BITS) as *const u32).read_unaligned(),
        );
        if x > limit {
            return 0.0;
        }
        let slope = f32::from_bits(
            (lf_checker_rt::relocated(SLOPE_BITS) as *const u32).read_unaligned(),
        );
        let one = f32::from_bits(
            (lf_checker_rt::relocated(ONE_BITS) as *const u32).read_unaligned(),
        );
        core::hint::black_box(one)
            - core::hint::black_box(
                core::hint::black_box(x) * core::hint::black_box(slope),
            )
    }
});
