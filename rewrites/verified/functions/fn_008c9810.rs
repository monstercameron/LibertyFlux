// original: 0x008C9810 stream_ready_flag (proposed)

/// Readiness flag: 1 when the global state word equals 2 and the global
/// enable byte is non-zero, otherwise 0.
///
/// No arguments (cdecl); the byte result is returned in AL.
lf_checker_rt::export!(cdecl, rw_008C9810() -> u32 {
    unsafe {
        /// Global state word; must equal `READY_STATE`.
        const STATE: u32 = 0x11D6FD0;
        /// State value meaning ready.
        const READY_STATE: u32 = 2;
        /// Global enable byte; must be non-zero.
        const ENABLE: u32 = 0x1173246;
        let state = (lf_checker_rt::relocated(STATE) as *const u32).read();
        let enable = (lf_checker_rt::relocated(ENABLE) as *const u8).read();
        u32::from(state == READY_STATE && enable != 0)
    }
});
