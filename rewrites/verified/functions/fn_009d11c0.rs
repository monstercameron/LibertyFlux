// original: 0x009D11C0 bounded_fill_cch (proposed)
//
/// Fills a bounded buffer through a worker, then checks how much it wrote.
///
/// `count` must be nonzero and at most `0x7fffffff` (unsigned); otherwise the
/// status is `E_INVALIDARG` and `dst[0]` is cleared unless `count` is zero.
/// Else the worker (callee) is asked to produce `count - 1` units into `dst`
/// (its third input comes from the caller's next stack slot; its length
/// output goes to the slot after that, observed via the call snapshot). How
/// the worker's answer compares with `count - 1` decides the rest, with an
/// UNSIGNED above-check and a signedness-relevant negative check: a negative
/// answer or one above `count - 1` clears `dst[count-1]` and reports
/// `INSUFFICIENT_BUFFER`; an answer of exactly `count - 1` clears it and
/// reports success; a smaller non-negative answer reports success as is.
/// Cdecl; four stack slots are declared because the original reads a third
/// and writes a fourth (as worker input/output) past its two logical inputs.
lf_checker_rt::export!(cdecl, rw_009D11C0(dst: u32, count: u32, aux: u32, outslot: u32) -> u32 {
    unsafe {
        const WORKER: u32 = 1;
        const E_INVALIDARG: u32 = 0x80070057;
        const INSUFF: u32 = 0x8007007a;
        if count == 0 || count > 0x7fffffff {
            if count != 0 {
                (dst as *mut u8).write(0);
            }
            return E_INVALIDARG;
        }
        let bound = count.wrapping_sub(1);
        let mut len = outslot;
        let n: u32 = lf_checker_rt::callee_cdecl!(WORKER, u32, dst, bound, aux, (&mut len as *mut u32) as u32);
        if (n as i32) < 0 || n > bound {
            (dst.wrapping_add(bound) as *mut u8).write(0);
            return INSUFF;
        }
        if n == bound {
            (dst.wrapping_add(bound) as *mut u8).write(0);
        }
        0
    }
});
