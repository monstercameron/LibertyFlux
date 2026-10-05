// original: 0x008CB230 STRTNEWGAME (symbols)

/// Starts a new game session for the streaming subsystem: resets the four
/// channel contexts through the reset callee (callee 0, thiscall without
/// stack arguments), clears the slot globals, refreshes the device handle
/// through the refresh callee (callee 1) into `HANDLE`, probes the session
/// through the probe callee (callee 2, thiscall with the handle address) and
/// records the outcome, zeroes the tail globals, and, when the global
/// auxiliary byte is set, resolves the auxiliary name through the lookup
/// callee (callee 3, thiscall with one stack argument) and hands it to the
/// attach callee (callee 4, cdecl with the buffer address, the lookup result
/// and 0x40).
///
/// No arguments (cdecl); no return value.
lf_checker_rt::export!(cdecl, rw_008CB230() -> u32 {
    unsafe {
        /// Reset callee id.
        const RESET: u32 = 0;
        /// Refresh callee id.
        const REFRESH: u32 = 1;
        /// Probe callee id.
        const PROBE: u32 = 2;
        /// Lookup callee id.
        const LOOKUP: u32 = 3;
        /// Attach callee id.
        const ATTACH: u32 = 4;
        /// Channel contexts reset in order.
        const CTX0: u32 = 0x117330C;
        const CTX1: u32 = 0x1173324;
        const CTX2: u32 = 0x117333C;
        const CTX3: u32 = 0x11732F4;
        /// Global device handle word.
        const HANDLE: u32 = 0x1173240;
        /// Probe callee's object.
        const PROBE_THIS: u32 = 0x11D6FE4;
        /// Session flag byte and its mirror word.
        const SESSION_FLAG: u32 = 0x1173117;
        const SESSION_MIRROR: u32 = 0x1031F28;
        /// Auxiliary switch byte.
        const AUX_SWITCH: u32 = 0x116C252;
        /// Auxiliary name and size passed to lookup.
        const AUX_NAME: u32 = 0xE805F0;
        const AUX_ARG: u32 = 0x40;
        /// Lookup callee's object.
        const LOOKUP_THIS: u32 = 0x116BFF0;
        /// Buffer passed to the attach callee.
        const ATTACH_BUF: u32 = 0x1173268;
        /// Global holding the device object pointer.
        const DEVICE: u32 = 0x1BB5624;
        let _r0: u32 = lf_checker_rt::callee_thiscall!(RESET, u32, lf_checker_rt::relocated(CTX0));
        let device = (lf_checker_rt::relocated(DEVICE) as *const u32).read();
        (lf_checker_rt::relocated(0x1031BB8) as *mut u32).write(0xFFFF_FFFF);
        (lf_checker_rt::relocated(0x1031BB4) as *mut u32).write(0xFFFF_FFFF);
        let handle: u32 = lf_checker_rt::callee_thiscall!(REFRESH, u32, device);
        (lf_checker_rt::relocated(HANDLE) as *mut u32).write(handle);
        let _r1: u32 = lf_checker_rt::callee_thiscall!(RESET, u32, lf_checker_rt::relocated(CTX1));
        (lf_checker_rt::relocated(0x1031BBC) as *mut u32).write(0xFFFF_FFFF);
        let _r2: u32 = lf_checker_rt::callee_thiscall!(RESET, u32, lf_checker_rt::relocated(CTX2));
        (lf_checker_rt::relocated(0x1031BC0) as *mut u32).write(0xFFFF_FFFF);
        let _r3: u32 = lf_checker_rt::callee_thiscall!(RESET, u32, lf_checker_rt::relocated(CTX3));
        (lf_checker_rt::relocated(0x1031BC4) as *mut u32).write(0xFFFF_FFFF);
        (lf_checker_rt::relocated(SESSION_FLAG) as *mut u8).write(0);
        (lf_checker_rt::relocated(SESSION_MIRROR) as *mut u32).write(0xFFFF_FFFF);
        let ok: u32 = lf_checker_rt::callee_thiscall!(
            PROBE,
            u32,
            lf_checker_rt::relocated(PROBE_THIS),
            lf_checker_rt::relocated(HANDLE)
        );
        if ok & 0xFF != 0 {
            let h = (lf_checker_rt::relocated(HANDLE) as *const u32).read();
            (lf_checker_rt::relocated(SESSION_FLAG) as *mut u8).write(1);
            (lf_checker_rt::relocated(SESSION_MIRROR) as *mut u32).write(h);
        }
        (lf_checker_rt::relocated(0x1173248) as *mut u32).write(0);
        (lf_checker_rt::relocated(0x117324C) as *mut u32).write(0);
        if (lf_checker_rt::relocated(AUX_SWITCH) as *const u8).read() == 0 {
            return 0;
        }
        let found: u32 = lf_checker_rt::callee_thiscall!(
            LOOKUP,
            u32,
            lf_checker_rt::relocated(LOOKUP_THIS),
            lf_checker_rt::relocated(AUX_NAME)
        );
        let _a: u32 = lf_checker_rt::callee_cdecl!(
            ATTACH,
            u32,
            lf_checker_rt::relocated(ATTACH_BUF),
            found,
            AUX_ARG
        );
        0
    }
});
