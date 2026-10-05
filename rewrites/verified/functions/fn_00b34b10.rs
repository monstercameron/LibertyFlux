// original: 0x00B34B10 heap_adjust_28_b

/// Twin of `rw_00B34A40` with the offset-0 keys and the operands swapped.
///
/// Calls callee 1, then for each element calls callee 2 (same twelve-word
/// shape) when the reference key at `[refp]` is strictly greater than the
/// element key at `[p]`, then callee 3, returning its answer. Cdecl, five
/// stack words (the fourth is padding).
///
/// Original: 0x00B34B10.

lf_checker_rt::export!(cdecl, rw_00B34B10(refp: u32, lo: u32, limit: u32, _w3: u32, extra: u32) -> u32 {
    unsafe {
        const STRIDE: u32 = 28;
        const PRE: u32 = 1;
        const FIX: u32 = 2;
        const POST: u32 = 3;
        let _: u32 = lf_checker_rt::callee_cdecl!(PRE, u32, refp, lo, extra);
        let rkey = (refp as *const f32).read_unaligned();
        let mut p = lo;
        while p < limit {
            let skey = (p as *const f32).read_unaligned();
            if rkey > skey {
                let b0 = (p as *const u32).read_unaligned();
                let b1 = (p.wrapping_add(4) as *const u32).read_unaligned();
                let b2 = (p.wrapping_add(8) as *const u32).read_unaligned();
                let b3 = (p.wrapping_add(12) as *const u32).read_unaligned();
                let b4 = (p.wrapping_add(16) as *const u32).read_unaligned();
                let b5 = (p.wrapping_add(20) as *const u32).read_unaligned();
                let b6 = (p.wrapping_add(24) as *const u32).read_unaligned();
                let _: u32 = lf_checker_rt::callee_cdecl!(
                    FIX, u32, refp, lo, p, b0, b1, b2, b3, b4, b5, b6, extra, 0
                );
            }
            p = p.wrapping_add(STRIDE);
        }
        lf_checker_rt::callee_cdecl!(POST, u32, refp, lo, extra)
    }
});
