// original: 0x008ED2D0 format_and_notify (proposed)

/// Format two values into a scratch buffer and notify with it.
///
/// Formats `a1` and `a2` with the format string at `FMT` into a 64-byte
/// scratch buffer (callee 1, cdecl, as `(buf, 64, a1, a2, 0, FMT)`), then
/// notifies (callee 2, thiscall on `obj`) as `(a0, buf)` and returns the
/// notification's answer. The buffer lives in the original's own frame, so
/// its address differs between the sides: the contract skips both buffer
/// arguments, has the formatter write scripted words into it, and compares
/// a 16-word call-time snapshot of it at the notification. The trailing
/// security-cookie check (callee 3) is intercepted with register
/// preservation and its cookie argument is not compared.
///
/// Original: 0x008ED2D0 (thiscall, three stack arguments).
lf_checker_rt::export!(thiscall, rw_008ED2D0(obj: u32, a0: u32, a1: u32, a2: u32) -> u32 {
    unsafe {
        /// Format string (file VA in read-only data).
        const FMT: u32 = 0xE832F4;
        /// Scratch buffer size in bytes.
        const BUF_LEN: u32 = 0x40;
        /// Formatter callee id.
        const FMT_CALLEE: u32 = 1;
        /// Notify callee id.
        const NOTIFY: u32 = 2;
        /// Security-cookie check callee id.
        const COOKIE: u32 = 3;

        let mut buf = [0u32; 16];
        let buf_ptr = &mut buf as *mut u32 as u32;
        let _: u32 = lf_checker_rt::callee_cdecl!(
            FMT_CALLEE,
            u32,
            buf_ptr,
            BUF_LEN,
            a1,
            a2,
            0,
            lf_checker_rt::relocated(FMT)
        );
        let ans: u32 =
            lf_checker_rt::callee_thiscall!(NOTIFY, u32, obj, a0, buf_ptr);
        let _: u32 = lf_checker_rt::callee_cdecl!(COOKIE, u32,);
        ans
    }
});
