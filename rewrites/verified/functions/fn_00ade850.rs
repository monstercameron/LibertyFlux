// original: 0x00ade850 introsort_loop (proposed)

/// Introsort the range, recursing on the larger side and looping on
/// the smaller one, with a heapsort fallback when the depth runs out.
///
/// `first` and `last` bound a word array, the third argument is
/// unread, `depth` is the remaining recursion allowance, `comp` is a
/// `cdecl(a, b) -> bool` comparator. Ranges of 16 words or fewer
/// return at once (the caller finishes them). Otherwise, when `depth`
/// is zero the heapsort callee (id 4: first, last, last, comparator)
/// runs and the function returns. Else the depth drops by one, a
/// median-of-three pivot is chosen between the first, middle and last
/// elements with two or three comparator calls in a fixed order, the
/// partition callee (id 2: first, last, pivot, comparator) splits the
/// range, the function recurses through the self callee (id 3) on the
/// upper part, and loops on the lower part while it still exceeds 16
/// words.
///
/// Edge cases: the median sequence always reads valid elements; the
/// loop re-reads the decremented depth from its own incoming slot, so
/// the rewrite keeps it in a local instead.
///
/// Original: cdecl, five stack words, one indirect comparator callee
/// (id 1, cdecl, two arguments) and three direct callees (id 2 cdecl
/// with four arguments, id 3 cdecl with five, id 4 cdecl with four).
/// Returns nothing.
lf_checker_rt::export!(cdecl, rw_00ade850(first: u32, last: u32, _unused: u32, depth: u32, comp: u32) -> u32 {
    unsafe {
        const SMALL_BYTES: u32 = 16 * 4;
        let cmp: extern "cdecl" fn(u32, u32) -> u8 =
            core::mem::transmute(comp as usize);
        let rd = |a: u32| (a as *const u32).read_unaligned();
        let span = last.wrapping_sub(first) & 0xFFFF_FFFC;
        if (span as i32) <= SMALL_BYTES as i32 {
            return 0;
        }
        let mut depth = depth;
        let mut lastv = last;
        loop {
            if depth == 0 {
                lf_checker_rt::callee_cdecl!(4, u32, first, lastv, lastv, comp);
                return 0;
            }
            depth = depth.wrapping_sub(1);
            let count = lastv.wrapping_sub(first) >> 2;
            let half = count / 2;
            let mid = first.wrapping_add(half.wrapping_mul(4));
            let lastm1 = lastv.wrapping_sub(4);
            let pivot_at = if cmp(rd(first), rd(mid)) != 0 {
                if cmp(rd(mid), rd(lastm1)) != 0 {
                    mid
                } else if cmp(rd(first), rd(lastm1)) != 0 {
                    lastm1
                } else {
                    first
                }
            } else if cmp(rd(first), rd(lastm1)) != 0 {
                first
            } else if cmp(rd(mid), rd(lastm1)) != 0 {
                lastm1
            } else {
                mid
            };
            let cut: u32 = lf_checker_rt::callee_cdecl!(2, u32, first, lastv, rd(pivot_at), comp);
            lf_checker_rt::callee_cdecl!(3, u32, cut, lastv, 0, depth, comp);
            let rest = cut.wrapping_sub(first) & 0xFFFF_FFFC;
            if (rest as i32) <= SMALL_BYTES as i32 {
                return 0;
            }
            lastv = cut;
        }
    }
});
