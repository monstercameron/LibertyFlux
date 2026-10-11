// original: 0x009d0950 get_timing_storage_address

/// Obtain the timing singleton, optionally bracket the operation with profiler callbacks, and return its interior address at offset 0x1B7C.
lf_checker_rt::export!(cdecl, rw_009d0950() -> u32 {
    const INIT_CALLEE_ID: u32 = 1;
    const PROFILE_BEGIN_ID: u32 = 2;
    const PROFILE_END_ID: u32 = 3;
    const PROFILE_TOKEN_VA: u32 = 0x0129588C;
    const PROFILER_ENABLED_VA: u32 = 0x0103AD58;
    const STORAGE_OFFSET: u32 = 0x1B7C;
    unsafe {
        let singleton = lf_checker_rt::callee_cdecl!(INIT_CALLEE_ID, u32,);
        if lf_checker_rt::global::<u8>(PROFILER_ENABLED_VA).read() != 0 {
            let _ = lf_checker_rt::callee_stdcall!(PROFILE_BEGIN_ID, u32, lf_checker_rt::relocated(PROFILE_TOKEN_VA));
        }
        let address = singleton.wrapping_add(STORAGE_OFFSET);
        if lf_checker_rt::global::<u8>(PROFILER_ENABLED_VA).read() != 0 {
            let _ = lf_checker_rt::callee_stdcall!(PROFILE_END_ID, u32, lf_checker_rt::relocated(PROFILE_TOKEN_VA));
        }
        address
    }
});
