// original: 0x00B343F0 introsort_28B_key0 (proposed)

/// Introsort main loop over 28-byte records keyed by the float at offset 0.
///
/// Same routine as rw_00b34140 (same five-test median tree, same cutoff,
/// same callee shapes: partition takes first, last, the seven median words
/// and extra; the fallback takes first, last, last and extra) except the key
/// is the float at record offset 0. Returns void.
///
/// Original: 0x00B343F0 (cdecl, five stack words), three call sites (the
/// self-call runs natively on both sides), no globals.
lf_checker_rt::export!(cdecl, rw_00b343f0(first: u32, last: u32, _dead: u32, depth: u32, extra: u32) -> u32 {
    unsafe {
        const STRIDE: u32 = 28;
        const KEY_OFF: u32 = 0;
        const CUTOFF: i32 = 16;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }

        #[inline(always)]
        unsafe fn key(rec: u32) -> f32 {
            unsafe { f32::from_bits(rd32(rec.wrapping_add(KEY_OFF))) }
        }

        /// Median of the three records' keys; same tree as rw_00b34140.
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
