// original: 0x008ADA70 audio_curve_stage_filtered
/// Selects one of two phase values using the object's threshold, combines
/// it with the stack input and object offsets, calls the sine helper, then
/// applies the object's multiplier and additive offset.
lf_checker_rt::export!(thiscall, rw_008ada70(this_: u32, arg_bits: u32) -> f32 {
    unsafe {
        #[inline(always)]
        fn fsub(left: f32, right: f32) -> f32 {
            core::hint::black_box(left) - core::hint::black_box(right)
        }
        #[inline(always)]
        fn fadd(left: f32, right: f32) -> f32 {
            core::hint::black_box(left) + core::hint::black_box(right)
        }
        #[inline(always)]
        fn fmul(left: f32, right: f32) -> f32 {
            core::hint::black_box(left) * core::hint::black_box(right)
        }

        const THRESHOLD_VA: u32 = 0x00FE8628;
        const SCALE_VA: u32 = 0x00FE8728;
        let threshold = f32::from_bits(core::ptr::read_unaligned(
            lf_checker_rt::global::<u32>(THRESHOLD_VA),
        ));
        let scale = f32::from_bits(core::ptr::read_unaligned(
            lf_checker_rt::global::<u32>(SCALE_VA),
        ));
        let multiplier = f32::from_bits(core::ptr::read_unaligned((this_ + 4) as *const u32));
        let addend = f32::from_bits(core::ptr::read_unaligned((this_ + 8) as *const u32));
        let value = f32::from_bits(core::ptr::read_unaligned((this_ + 12) as *const u32));
        let final_addend = f32::from_bits(core::ptr::read_unaligned((this_ + 16) as *const u32));
        let alternate = f32::from_bits(core::ptr::read_unaligned((this_ + 20) as *const u32));
        let argument = f32::from_bits(arg_bits);

        let selected = if value.is_nan() || threshold.is_nan() || value < threshold {
            alternate
        } else {
            fsub(value, addend)
        };
        let weighted = fmul(selected, argument);
        let shifted = fadd(weighted, addend);
        let helper_input = fmul(shifted, scale);
        let answer_bits = lf_checker_rt::callee_cdecl!(1, u32, helper_input.to_bits());
        let answer = f32::from_bits(answer_bits);
        let scaled_answer = fmul(answer, multiplier);
        fadd(scaled_answer, final_addend)
    }
});
