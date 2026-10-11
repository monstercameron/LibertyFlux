// original: 0x009d0c20 get_object_byte_2c2

/// Optionally bracket the read with profiler callbacks, then return the byte at object offset 0x2C2.
lf_checker_rt::export!(thiscall, rw_009d0c20(this: u32) -> al {
    const PROFILE_BEGIN_ID: u32 = 2;
    const PROFILE_END_ID: u32 = 3;
    const PROFILE_TOKEN_VA: u32 = 0x0129588C;
    const PROFILER_ENABLED_VA: u32 = 0x0103AD58;
    const FIELD_OFFSET: u32 = 0x2C2;
    unsafe {
        if lf_checker_rt::global::<u8>(PROFILER_ENABLED_VA).read() != 0 {
            let _ = lf_checker_rt::callee_stdcall!(PROFILE_BEGIN_ID, u32, lf_checker_rt::relocated(PROFILE_TOKEN_VA));
        }
        let field_pointer = this.wrapping_add(FIELD_OFFSET) as *const u8;
        let result = field_pointer.read() as u32;
        if lf_checker_rt::global::<u8>(PROFILER_ENABLED_VA).read() != 0 {
            let _ = lf_checker_rt::callee_stdcall!(PROFILE_END_ID, u32, lf_checker_rt::relocated(PROFILE_TOKEN_VA));
        }
        result
    }
});
