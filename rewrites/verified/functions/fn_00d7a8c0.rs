// original: 0x00D7A8C0 expand_box_pair_guarded (proposed)

/// Widen two min/max boxes by a half-extent box, when the span gate passes.
///
/// `params` holds floats: half-extent at `+8`, centre x/y at `+0x30`/`+0x34`
/// and a top at `+0x38`. When 2.0 is strictly above `top - half` (ordered
/// ordered comparison, so NaN takes the early path), each of the x and y lanes
/// updates `minbox` with `min(old, centre - half)` and `maxbox` with
/// `max(old, centre + half)` -- min/max with the original's NaN choice
/// (NaN in the stored value wins, NaN in the candidate loses) -- then sets
/// the flag byte to 1. Returns `flag` on the full path, `params` on the
/// early path. Cdecl, four stack words. Float operation order is the
/// original's.
use lf_checker_rt::export;

export!(cdecl, rw_00d7a8c0(params: u32, minbox: u32, maxbox: u32, flag: u32) -> u32 {
    unsafe {
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits((a as *const u32).read_unaligned()) }
        }
        #[inline(always)]
        unsafe fn wrf(a: u32, v: f32) {
            unsafe { (a as *mut u32).write_unaligned(v.to_bits()) }
        }
        const HALF_OFF: u32 = 8;
        const CX_OFF: u32 = 0x30;
        const CY_OFF: u32 = 0x34;
        const TOP_OFF: u32 = 0x38;
        const SPAN_LIMIT: f32 = f32::from_bits(0x4000_0000); // 2.0
        let half = rdf(params + HALF_OFF);
        let cx = rdf(params + CX_OFF);
        let cy = rdf(params + CY_OFF);
        let top = rdf(params + TOP_OFF);
        if !(SPAN_LIMIT > sub(top, half)) {
            return params;
        }
        let lo = rdf(minbox);
        let cand = sub(cx, half);
        wrf(minbox, if lo > cand { cand } else { lo });
        let hi = rdf(maxbox);
        let candh = add(cx, half);
        wrf(maxbox, if candh > hi { candh } else { hi });
        let lo = rdf(minbox + 4);
        let cand = sub(cy, half);
        wrf(minbox + 4, if lo > cand { cand } else { lo });
        let hi = rdf(maxbox + 4);
        let candh = add(cy, half);
        wrf(maxbox + 4, if candh > hi { candh } else { hi });
        (flag as *mut u8).write(1);
        flag
    }
});
