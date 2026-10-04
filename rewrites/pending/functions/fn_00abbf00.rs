// original: 0x00abbf00 sift_down_range
/// Drive callee 1 over the index range of a dword array, high to low.
///
/// With `n = (b - a) >> 2` (signed), indices `m = (n-2)/2` down to 0 are
/// each reported as `(base, index, count, element, ctx)`. Ranges shorter
/// than 2 elements return the count without calling. Note: the original
/// returns its incoming EAX on that short path (undefined value); this
/// rewrite returns the count. The contract only exercises `n >= 2`.
lf_checker_rt::export!(cdecl, rw_00abbf00(a: u32, b: u32, c: u32) -> u32 {
    let n = (b.wrapping_sub(a) as i32) >> 2;
    if n < 2 {
        return n as u32;
    }
    let mut m = (n - 2) / 2;
    let mut ans;
    loop {
        // SAFETY: caller array; the checker keeps every indexed element
        // inside heap memory on every trial.
        let elem = unsafe { ((a.wrapping_add((m as u32).wrapping_mul(4))) as *const u32).read_unaligned() };
        ans = lf_checker_rt::callee_cdecl!(1, u32, a, m as u32, n as u32, elem, c);
        if m == 0 {
            break;
        }
        m -= 1;
    }
    ans
});

