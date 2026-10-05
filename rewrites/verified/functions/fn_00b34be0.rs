// original: 0x00B34BE0 heap_adjust_16

/// Replace-and-notify sweep over 16-byte elements plus a countdown of plain
/// calls.
///
/// Calls callee 1 as `(refp, lo, extra)`. For each 16-byte element `p` from
/// `lo` while `p < limit` (unsigned), when the reference key at `[refp]` is
/// strictly greater than the element key at `[p]`, copies the reference
/// element over `[p]` and calls callee 2 as `(refp, 0, (lo-refp)>>4, old
/// words..., extra)` (eight stack words; the old element travels by value).
/// Then, while the shrinking span `span = end - refp` satisfies
/// `(span AND NOT 0xF) > 16` (signed), calls callee 3 as `(refp, end, extra)`
/// and shrinks `end` by 16. Returns the final masked span. Cdecl, five
/// stack words (the fourth is padding).
///
/// Original: 0x00B34BE0.

lf_checker_rt::export!(cdecl, rw_00B34BE0(refp: u32, lo: u32, limit: u32, _w3: u32, extra: u32) -> u32 {
    unsafe {
        const STRIDE: u32 = 16;
        const PRE: u32 = 1;
        const FIX: u32 = 2;
        const POST: u32 = 3;
        let _: u32 = lf_checker_rt::callee_cdecl!(PRE, u32, refp, lo, extra);
        let mut p = lo;
        while p < limit {
            let rkey = (refp as *const f32).read_unaligned();
            let skey = (p as *const f32).read_unaligned();
            if rkey > skey {
                let o0 = (p as *const u32).read_unaligned();
                let o1 = (p.wrapping_add(4) as *const u32).read_unaligned();
                let o2 = (p.wrapping_add(8) as *const u32).read_unaligned();
                let o3 = (p.wrapping_add(12) as *const u32).read_unaligned();
                for i in 0..4u32 {
                    let w = (refp.wrapping_add(i * 4) as *const u32).read_unaligned();
                    (p.wrapping_add(i * 4) as *mut u32).write_unaligned(w);
                }
                let count = ((lo.wrapping_sub(refp) as i32) >> 4) as u32;
                let _: u32 =
                    lf_checker_rt::callee_cdecl!(FIX, u32, refp, 0, count, o0, o1, o2, o3, extra);
            }
            p = p.wrapping_add(STRIDE);
        }
        let mut span = lo.wrapping_sub(refp);
        let mut end = lo;
        let mut e = span & 0xFFFF_FFF0;
        if (e as i32) > 16 {
            loop {
                let _: u32 = lf_checker_rt::callee_cdecl!(POST, u32, refp, end, extra);
                span = span.wrapping_sub(STRIDE);
                e = span & 0xFFFF_FFF0;
                end = end.wrapping_sub(STRIDE);
                if !((e as i32) > 16) {
                    break;
                }
            }
        }
        e
    }
});
