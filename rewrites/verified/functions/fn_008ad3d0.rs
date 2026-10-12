// original: 0x008AD3D0 audio_shape_call_blend
/// Selects an input or its complement, calls a two-float helper, then returns
/// either the helper result or its complement according to the return gate.
/// Both helper arguments use XMM registers; the helper result is returned in XMM0.
lf_checker_rt::export!(thiscall, rw_008ad3d0(this_: u32, arg_bits: u32) -> f32 {
    unsafe {
        #[inline(always)]
        fn fsub(left: f32, right: f32) -> f32 {
            core::hint::black_box(left) - core::hint::black_box(right)
        }

        const ONE_VA: u32 = 0x00FE88E8;
        let one = f32::from_bits(core::ptr::read_unaligned(
            lf_checker_rt::global::<u32>(ONE_VA),
        ));
        let selector = f32::from_bits(core::ptr::read_unaligned((this_ + 4) as *const u32));
        let return_gate = f32::from_bits(core::ptr::read_unaligned((this_ + 8) as *const u32));
        let helper_factor = f32::from_bits(core::ptr::read_unaligned((this_ + 12) as *const u32));
        let argument = f32::from_bits(arg_bits);

        let helper_input = if !selector.is_nan() && selector >= 0.0 {
            argument
        } else {
            fsub(one, argument)
        };
        let answer_bits = lf_checker_rt::callee_cdecl!(
            1,
            u32,
            helper_input.to_bits(),
            helper_factor.to_bits()
        );
        let answer = f32::from_bits(answer_bits);

        if return_gate.is_nan() || return_gate < 0.0 {
            fsub(one, answer)
        } else {
            answer
        }
    }
});
