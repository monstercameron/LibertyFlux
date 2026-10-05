// original: 0x008C95C0 stream_mode_value (proposed)

/// Streaming mode gate: returns the shared streaming value word while the
/// global mode word is 1 or 2, otherwise 0.
///
/// Reads two global dwords and takes no arguments (cdecl). Edge cases: any
/// mode other than 1 or 2 (including 0 and large values) yields 0.
lf_checker_rt::export!(cdecl, rw_008C95C0() -> u32 {
    unsafe {
        /// Global streaming mode selector.
        const MODE: u32 = 0x11730F8;
        /// Value exposed while the mode is 1 or 2.
        const VALUE: u32 = 0x1172FE4;
        let mode = (lf_checker_rt::relocated(MODE) as *const u32).read();
        if mode == 1 || mode == 2 {
            (lf_checker_rt::relocated(VALUE) as *const u32).read()
        } else {
            0
        }
    }
});
