// original: 0x00E645C0 PED_ON_PED_BUMP
/// Calls the configured helper with the original arguments and stores its return value in the named global. The helper is intercepted by the checker.
lf_checker_rt::export!(cdecl, rw_00E645C0() -> u32 {
    let call_result_0 = lf_checker_rt::callee_cdecl!(1u32, u32, lf_checker_rt::relocated(0x00E8CE28), 0u32);
    unsafe { lf_checker_rt::global::<u32>(0x01231574).write(call_result_0); }
    call_result_0
});
