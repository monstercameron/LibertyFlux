// original: 0x00b1e480 notify_triple (proposed)

/// Notifies three listeners about a key pair, with a fallback second key.
///
/// Cdecl of three stack words. Calls the first and third listeners with
/// (b, c); the middle listener gets (a, c) when b equals -1, else
/// (a, -1). All three callees are cdecl of two words. Returns the third
/// call's result.
lf_checker_rt::export!(cdecl, rw_00b1e480(a: u32, b: u32, c: u32) -> u32 {
    unsafe {
        const NONE: u32 = 0xffff_ffff;
        lf_checker_rt::callee_cdecl!(1, u32, b, c);
        let middle = if b == NONE { c } else { NONE };
        lf_checker_rt::callee_cdecl!(2, u32, a, middle);
        lf_checker_rt::callee_cdecl!(3, u32, b, c)
    }
});
