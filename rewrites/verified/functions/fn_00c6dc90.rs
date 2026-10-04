// original: 0x00c6dc90 introsort_loop
/// Introsort over `[first, last)`: partition around the median of the first,
/// middle and last keys, recurse (via the intercepted call) into the right
/// half and loop on the left. Ranges of 16 elements or fewer are left for
/// the final insertion pass, and an exhausted `depth` falls back to heapsort.
/// The third argument is unused.
export!(cdecl, rw_00c6dc90(first: u32, last: u32, _u: u32, depth: u32, extra: u32) -> () {
    unsafe {
        let mut lo = first;
        let mut hi = last;
        let mut d = depth;
        loop {
            if ((hi.wrapping_sub(lo) & 0xFFFF_FFF8) as i32) <= 0x80 {
                return;
            }
            if d == 0 {
                callee_cdecl!(3, u32, lo, hi, hi, extra);
                return;
            }
            let n = ((hi.wrapping_sub(lo) as i32) >> 3);
            let midp = lo.wrapping_add(((n / 2) as u32).wrapping_mul(8));
            let lastp = hi.wrapping_sub(8);
            let a = *(lo as *const u32) as i32;
            let b = *(midp as *const u32) as i32;
            let c = *(lastp as *const u32) as i32;
            let piv = if a < b {
                if b < c {
                    midp
                } else if a < c {
                    lastp
                } else {
                    lo
                }
            } else if a < c {
                lo
            } else if b < c {
                lastp
            } else {
                midp
            };
            let pk = *(piv as *const u32);
            let pv = *((piv + 4) as *const u32);
            let m = callee_cdecl!(1, u32, lo, hi, pk, pv, extra);
            d = d.wrapping_sub(1);
            callee_cdecl!(2, u32, m, hi, 0, d, extra);
            hi = m;
        }
    }
});
