// original: 0x00B34140 introsort_28B_key12 (proposed)

/// Introsort main loop over 28-byte records keyed by the float at offset 12.
///
/// `first`/`last` bound the run (`last` exclusive). The third word is unread.
/// `depth` is the remaining recursion budget: it is decremented in place on
/// every pass through the loop. `extra` is passed through to both callees.
/// Returns void (the original leaves garbage in eax).
///
/// Runs of 16 records or fewer are left alone for the caller's final insertion
/// pass. Otherwise, while budget remains, each pass picks the median of the
/// first, middle and last record's keys (NaN sorts as neither above nor below:
/// every test is `x > y` or its negation, exactly the original's `comiss`
/// with `ja`-to-take / `jbe`-to-stop), hands the median record to the
/// partition callee by value (first, last, seven words, extra), recurses on
/// the right half with the decremented budget and loops on the left half.
/// When the budget reaches zero the fallback-sort callee runs instead
/// (first, last, last, extra) and the run is left to the caller.
///
/// Original: 0x00B34140 (cdecl, five stack words), three call sites (the
/// self-call runs natively on both sides), no globals.
lf_checker_rt::export!(cdecl, rw_00b34140(first: u32, last: u32, _dead: u32, depth: u32, extra: u32) -> u32 {
    unsafe {
        const STRIDE: u32 = 28;
        const KEY_OFF: u32 = 12;
        const CUTOFF: i32 = 16;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }

        #[inline(always)]
        unsafe fn key(rec: u32) -> f32 {
            unsafe { f32::from_bits(rd32(rec.wrapping_add(KEY_OFF))) }
        }

        /// Median of the three records' keys: the middle value. Written as
        /// the original's five-test tree (first/middle, then middle/last or
        /// first/last, then the remaining pair), so NaN behaves identically.
        #[inline(always)]
        unsafe fn median_of_three(f: u32, m: u32, l: u32) -> u32 {
            unsafe {
                let a = key(f);
                let b = key(m);
                let c = key(l);
                if !(a > b) {
                    if !(a > c) {
                        if b > c { l } else { m }
                    } else {
                        f
                    }
                } else if b > c {
                    m
                } else if a > c {
                    l
                } else {
                    f
                }
            }
        }

        unsafe fn sort(lo: u32, hi: u32, depth: u32, extra: u32) {
            unsafe {
                let count = hi.wrapping_sub(lo) as i32 / STRIDE as i32;
                if count <= CUTOFF {
                    return;
                }
                let mut lo = lo;
                let mut hi = hi;
                let mut d = depth;
                loop {
                    if d == 0 {
                        lf_checker_rt::callee_cdecl!(2, u32, lo, hi, hi, extra);
                        return;
                    }
                    d = d.wrapping_sub(1);
                    let n = hi.wrapping_sub(lo) as i32 / STRIDE as i32;
                    let mid_rec = lo.wrapping_add((n / 2) as u32 * STRIDE);
                    let piv = median_of_three(lo, mid_rec, hi.wrapping_sub(STRIDE));
                    let w0 = rd32(piv);
                    let w1 = rd32(piv.wrapping_add(4));
                    let w2 = rd32(piv.wrapping_add(8));
                    let w3 = rd32(piv.wrapping_add(12));
                    let w4 = rd32(piv.wrapping_add(16));
                    let w5 = rd32(piv.wrapping_add(20));
                    let w6 = rd32(piv.wrapping_add(24));
                    let mid: u32 = lf_checker_rt::callee_cdecl!(1, u32, lo, hi, w0, w1, w2, w3, w4, w5, w6, extra);
                    sort(mid, hi, d, extra);
                    let left = mid.wrapping_sub(lo) as i32 / STRIDE as i32;
                    if left <= CUTOFF {
                        return;
                    }
                    hi = mid;
                }
            }
        }

        sort(first, last, depth, extra);
        0
    }
});
