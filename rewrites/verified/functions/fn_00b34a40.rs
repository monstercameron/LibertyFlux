// original: 0x00B34A40 heap_adjust_28_a

/// Bracket a conditional fix-up sweep over `[lo, limit)` with two plain
/// calls, comparing the offset-12 float keys.
///
/// Calls callee 1 as `(refp, lo, extra)`. Then for each 28-byte element `p`
/// from `lo` while `p < limit` (unsigned), when the element key at `+12` is
/// strictly greater than the reference key at `[refp + 12]`, calls callee 2
/// as `(refp, lo, p, elem_words..., extra, 0)` (twelve stack words; the
/// struct travels by value on the stack and the `ecx` the original sets is
/// dead: the callee overwrites it on entry, so the contract uses plain
/// cdecl). Finishes with callee 3 as `(refp, lo, extra)` and returns its
/// answer. Cdecl, five stack words (the fourth is padding the original
/// never reads).
///
/// Original: 0x00B34A40.

lf_checker_rt::export!(cdecl, rw_00B34A40(refp: u32, lo: u32, limit: u32, _w3: u32, extra: u32) -> u32 {
    unsafe {
        const STRIDE: u32 = 28;
        const KEY_OFF: u32 = 12;
        const PRE: u32 = 1;
        const FIX: u32 = 2;
        const POST: u32 = 3;
        let _: u32 = lf_checker_rt::callee_cdecl!(PRE, u32, refp, lo, extra);
        let rkey = (refp.wrapping_add(KEY_OFF) as *const f32).read_unaligned();
        let mut p = lo;
        while p < limit {
            let skey = (p.wrapping_add(KEY_OFF) as *const f32).read_unaligned();
            if skey > rkey {
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
