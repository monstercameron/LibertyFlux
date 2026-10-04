// original: 0x00a1cca0 cam_dist_clamp_update (proposed)

/// Lowers a stored distance toward a newly measured one, with guards.
///
/// `this` points to a record holding a reference point (three floats at
/// `+REF_OFF`, `+REF_OFF+4`, `+REF_OFF+8`). `p` points to three measured
/// floats, `out` to the stored distance, and `limit`/`floor` are two
/// float arguments. The distance is
/// `sqrt((dx*dx + dy*dy) + dz*dz)` with that exact association, via the
/// scalar square root. The store updates only when the stored value is
/// strictly above both the distance and `limit`; the value written is
/// the larger of `floor` and the distance. Any NaN comparison takes the
/// keep-old-value path. Returns nothing.
///
/// Original: 0x00a1cca0 (thiscall, four stack words).
lf_checker_rt::export!(thiscall, rw_00a1cca0(this: u32, p: u32, out: u32, limit: u32, floor: u32) -> u32 {
    unsafe {
        const REF_OFF: u32 = 0x240;
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits((a as *const u32).read_unaligned()) }
        }
        let dx = sub(rdf(p), rdf(this + REF_OFF));
        let dy = sub(rdf(p + 4), rdf(this + REF_OFF + 4));
        let dz = sub(rdf(p + 8), rdf(this + REF_OFF + 8));
        let dist = add(add(mul(dx, dx), mul(dy, dy)), mul(dz, dz)).sqrt();
        let cur = rdf(out);
        if cur > dist && cur > f32::from_bits(limit) {
            let c = f32::from_bits(floor);
            let v = if c > dist { c } else { dist };
            (out as *mut u32).write_unaligned(v.to_bits());
        }
        0
    }
});
