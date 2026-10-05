// original: 0x00a4ee00 sort_introsort_loop (proposed)

/// Introsort over [first, last): partition, recurse right, loop left.
///
/// Ranges of sixteen elements or fewer (signed byte-span compare) return
/// for the insertion pass. With no depth left the heap-sort callee takes
/// the whole range with the limit slot set to `last` and `extra` kept. Otherwise the pivot
/// is the median of the first, middle and last elements' keys (strict
/// greater-than picks; ties and NaN fall through the documented side), the
/// partition callee splits the range, the right side recurses through this
/// same entry with one less depth, and the left side loops. The third
/// argument is unused; `extra` threads through. Cdecl, five stack words,
/// three callees, no result.
lf_checker_rt::export!(cdecl, rw_00a4ee00(first: u32, last: u32, _u: u32, depth: u32, extra: u32) -> u32 {
    unsafe {
        const KEY_OFF: u32 = 0x80;
        const SMALL_SPAN: u32 = 0x40;
        const PART: u32 = 1;
        const SELF: u32 = 2;
        const HEAP: u32 = 3;
        unsafe fn rd(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        unsafe fn key(elem: u32) -> f32 {
            unsafe { f32::from_bits(rd(elem.wrapping_add(KEY_OFF))) }
        }
        let mut hi = last;
        let mut depth = depth;
        if (((hi.wrapping_sub(first)) & !3) as i32) <= (SMALL_SPAN as i32) {
            return 0;
        }
        loop {
            if depth == 0 {
                lf_checker_rt::callee_cdecl!(HEAP, u32, first, hi, hi, extra);
                return 0;
            }
            let count = ((hi.wrapping_sub(first)) as i32) >> 2;
            let mptr = first.wrapping_add(((count >> 1).wrapping_mul(4)) as u32);
            let kf = key(rd(first));
            let km = key(rd(mptr));
            let kl = key(rd(hi.wrapping_sub(4)));
            let pivot: u32;
            if km > kf {
                if kl > km {
                    pivot = rd(mptr);
                } else {
                    let mut c = first;
                    if kl > kf {
                        c = hi.wrapping_sub(4);
                    }
                    pivot = rd(c);
                }
            } else if kl > kf {
                pivot = rd(first);
            } else {
                let mut c = mptr;
                if kl > km {
                    c = hi.wrapping_sub(4);
                }
                pivot = rd(c);
            }
            depth = depth.wrapping_sub(1);
            let p = lf_checker_rt::callee_cdecl!(PART, u32, first, hi, pivot, extra);
            lf_checker_rt::callee_cdecl!(SELF, u32, p, hi, 0, depth, extra);
            hi = p;
            if (((p.wrapping_sub(first)) & !3) as i32) <= (SMALL_SPAN as i32) {
                return 0;
            }
        }
    }
});
