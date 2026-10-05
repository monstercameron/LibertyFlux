// original: 0x0093DF70 stream_tree_walk_recursive (proposed)

/// Sort a pointer range by quicksort with a depth fallback.
///
/// Ranges of 64 bytes or fewer answer the masked length at once.
/// Otherwise, while depth remains: pick the
/// unsigned median of the first, middle and last keys, partition the
/// range around the pivot element, sort the right part by recursion
/// (stubbed by the checker), and continue with the left part while it
/// still exceeds 64 bytes. With depth exhausted, the fallback sorts the
/// range directly. Answers the last worker's answer. (The second
/// argument slot is never read; ranges with end below begin are
/// excluded: the middle-index rounding differs there.)
lf_checker_rt::export!(cdecl, rw_0093df70(begin: u32, end: u32, _unused: u32, depth: u32, ctx: u32) -> u32 {
    unsafe {
        const PARTITION: u32 = 1;
        const RECURSE: u32 = 2;
        const FALLBACK: u32 = 3;
        const LEAF: u32 = 0x40;
        let key = |p: u32| -> u32 {
            (p as *const u32).read_unaligned()
        };
        let mut lo = begin;
        let mut hi = end;
        let mut left = depth;
        let mut answer: u32 = 0;
        // NOTE: short ranges answer the masked length still in EAX.
        let len0 = hi.wrapping_sub(lo) & !3;
        if ((len0 as i32) <= LEAF as i32) {
            return len0;
        }
        loop {
            if left == 0 {
                return lf_checker_rt::callee_cdecl!(FALLBACK, u32, lo, hi, hi, ctx);
            }
            let n = hi.wrapping_sub(lo);
            // Middle element: arithmetic shift right by 3 of the byte
            // count (exact for end >= begin, which the contract holds).
            let mid = lo.wrapping_add((((n as i32) >> 3) as u32).wrapping_mul(4));
            let p0 = (lo as *const u32).read_unaligned();
            let pm = (mid as *const u32).read_unaligned();
            let plast = ((hi.wrapping_sub(4)) as *const u32).read_unaligned();
            let k0 = key(p0);
            let km = key(pm);
            let kl = key(plast);
            // Unsigned median-of-three over (k0, km, kl), same compares.
            let pivot_at = if k0 >= km {
                if k0 >= kl {
                    if km < kl {
                        hi.wrapping_sub(4)
                    } else {
                        mid
                    }
                } else {
                    lo
                }
            } else if km < kl {
                mid
            } else if k0 < kl {
                hi.wrapping_sub(4)
            } else {
                lo
            };
            left = left.wrapping_sub(1);
            let pivot_val = (pivot_at as *const u32).read_unaligned();
            let pos: u32 = lf_checker_rt::callee_cdecl!(PARTITION, u32, lo, hi, pivot_val, ctx);
            answer = lf_checker_rt::callee_cdecl!(RECURSE, u32, pos, hi, 0, left, ctx);
            hi = pos;
            if (((pos.wrapping_sub(lo) & !3) as i32) <= LEAF as i32) {
                break;
            }
        }
        answer
    }
});
