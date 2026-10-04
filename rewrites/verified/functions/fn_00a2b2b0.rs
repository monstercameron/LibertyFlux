// original: 0x00a2b2b0 ped_move_scale_query

/// Query a movement scale factor from the ped's extension block.
/// Reads the float at `+0x3B8` of the inner block (`[this+0x228] + 0x70`,
/// faulting on a null extension like the original). When it is above the
/// global at `G_T1`, the result is `table[index * 32] * max(v / G_T1 -
/// G_K, +0.0 or the difference itself at exact zero) + G_K`, with the
/// arithmetic in the original's order; otherwise the result is 1.0 when the
/// value is above the global at `G_T3` and 0.0 when at or below it (NaN
/// counts as below in both comparisons). `index` selects one of eight
/// 32-byte table rows at `G_TAB`.
/// Original: 0x00a2b2b0 (thiscall, one stack word, float result on ST0).
lf_checker_rt::export!(thiscall, rw_00a2b2b0(this: u32, index: u32) -> f32 {
    unsafe {
    unsafe fn rd32(a: u32) -> u32 {
        unsafe { (a as *const u32).read_unaligned() }
    }
    unsafe fn rdf(a: u32) -> f32 {
        unsafe { f32::from_bits(rd32(a)) }
    }
    unsafe fn gbit(file_va: u32) -> f32 {
        unsafe { f32::from_bits(rd32(lf_checker_rt::relocated(file_va))) }
    }
    fn fmul(a: f32, b: f32) -> f32 {
        core::hint::black_box(a) * core::hint::black_box(b)
    }
    fn fadd(a: f32, b: f32) -> f32 {
        core::hint::black_box(a) + core::hint::black_box(b)
    }
    fn fsub(a: f32, b: f32) -> f32 {
        core::hint::black_box(a) - core::hint::black_box(b)
    }
    fn fdiv(a: f32, b: f32) -> f32 {
        core::hint::black_box(a) / core::hint::black_box(b)
    }
        const EXT: u32 = 0x228;
        const INNER: u32 = 0x70;
        const VAL: u32 = 0x3b8;
        const G_T1: u32 = 0x103c7c0;
        const G_K: u32 = 0x00fe88e8;
        const G_T3: u32 = 0x00fe8628;
        const G_TAB: u32 = 0x103c7e4;
        const ROW: u32 = 32;
        let raw = rd32(this.wrapping_add(EXT));
        let inner = if raw == 0 { 0 } else { raw.wrapping_add(INNER) };
        let v = rdf(inner.wrapping_add(VAL));
        if v > gbit(G_T1) {
            let q = fdiv(v, gbit(G_T1));
            let k = fsub(q, gbit(G_K));
            // The original keeps a non-positive difference only when it is
            // strictly below zero; exact zeros keep their sign and NaN
            // passes through.
            let lo = if k >= 0.0 || k.is_nan() { k } else { 0.0 };
            let t = gbit(index.wrapping_mul(ROW).wrapping_add(G_TAB));
            fadd(fmul(t, lo), gbit(G_K))
        } else if v > gbit(G_T3) {
            1.0
        } else {
            0.0
        }
    }
});
