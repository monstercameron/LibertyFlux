// original: 0x00E63C50 allocate_two_global_resources_and_register_cleanup
/// Runs the listed initialization calls in order and stores their scripted results where the original does. The checker compares every call and global result; stack comparison is narrowed because the original accumulates several outgoing call frames before cleanup.
lf_checker_rt::export!(cdecl, rw_00E63C50() -> u32 {
    let call_result_0 = lf_checker_rt::callee_cdecl!(1u32, u32, 0u32);
    unsafe { lf_checker_rt::global::<u32>(0x01037820).write(call_result_0); }
    let call_result_1 = lf_checker_rt::callee_cdecl!(1u32, u32, 0u32);
    unsafe { lf_checker_rt::global::<u32>(0x01037824).write(call_result_1); }
    let call_result_2 = lf_checker_rt::callee_cdecl!(2u32, u32, lf_checker_rt::relocated(0x00E71600));
    call_result_2
});
