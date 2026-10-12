// original: 0x00C61EA0 scale_by_frame_time_or_zero

/// Returns the frame-time input multiplied by the image's scale constant and
/// divided by the object's frame-time denominator, unless that denominator
/// is not strictly positive, in which case it returns positive zero. The
/// original's ordered compare branches to zero for equality and NaN. It
/// writes the result into its incoming stack argument before returning it on
/// the x87 stack; the rewrite can reproduce the returned `f32`, while the
/// contract observes that value and disables only the stack-slot comparison.
lf_checker_rt::export!(thiscall, rw_00c61ea0(this: u32, frame_time: f32) -> f32 {
    const FRAME_DENOMINATOR: u32 = 0x0C;
    const ZERO_VA: u32 = 0x00FE_8628;
    const SCALE_VA: u32 = 0x00FE_8750;

    #[inline(always)]
    fn mul32(left: f32, right: f32) -> f32 {
        core::hint::black_box(left) * core::hint::black_box(right)
    }
    #[inline(always)]
    fn div32(numerator: f32, denominator: f32) -> f32 {
        core::hint::black_box(numerator) / core::hint::black_box(denominator)
    }

    unsafe {
        let denominator = (this.wrapping_add(FRAME_DENOMINATOR) as *const f32).read_unaligned();
        let zero = lf_checker_rt::global::<f32>(ZERO_VA).read_unaligned();
        if !(denominator > zero) {
            return 0.0;
        }
        let scale = lf_checker_rt::global::<f32>(SCALE_VA).read_unaligned();
        div32(mul32(frame_time, scale), denominator)
    }
});
