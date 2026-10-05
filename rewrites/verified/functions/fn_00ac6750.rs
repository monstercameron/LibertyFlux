// original: 0x00AC6750 stream_clear_mask_and_notify (proposed)

/// Clear mask bits from the low byte of `arg`, then notify with the cookie.
///
/// The original ANDs the mask word with the bitwise NOT of the argument's
/// low byte (upper bytes ignored) and calls the notify callee with the
/// cookie word (cdecl, one word). No value is returned.
lf_checker_rt::export!(cdecl, rw_00AC6750(arg: u32) -> u32 {
    unsafe {
        const COOKIE: u32 = 0x0154E044;
        const MASK: u32 = 0x0154E048;
        const NOTIFY: u32 = 1;
        let cookie = (lf_checker_rt::relocated(COOKIE) as *const u32).read_unaligned();
        let m = lf_checker_rt::relocated(MASK) as *mut u32;
        *m = *m & !(arg & 0xff);
        lf_checker_rt::callee_cdecl!(NOTIFY, u32, cookie);
        0
    }
});
