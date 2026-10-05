// original: 0x00b91980 NativeImpl_SET_MENU_ITEM_WITH_2_NUMBERS

/// Copies a menu string onto the stack and forwards it with two numbers.
///
/// Copies the NUL-terminated string at `src` (the fourth stack word) byte
/// by byte into a 12-byte frame buffer (overlong strings overflow the
/// frame; the contract tests only strings of up to 11 characters, see
/// `narrowed`), then calls `MENU_CALLEE` with (`a0`, `a1`, `a2`, buffer,
/// `a4`, `a5`). Ends with the compiler's security-cookie check.
///
/// The buffer pointer differs legitimately between frames, so the contract
/// skips that call argument and snapshot-verifies the copied bytes; the
/// cookie-check argument (`cookie (an instruction of the original)`) is likewise uncompared.
///
/// Original: 0x00B91980 (cdecl, six stack words, no return value).
lf_checker_rt::export!(cdecl, rw_00b91980(a0: u32, a1: u32, a2: u32, src: u32, a4: u32, a5: u32) -> u32 {
    unsafe {
        const MENU_CALLEE: u32 = 1;
        const COOKIE_CALLEE: u32 = 2;
        const COOKIE_GLOB: u32 = 0x01057FB4;
        const BUF_LEN: usize = 12;
        let mut buf = [0u8; BUF_LEN];
        let mut i = 0usize;
        loop {
            let b = (src as *const u8).add(i).read();
            buf[i] = b;
            if b == 0 {
                break;
            }
            i += 1;
        }
        let _: u32 = lf_checker_rt::callee_cdecl!(
            MENU_CALLEE, u32, a0, a1, a2, buf.as_mut_ptr() as u32, a4, a5
        );
        let cookie = lf_checker_rt::global::<u32>(COOKIE_GLOB).read();
        let _: u32 = lf_checker_rt::callee_thiscall!(COOKIE_CALLEE, u32, cookie);
        0
    }
});
