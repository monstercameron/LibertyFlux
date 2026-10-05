// original: 0x008e6700 float_key_introsort
/// Float-key introsort over 8-byte elements (u32 id, f32 key at +4):
/// insertion helper when depth runs out or the range is small, else a
/// median-of-3 pivot select, a partition call, a self call on the right
/// part and a loop on the left.
///
/// Original: cdecl/5 (lo ptr, hi/end ptr, DEAD/unread, depth, context).
/// Small ranges return the masked span; otherwise EAX is the last helper
/// answer up the taken path. Median gates use ordered `>` (false when
/// unordered), exactly matching the original's branch conditions.
export!(cdecl, rw_008e6700(lo: u32, hi: u32, _dead: u32, depth: u32, ctx: u32) -> u32 {
    unsafe {
        // Small range: eax holds the masked span on this path (signed test).
        if ((hi.wrapping_sub(lo) & !7) as i32) <= 0x80 {
            return hi.wrapping_sub(lo) & !7;
        }
        let mut hi = hi;
        let mut depth = depth;
        loop {
            if depth == 0 {
                // Depth exhausted: insertion helper over (lo, hi, hi, ctx).
                return callee_cdecl!(3, u32, lo, hi, hi, ctx);
            }
            depth = depth.wrapping_sub(1);
            // Middle element: ((n - sign(n)) >> 1) with n = (hi-lo)/8,
            // matching the shift/subtract/shift sequence exactly.
            let q = (hi.wrapping_sub(lo) >> 3) as i32;
            let edx = if q < 0 { -1i32 } else { 0 };
            let mid_n = (q.wrapping_sub(edx) >> 1) as u32;
            let mid = lo.wrapping_add(mid_n.wrapping_mul(8));
            let last = hi.wrapping_sub(8);
            let k_first = *((lo + 4) as *const f32);
            let k_mid = *((mid + 4) as *const f32);
            let k_last = *((last + 4) as *const f32);
            let piv_ptr: u32;
            if k_first > k_mid {
                if k_mid > k_last {
                    piv_ptr = mid;
                } else if k_first > k_last {
                    piv_ptr = last;
                } else {
                    piv_ptr = lo;
                }
            } else if k_first > k_last {
                piv_ptr = lo;
            } else if k_mid > k_last {
                piv_ptr = last;
            } else {
                piv_ptr = mid;
            }
            let piv_id = *(piv_ptr as *const u32);
            let piv_key = *((piv_ptr + 4) as *const u32);
            let piv_pos: u32 = callee_cdecl!(1, u32, lo, hi, piv_id, piv_key, ctx);
            let self_ans: u32 = callee_cdecl!(2, u32, piv_pos, hi, 0, depth, ctx);
            hi = piv_pos;
            if ((hi.wrapping_sub(lo) & !7) as i32) <= 0x80 {
                return self_ans;
            }
        }
    }
});
