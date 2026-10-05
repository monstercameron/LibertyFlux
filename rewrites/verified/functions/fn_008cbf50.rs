// original: 0x008CBF50 stream_session_reset (proposed)

/// Resets the streaming session: resets the four channel contexts through
/// the reset callee (callee 0, thiscall without stack arguments), clears the
/// slot globals, refreshes the device handle through the refresh callee
/// (callee 1) into `HANDLE`, re-reads the device object through the status
/// callee (callee 2), zeroes the tail globals, and, when the device's flag
/// byte at `+DEV_FLAG` is set, keeps `HANDLE` unless the global override is
/// not -1, in which case the override wins.
///
/// No arguments (cdecl); the status result is returned in EAX.
lf_checker_rt::export!(cdecl, rw_008CBF50() -> u32 {
    unsafe {
        /// Reset callee id.
        const RESET: u32 = 0;
        /// Refresh callee id.
        const REFRESH: u32 = 1;
        /// Status callee id.
        const STATUS: u32 = 2;
        /// Channel contexts reset in order.
        const CTX0: u32 = 0x117330C;
        const CTX1: u32 = 0x1173324;
        const CTX2: u32 = 0x117333C;
        const CTX3: u32 = 0x11732F4;
        /// Global holding the device object pointer.
        const DEVICE: u32 = 0x1BB5624;
        /// Global device handle word.
        const HANDLE: u32 = 0x1173240;
        /// Global override word; -1 keeps the handle.
        const OVERRIDE: u32 = 0x1031BB4;
        /// Offset of the flag byte in the device object.
        const DEV_FLAG: u32 = 0x169;
        /// Override value meaning keep.
        const KEEP: u32 = 0xFFFF_FFFF;
        let _r0: u32 = lf_checker_rt::callee_thiscall!(RESET, u32, lf_checker_rt::relocated(CTX0));
        (lf_checker_rt::relocated(0x1031BB8) as *mut u32).write(KEEP);
        let _r1: u32 = lf_checker_rt::callee_thiscall!(RESET, u32, lf_checker_rt::relocated(CTX1));
        (lf_checker_rt::relocated(0x1031BBC) as *mut u32).write(KEEP);
        let _r2: u32 = lf_checker_rt::callee_thiscall!(RESET, u32, lf_checker_rt::relocated(CTX2));
        (lf_checker_rt::relocated(0x1031BC0) as *mut u32).write(KEEP);
        let _r3: u32 = lf_checker_rt::callee_thiscall!(RESET, u32, lf_checker_rt::relocated(CTX3));
        let device = (lf_checker_rt::relocated(DEVICE) as *const u32).read();
        (lf_checker_rt::relocated(0x1031BC4) as *mut u32).write(KEEP);
        (lf_checker_rt::relocated(0x1172D8C) as *mut u32).write(0);
        let handle: u32 = lf_checker_rt::callee_thiscall!(REFRESH, u32, device);
        let device2 = (lf_checker_rt::relocated(DEVICE) as *const u32).read();
        (lf_checker_rt::relocated(HANDLE) as *mut u32).write(handle);
        let status: u32 = lf_checker_rt::callee_thiscall!(STATUS, u32, device2);
        let flag = ((device2 + DEV_FLAG) as *const u8).read();
        (lf_checker_rt::relocated(0x1173248) as *mut u32).write(0);
        (lf_checker_rt::relocated(0x117324C) as *mut u32).write(0);
        if flag != 0 {
            let ov = (lf_checker_rt::relocated(OVERRIDE) as *const u32).read();
            if ov != KEEP {
                (lf_checker_rt::relocated(HANDLE) as *mut u32).write(ov);
            }
        }
        status
    }
});
