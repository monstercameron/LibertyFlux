// original: 0x005d6b80 m:/html

/// Initialise the HTML subsystem and report its render mode.
///
/// On first call (low bit of the state word clear) publishes the subsystem
/// descriptor, clears the work counter and runs the one-time setup callee.
/// Then runs the attach callee on the registry with tag `1`, reads the
/// quality byte, runs the mode-select callee, and stores the render mode:
/// `5` when the callee answered non-zero, otherwise the configured default.
/// Returns the mode-select callee's answer.
///
/// Original: 0x005d6b80 (cdecl, no arguments).
lf_checker_rt::export!(cdecl, rw_005d6b80() -> u32 {
    unsafe {
        const STATE: u32 = 0x01C99E70;
        const REGISTRY: u32 = 0x01C99E78;
        const WORK: u32 = 0x01C9A07C;
        const MODE: u32 = 0x01C9A080;
        const QUALITY: u32 = 0x01C9A084;
        const DESCR_PTR: u32 = 0x00E6F050;
        const DESCR_VALUE: u32 = 0x00E7F8AC;
        const ATTACH_TAG: u32 = 0x00F907E8;
        const SELECT_TAG: u32 = 0x00F907D8;
        const ATTACH_ARG: u32 = 1;
        const FORCED_MODE: u32 = 5;
        const SETUP_CALLEE: u32 = 1;
        const ATTACH_CALLEE: u32 = 2;
        const SELECT_CALLEE: u32 = 3;

        #[inline(always)]
        unsafe fn g32(va: u32) -> u32 {
            unsafe { (lf_checker_rt::relocated(va) as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wg32(va: u32, v: u32) {
            unsafe { (lf_checker_rt::relocated(va) as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn g8(va: u32) -> u32 {
            unsafe { (lf_checker_rt::relocated(va) as *const u8).read() as u32 }
        }

        let state = g32(STATE);
        if state & 1 == 0 {
            wg32(STATE, state | 1);
            wg32(REGISTRY, lf_checker_rt::relocated(DESCR_VALUE));
            wg32(WORK, 0);
            lf_checker_rt::callee_cdecl!(
                SETUP_CALLEE,
                u32,
                lf_checker_rt::relocated(DESCR_PTR)
            );
        }
        lf_checker_rt::callee_thiscall!(
            ATTACH_CALLEE,
            u32,
            lf_checker_rt::relocated(REGISTRY),
            lf_checker_rt::relocated(ATTACH_TAG),
            ATTACH_ARG
        );
        let quality = g8(QUALITY);
        let answer: u32 = lf_checker_rt::callee_cdecl!(
            SELECT_CALLEE,
            u32,
            lf_checker_rt::relocated(SELECT_TAG),
            lf_checker_rt::relocated(REGISTRY),
            quality
        );
        let mut mode = g32(MODE);
        if answer & 0xff != 0 {
            mode = FORCED_MODE;
        }
        wg32(MODE, mode);
        answer
    }
});
