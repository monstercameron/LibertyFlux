// original: 0x00B346A0 introsort_16B_key0 (proposed)

/// Introsort main loop over 16-byte records keyed by the float at offset 0.
///
/// Same shape as rw_00b34140 for a shorter record: `first`/`last` bound the
/// run, the third word is unread, `depth` is the loop-shared budget passed
/// decremented into the right-half recursion, `extra` passes through.
/// Returns void.
///
/// Runs of 16 records or fewer (256 bytes or fewer) are left for the caller's
/// final insertion pass. Each pass otherwise takes the median of the first,
/// middle and last record's keys through the original's five-test tree (every
/// test is `x > y` or its negation, matching `comiss` with `ja`-to-take /
/// `jbe`-to-stop, NaN included), hands the median record to the partition
/// callee by value (first, last, four words, extra), recurses on the right
/// half and loops on the left. At zero budget the fallback-sort callee runs
/// (first, last, last, extra) instead.
///
/// Original: 0x00B346A0 (cdecl, five stack words), three call sites (the
/// self-call runs natively on both sides), no globals.
lf_checker_rt::export!(cdecl, rw_00b346a0(first: u32, last: u32, _dead: u32, depth: u32, extra: u32) -> u32 {
    unsafe {
        const STRIDE: u32 = 16;
        const CUTOFF_BYTES: i32 = 0x100;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }

        #[inline(always)]
        unsafe fn key(rec: u32) -> f32 {
            unsafe { f32::from_bits(rd32(rec)) }
        }

        /// Median of the three records' keys through the original's tree.
        #[inline(always)]
        unsafe fn median_of_three(f: u32, m: u32, l: u32) -> u32 {
            unsafe {
                let a = key(f);
                let b = key(m);
                let c = key(l);
                if !(b > a) {
                    if !(c > a) {
                        if c > b { l } else { m }
                    } else {
                        f
                    }
                } else if c > b {
                    m
                } else if c > a {
                    l
                } else {
                    f
                }
            }
        }

        #[inline(always)]
        fn run_len(lo: u32, hi: u32) -> i32 {
            hi.wrapping_sub(lo) as i32 & !15
        }

        unsafe fn sort(lo: u32, hi: u32, depth: u32, extra: u32) {
            unsafe {
                if run_len(lo, hi) <= CUTOFF_BYTES {
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
                    let n16 = (run_len(lo, hi) >> 4) / 2;
                    let mid_rec = lo.wrapping_add(n16 as u32 * STRIDE);
                    let piv = median_of_three(lo, mid_rec, hi.wrapping_sub(STRIDE));
                    let w0 = rd32(piv);
                    let w1 = rd32(piv.wrapping_add(4));
                    let w2 = rd32(piv.wrapping_add(8));
                    let w3 = rd32(piv.wrapping_add(12));
                    let mid: u32 = lf_checker_rt::callee_cdecl!(1, u32, lo, hi, w0, w1, w2, w3, extra);
                    sort(mid, hi, d, extra);
                    if run_len(lo, mid) <= CUTOFF_BYTES {
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
