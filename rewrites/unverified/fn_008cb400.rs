// original: 0x008CB400 stream_active_flag (proposed)

/// Activity flag: 0 only when the global mode word is 0 and the global
/// override byte is 0 at the same time; 1 otherwise.
///
/// No arguments (cdecl); the byte result is returned in AL.
lf_checker_rt::export!(cdecl, rw_008CB400() -> u32 {
    unsafe {
        /// Global streaming mode word.
        const MODE: u32 = 0x11730F8;
        /// Global override byte.
        const OVERRIDE: u32 = 0x1172CD0;
        let mode = (lf_checker_rt::relocated(MODE) as *const u32).read();
        if mode != 0 {
            return 1;
        }
        let ov = (lf_checker_rt::relocated(OVERRIDE) as *const u8).read();
        u32::from(ov != 0)
    }
});
