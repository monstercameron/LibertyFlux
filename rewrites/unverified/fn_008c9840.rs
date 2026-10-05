// original: 0x008C9840 stream_probe_acquire (proposed)

/// Probes the streaming device and stores its handle: writes -1 to the
/// global `HANDLE`, calls the probe callee, and returns 0 when the probe
/// reports 0. Otherwise probes once more, calls the acquire callee, stores
/// its result in `HANDLE` and returns 1.
///
/// No arguments (cdecl); the byte result is returned in AL. Callee 0 is the
/// probe (no arguments, called up to twice), callee 1 is the acquire step
/// (no stack arguments).
lf_checker_rt::export!(cdecl, rw_008C9840() -> u32 {
    unsafe {
        /// Global streaming handle word.
        const HANDLE: u32 = 0x1172CD4;
        /// Probe callee id.
        const PROBE: u32 = 0;
        /// Acquire callee id.
        const ACQUIRE: u32 = 1;
        (lf_checker_rt::relocated(HANDLE) as *mut u32).write(0xFFFF_FFFF);
        let probe: u32 = lf_checker_rt::callee_cdecl!(PROBE, u32);
        if probe == 0 {
            return 0;
        }
        let _again: u32 = lf_checker_rt::callee_cdecl!(PROBE, u32);
        let handle: u32 = lf_checker_rt::callee_cdecl!(ACQUIRE, u32);
        (lf_checker_rt::relocated(HANDLE) as *mut u32).write(handle);
        1
    }
});
