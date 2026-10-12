// original: 0x008AD310 audio_clamp_call_select
/// Clamp a phase value, call the sine helper, and optionally return its
/// complement. The selector at `this+4` chooses the input or `one-input`;
/// the value is clamped to `[0, one]`, scaled by the image factor, and sent
/// to the helper. The selector at `this+8` returns either the helper answer
/// or `one-answer`. The helper uses XMM0 for its f32 argument and result.
lf_checker_rt::export!(thiscall, rw_008ad310(this_: u32, arg_bits: u32) -> f32 {
    unsafe {
        #[inline(always)]
        fn fsub(left: f32, right: f32) -> f32 {
            core::hint::black_box(left) - core::hint::black_box(right)
        }
        #[inline(always)]
        fn fmul(left: f32, right: f32) -> f32 {
            core::hint::black_box(left) * core::hint::black_box(right)
        }

        const ONE_VA: u32 = 0x00FE88E8;
        const SCALE_VA: u32 = 0x00FE8978;
        const ZERO_VA: u32 = 0x00FE8628;
        let one = f32::from_bits(core::ptr::read_unaligned(lf_checker_rt::global::<u32>(ONE_VA)));
        let scale = f32::from_bits(core::ptr::read_unaligned(lf_checker_rt::global::<u32>(SCALE_VA)));
        let zero = f32::from_bits(core::ptr::read_unaligned(lf_checker_rt::global::<u32>(ZERO_VA)));
        let selector = f32::from_bits(core::ptr::read_unaligned((this_ + 4) as *const u32));
        let return_gate = f32::from_bits(core::ptr::read_unaligned((this_ + 8) as *const u32));
        let argument = f32::from_bits(arg_bits);

        let literal_zero = 0.0f32;
        let mut value = if !selector.is_nan() && selector >= literal_zero {
            argument
        } else {
            fsub(one, argument)
        };
        if value < literal_zero {
            value = literal_zero;
        }
        if value > one {
            value = one;
        }

        let scaled = fmul(value, scale);
        let sine = lf_checker_rt::callee_cdecl!(1, f32, scaled.to_bits());
        if return_gate.is_nan() || zero.is_nan() || return_gate < zero {
            fsub(one, sine)
        } else {
            sine
        }
    }
});
