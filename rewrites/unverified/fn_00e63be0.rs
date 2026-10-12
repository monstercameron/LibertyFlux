// original: 0x00E63BE0 initialize_audio_entity_global
/// Runs the configured initialization or registration calls in order, forwarding the original relocated object and callback pointers, and returns the final helper result.
lf_checker_rt::export!(cdecl, rw_00E63BE0() -> u32 {
    let call_result_0 = lf_checker_rt::callee_thiscall!(1u32, u32, 18956984u32);
    let initializer_value = lf_checker_rt::relocated(0x00E83134);
    unsafe { lf_checker_rt::global::<u32>(0x012142B8).write(initializer_value); }
    let call_result_2 = lf_checker_rt::callee_cdecl!(2u32, u32, lf_checker_rt::relocated(0x00E715B0));
    call_result_2
});
