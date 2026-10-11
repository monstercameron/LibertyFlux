// original: 0x009d1030 set_object_word_2bc

/// Optionally begin a profiler scope, store the supplied word at object offset 0x2BC, then either tail-call the profiler end callback or return the supplied word. The profiled path overwrites the incoming argument slot with the callback token.
lf_checker_rt::export!(thiscall, rw_009d1030(this: u32, value: u32) -> eax {
    const PROFILE_BEGIN_ID: u32 = 2;
    const PROFILE_END_ID: u32 = 3;
    const PROFILE_TOKEN_VA: u32 = 0x0129588C;
    const PROFILER_ENABLED_VA: u32 = 0x0103AD58;
    const FIELD_OFFSET: u32 = 0x2BC;
    unsafe {
        if lf_checker_rt::global::<u8>(PROFILER_ENABLED_VA).read() != 0 {
            let _ = lf_checker_rt::callee_stdcall!(PROFILE_BEGIN_ID, u32, lf_checker_rt::relocated(PROFILE_TOKEN_VA));
        }
        (this.wrapping_add(FIELD_OFFSET) as *mut u32).write_unaligned(value);
        if lf_checker_rt::global::<u8>(PROFILER_ENABLED_VA).read() != 0 {
            lf_checker_rt::callee_stdcall!(PROFILE_END_ID, u32, lf_checker_rt::relocated(PROFILE_TOKEN_VA))
        } else {
            value
        }
    }
});
