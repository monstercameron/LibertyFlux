// original: 0x008CB320 stream_device_check (proposed)

/// Checks the streaming device: calls the check callee (callee 0, thiscall
/// with one stack argument, the global handle word) and, when its low byte
/// reports success, sets the flag bytes at `+FLAG0` and `+FLAG1` and returns
/// 1. Otherwise calls the fallback callee (callee 1, cdecl with argument 1)
/// and returns 0.
///
/// No arguments (cdecl); the byte result is returned in AL.
lf_checker_rt::export!(cdecl, rw_008CB320() -> u32 {
    unsafe {
        /// Global streaming handle word passed to the check.
        const HANDLE: u32 = 0x1172CD4;
        /// Object pointer the check callee expects in ECX.
        const CHECK_THIS: u32 = 0x18B7A4D;
        /// First flag byte set on success.
        const FLAG0: u32 = 0x1172CD1;
        /// Second flag byte set on success.
        const FLAG1: u32 = 0x1172CD3;
        /// Check callee id.
        const CHECK: u32 = 0;
        /// Fallback callee id.
        const FALLBACK: u32 = 1;
        let handle = (lf_checker_rt::relocated(HANDLE) as *const u32).read();
        let ok: u32 =
            lf_checker_rt::callee_thiscall!(CHECK, u32, lf_checker_rt::relocated(CHECK_THIS), handle);
        if ok & 0xFF == 0 {
            let _fb: u32 = lf_checker_rt::callee_cdecl!(FALLBACK, u32, 1);
            return 0;
        }
        (lf_checker_rt::relocated(FLAG0) as *mut u8).write(1);
        (lf_checker_rt::relocated(FLAG1) as *mut u8).write(1);
        1
    }
});
