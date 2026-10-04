// original: 0x00a2ccf0 ped_range_attenuation

/// Compute a range attenuation factor between two anchored points.
/// The source anchor is `[arg+0x20]` plus `0x30` (or `arg + 0x10` when
/// that pointer is null); the reference anchor is `[this][0x20]`. With
/// the per-axis differences in the original's order, the squared length
/// `l2` feeds `sqrt`; when the length is above the global at `G_ONE`
/// (NaN counts as above) the result is `G_ONE / sqrt(sqrt(l2))`, else
/// exactly 1.0. All arithmetic keeps the original's operand order.
/// Original: 0x00a2ccf0 (thiscall, one stack word, float result on ST0).
lf_checker_rt::export!(thiscall, rw_00a2ccf0(this: u32, arg: u32) -> f32 {
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
        const SRC_LINK: u32 = 0x20;
        const SRC_FALLBACK: u32 = 0x10;
        const LINK_SKIP: u32 = 0x30;
        const VT_LINK: u32 = 0x20;
        const REF_X: u32 = 0x30;
        const REF_Y: u32 = 0x34;
        const REF_Z: u32 = 0x38;
        const G_ONE: u32 = 0x00fe88e8;
        let q = rd32(arg.wrapping_add(SRC_LINK));
        let p = if q == 0 {
            arg.wrapping_add(SRC_FALLBACK)
        } else {
            q.wrapping_add(LINK_SKIP)
        };
        let base = rd32(rd32(this).wrapping_add(VT_LINK));
        let dx = fsub(rdf(p), rdf(base.wrapping_add(REF_X)));
        let dy = fsub(rdf(p.wrapping_add(4)), rdf(base.wrapping_add(REF_Y)));
        let dz = fsub(rdf(p.wrapping_add(8)), rdf(base.wrapping_add(REF_Z)));
        let l2 = fadd(fadd(fmul(dx, dx), fmul(dy, dy)), fmul(dz, dz));
        let one = gbit(G_ONE);
        let len = core::hint::black_box(l2).sqrt();
        if len <= one {
            1.0
        } else {
            fdiv(one, core::hint::black_box(len).sqrt().sqrt())
        }
    }
});
