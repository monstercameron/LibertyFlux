// original: 0x009d07e0 get_timing_word_004

/// Obtain the timing singleton, optionally bracket the read with profiler callbacks, then return its word at offset 0x04.
lf_checker_rt::export!(cdecl, rw_009d07e0() -> eax {
    const INIT_CALLEE_ID: u32 = 1;
    const PROFILE_BEGIN_ID: u32 = 2;
    const PROFILE_END_ID: u32 = 3;
    const SINGLETON_VA: u32 = 0x0129588C;
    const PROFILER_ENABLED_VA: u32 = 0x0103AD58;
    const FIELD_OFFSET: u32 = 0x4;
    unsafe {
        let singleton = lf_checker_rt::callee_cdecl!(INIT_CALLEE_ID, u32);
        if lf_checker_rt::global::<u8>(PROFILER_ENABLED_VA).read() != 0 {
            let _ = lf_checker_rt::callee_stdcall!(PROFILE_BEGIN_ID, u32, lf_checker_rt::relocated(SINGLETON_VA));
        }
        let value = lf_checker_rt::global::<u32>(singleton.wrapping_add(FIELD_OFFSET)).read_unaligned();
        if lf_checker_rt::global::<u8>(PROFILER_ENABLED_VA).read() != 0 {
            let _ = lf_checker_rt::callee_stdcall!(PROFILE_END_ID, u32, lf_checker_rt::relocated(SINGLETON_VA));
        }
        value
    }
});
