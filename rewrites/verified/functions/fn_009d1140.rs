// original: 0x009D1140 strsafe_copy_cch (proposed)
//
/// Copies a string into a sized buffer after validating the destination.
///
/// Validates `dst`/`dstsize` through the length helper (callee), which also
/// reports the current string length there (observed via the call snapshot).
/// A negative (signed) status aborts with that status. Otherwise `src` is
/// copied to `dst + len`: bytes until the first NUL that fits, NUL-terminated
/// with status 0 on success; when `src` does not fit (or `len` already covers
/// `dstsize`) the last byte is forced to NUL and the status is
/// `INSUFFICIENT_BUFFER` (`0x8007007a`). The original also counts down a second
/// bound from `0x7ffffffe` per byte; it can never reach zero on these inputs.
/// Stdcall, three stack words
/// `(dst, dstsize, src)`.
lf_checker_rt::export!(stdcall, rw_009D1140(dst: u32, dstsize: u32, src: u32) -> u32 {
    unsafe {
        const LEN_CALLEE: u32 = 1;
        const MAX_CCH: u32 = 0x7fffffff;
        const INSUFF: u32 = 0x8007007a;
        let mut len = 0u32;
        let st: u32 = lf_checker_rt::callee_stdcall!(
            LEN_CALLEE, u32, dst, dstsize, (&mut len as *mut u32) as u32, MAX_CCH);
        if (st as i32) < 0 {
            return st;
        }
        let remaining = dstsize.wrapping_sub(len);
        let mut out = dst.wrapping_add(len);
        if remaining == 0 {
            out = out.wrapping_sub(1);
            (out as *mut u8).write(0);
            return INSUFF;
        }
        let mut left = remaining;
        let mut p = src;
        loop {
            let b = (p as *const u8).read();
            if b == 0 {
                break;
            }
            (out as *mut u8).write(b);
            out = out.wrapping_add(1);
            p = p.wrapping_add(1);
            left = left.wrapping_sub(1);
            if left == 0 {
                out = out.wrapping_sub(1);
                (out as *mut u8).write(0);
                return INSUFF;
            }
        }
        (out as *mut u8).write(0);
        0
    }
});
