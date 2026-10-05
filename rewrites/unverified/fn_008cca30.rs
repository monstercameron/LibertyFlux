// original: 0x008CCA30 stream_mode_tail (proposed)

/// Mode-gated tail dispatch: returns at once when the global mode word is
/// not 2. Otherwise calls the probe callee (callee 0); when it reports 0
/// the finish routine is tail-called, else the prepare callee (callee 1) is
/// called first and then the finish routine is tail-called (callee 2, a
/// patched tail jump; the rewrite performs it as its final call and returns
/// its result).
///
/// No arguments (cdecl); the tail result is returned in EAX.
lf_checker_rt::export!(cdecl, rw_008CCA30() -> u32 {
    unsafe {
        /// Global streaming mode word.
        const MODE: u32 = 0x11730F8;
        /// Mode value that enables the dispatch.
        const ENABLED: u32 = 2;
        /// Probe callee id.
        const PROBE: u32 = 0;
        /// Prepare callee id.
        const PREPARE: u32 = 1;
        /// Tail-called finish routine id.
        const FINISH: u32 = 2;
        if (lf_checker_rt::relocated(MODE) as *const u32).read() != ENABLED {
            return 0;
        }
        let probe: u32 = lf_checker_rt::callee_cdecl!(PROBE, u32);
        if probe != 0 {
            let _p: u32 = lf_checker_rt::callee_cdecl!(PREPARE, u32);
        }
        lf_checker_rt::callee_cdecl!(FINISH, u32)
    }
});
