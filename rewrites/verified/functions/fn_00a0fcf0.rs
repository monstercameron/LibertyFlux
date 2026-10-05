// original: 0x00a0fcf0 slab_dot4_test (proposed)
/// Test a point against four planes, reporting whether any is exceeded.
///
/// Forms `d = point - origin` from the three floats at `p` minus those at
/// `this + 0x0`, then evaluates four dot products against the coefficient
/// triples at `this + 0x10`, `+0x20`, `+0x30` and `+0x40`, each in the
/// original's SSE order: the first three are `(cy*dy + cx*dx) + cz*dz` and
/// the last is `(dy*sy + dx*sx) + dz*sz`. Returns 1 (low byte) when any
/// product is strictly above `limit`, else 0; an unordered NaN comparison
/// continues to the next plane. Thiscall, two stack arguments.
export!(thiscall, rw_00a0fcf0(this: u32, p: u32, limit: u32) -> u32 {
    unsafe {
        #[inline(always)]
        unsafe fn rdf(slot: u32) -> f32 {
            unsafe { f32::from_bits((slot as *const u32).read_unaligned()) }
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        let dx = sub(rdf(p), rdf(this));
        let dy = sub(rdf(p + 4), rdf(this + 4));
        let dz = sub(rdf(p + 8), rdf(this + 8));
        let t = f32::from_bits(limit);
        for k in 0..3u32 {
            let base = this + 0x10 + 0x10 * k;
            let d = add(
                add(mul(rdf(base + 4), dy), mul(rdf(base), dx)),
                mul(rdf(base + 8), dz),
            );
            if d > t {
                return 1;
            }
        }
        let sx = rdf(this + 0x40);
        let sy = rdf(this + 0x44);
        let sz = rdf(this + 0x48);
        let d4 = add(add(mul(dy, sy), mul(dx, sx)), mul(dz, sz));
        (d4 > t) as u32
    }
});
