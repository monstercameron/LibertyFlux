// original: 0x00b91a70 NativeImpl_SET_MISSION_PASSED_CASH

/// Formats a mission-passed message and publishes it to global state.
///
/// Picks one of four format strings from the flag byte and the mode global
/// `MODE_GLOB` (modes 1 and 4 select the alternate pair), formats `a1` into
/// a 52-byte frame buffer through `FMT_CALLEE`, copies the result including
/// its NUL into the global text slot at `OUT_BUF`, stores `a2` into
/// `OUT_VAL` and raises `DONE_FLAG`. Ends with the security-cookie check.
///
/// The buffer pointer and the cookie-check argument are uncompared frame
/// artifacts (see `narrowed`); the formatted bytes are verified through the
/// global copy, and the format selection through the call log.
///
/// Original: 0x00B91A70 (cdecl, three stack words, no return value).
lf_checker_rt::export!(cdecl, rw_00b91a70(flag: u32, a1: u32, a2: u32) -> u32 {
    unsafe {
        const FMT_CALLEE: u32 = 1;
        const COOKIE_CALLEE: u32 = 2;
        const MODE_GLOB: u32 = 0x01160CC8;
        const DONE_FLAG: u32 = 0x011E61EC;
        const OUT_BUF: u32 = 0x011E61ED;
        const OUT_VAL: u32 = 0x011E6220;
        const COOKIE_GLOB: u32 = 0x01057FB4;
        const FMT_A: u32 = 0x00EB54C0;
        const FMT_B: u32 = 0x00EB54C8;
        const FMT_C: u32 = 0x00EB54E0;
        const FMT_D: u32 = 0x00EB54E8;
        const BUF_LEN: usize = 52;
        let mode = lf_checker_rt::global::<u32>(MODE_GLOB).read();
        (lf_checker_rt::relocated(DONE_FLAG) as *mut u8).write(1);
        let fmt = if flag & 0xFF != 0 {
            if mode == 4 || mode == 1 { FMT_B } else { FMT_A }
        } else if mode == 4 || mode == 1 {
            FMT_D
        } else {
            FMT_C
        };
        let mut buf = [0u8; BUF_LEN];
        let _: u32 = lf_checker_rt::callee_cdecl!(
            FMT_CALLEE, u32, buf.as_mut_ptr() as u32, lf_checker_rt::relocated(fmt), a1
        );
        let mut i = 0usize;
        loop {
            let b = buf[i];
            (lf_checker_rt::relocated(OUT_BUF) as *mut u8).add(i).write(b);
            if b == 0 {
                break;
            }
            i += 1;
        }
        lf_checker_rt::global::<u32>(OUT_VAL).write(a2);
        let cookie = lf_checker_rt::global::<u32>(COOKIE_GLOB).read();
        let _: u32 = lf_checker_rt::callee_thiscall!(COOKIE_CALLEE, u32, cookie);
        0
    }
});
