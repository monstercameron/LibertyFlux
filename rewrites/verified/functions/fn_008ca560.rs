// original: 0x008CA560 stream_shutdown (proposed)

/// Shuts the streaming subsystem down: clears the context at `CTX_A` through
/// the clear callee (callee 0), marks the globals stopped, calls the detach
/// callee (callee 1), calls the release callee (callee 2, thiscall with
/// argument 0) on `CTX_B`, then frees the pending buffer (when non-null)
/// through the free callee (callee 3, cdecl) and zeroes the tail globals.
/// Returns 0.
///
/// No arguments (cdecl); the result is returned in EAX.
lf_checker_rt::export!(cdecl, rw_008CA560() -> u32 {
    unsafe {
        /// First context cleared.
        const CTX_A: u32 = 0x1172FE0;
        /// Context released.
        const CTX_B: u32 = 0x11732E8;
        /// Detach callee's object.
        const DETACH_THIS: u32 = 0x1173358;
        /// Clear callee id.
        const CLEAR: u32 = 0;
        /// Detach callee id.
        const DETACH: u32 = 1;
        /// Release callee id.
        const RELEASE: u32 = 2;
        /// Free callee id.
        const FREE: u32 = 3;
        let _c: u32 = lf_checker_rt::callee_thiscall!(CLEAR, u32, lf_checker_rt::relocated(CTX_A));
        (lf_checker_rt::relocated(0x1032110) as *mut u32).write(0xFFFF_FFFF);
        (lf_checker_rt::relocated(0x1031F2E) as *mut u8).write(1);
        (lf_checker_rt::relocated(0x1172CD4) as *mut u32).write(0xFFFF_FFFF);
        let _d: u32 =
            lf_checker_rt::callee_thiscall!(DETACH, u32, lf_checker_rt::relocated(DETACH_THIS));
        (lf_checker_rt::relocated(0x1172CD1) as *mut u8).write(0);
        (lf_checker_rt::relocated(0x1172CD2) as *mut u8).write(0);
        (lf_checker_rt::relocated(0x1172CD3) as *mut u8).write(0);
        let _r: u32 =
            lf_checker_rt::callee_thiscall!(RELEASE, u32, lf_checker_rt::relocated(CTX_B), 0);
        let pending = (lf_checker_rt::relocated(0x1173254) as *const u32).read();
        (lf_checker_rt::relocated(0x1031F2C) as *mut u8).write(1);
        (lf_checker_rt::relocated(0x1173245) as *mut u8).write(0);
        (lf_checker_rt::relocated(0x1031F2D) as *mut u8).write(1);
        (lf_checker_rt::relocated(0x1173250) as *mut u32).write(0);
        if pending != 0 {
            let _f: u32 = lf_checker_rt::callee_cdecl!(FREE, u32, pending);
        }
        (lf_checker_rt::relocated(0x1173254) as *mut u32).write(0);
        (lf_checker_rt::relocated(0x1173268) as *mut u16).write(0);
        0
    }
});
