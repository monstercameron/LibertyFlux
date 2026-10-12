// original: 0x00E63A80 CLOUD
/// Runs the listed initialization calls in order and stores their scripted results where the original does. The checker compares every call and global result; stack comparison is narrowed because the original accumulates several outgoing call frames before cleanup.
lf_checker_rt::export!(cdecl, rw_00E63A80() -> u32 {
    let call_result_0 = lf_checker_rt::callee_cdecl!(1u32, u32, lf_checker_rt::relocated(0x00E88E0C), 0u32);
    unsafe { lf_checker_rt::global::<u32>(0x011D76D4).write(call_result_0); }
    let call_result_1 = lf_checker_rt::callee_cdecl!(1u32, u32, lf_checker_rt::relocated(0x00E88E10), 0u32);
    unsafe { lf_checker_rt::global::<u32>(0x011D76D8).write(call_result_1); }
    let call_result_2 = lf_checker_rt::callee_cdecl!(1u32, u32, lf_checker_rt::relocated(0x00E88E18), 0u32);
    unsafe { lf_checker_rt::global::<u32>(0x011D76DC).write(call_result_2); }
    let call_result_3 = lf_checker_rt::callee_cdecl!(1u32, u32, lf_checker_rt::relocated(0x00E88908), 0u32);
    unsafe { lf_checker_rt::global::<u32>(0x011D76E0).write(call_result_3); }
    let call_result_4 = lf_checker_rt::callee_cdecl!(1u32, u32, lf_checker_rt::relocated(0x00E8890C), 0u32);
    unsafe { lf_checker_rt::global::<u32>(0x011D76E4).write(call_result_4); }
    call_result_4
});
