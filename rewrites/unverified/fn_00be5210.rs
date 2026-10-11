// original: 0x00BE5210 task_control_check_result

/// Return eight when the context skip bit is set or the native context check
/// succeeds. Otherwise return seven. The context pointer is a cdecl stack
/// argument; the delegated native method is intercepted by the contract.
lf_checker_rt::export!(cdecl, rw_00be5210(context: u32) -> u32 {
    unsafe {
        const SKIP_NATIVE_CHECK: u32 = 0x26c;
        let flag_byte = ((context + SKIP_NATIVE_CHECK) as *const u8).read();
        if flag_byte & 4 != 0 {
            8
        } else {
            let native_result = lf_checker_rt::callee_thiscall!(1, u8, context);
            if native_result == 0 { 7 } else { 8 }
        }
    }
});
