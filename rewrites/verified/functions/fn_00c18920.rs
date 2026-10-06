// original: 0x00c18920 dist3dsq_under_threshold

/// Test whether two points are closer than the stored threshold.
///
/// Takes pointers to two triples of floats, forms the squared distance in the
/// original's order (`(dy*dy + dx*dx) + dz*dz`, where `d = a - b` per lane)
/// and returns 1 when the constant at `0x00e9cae0` (900.0) is strictly above
/// it (`seta` after `comiss`, so NaN gives 0), else 0.
///
/// Original: 0x00C18920 (stdcall, two stack words).
lf_checker_rt::export!(stdcall, rw_00c18920(a: u32, b: u32) -> u32 {
    unsafe {
        #[inline(always)]
        fn fmul(a: f32, b: f32) -> f32 {
            let r = core::hint::black_box(a) * core::hint::black_box(b);
            core::hint::black_box(a);
            core::hint::black_box(b);
            r
        }
        #[inline(always)]
        fn fadd(a: f32, b: f32) -> f32 {
            let r = core::hint::black_box(a) + core::hint::black_box(b);
            core::hint::black_box(a);
            core::hint::black_box(b);
            r
        }
        let thresh = lf_checker_rt::global::<f32>(0x00e9_cae0).read();
        let ax = (a as *const f32).read_unaligned();
        let ay = ((a + 4) as *const f32).read_unaligned();
        let az = ((a + 8) as *const f32).read_unaligned();
        let bx = (b as *const f32).read_unaligned();
        let by = ((b + 4) as *const f32).read_unaligned();
        let bz = ((b + 8) as *const f32).read_unaligned();
        let dx = ax - bx;
        let dy = ay - by;
        let dz = az - bz;
        let dx2 = fmul(dx, dx);
        let dy2 = fmul(dy, dy);
        let dz2 = fmul(dz, dz);
        let d = fadd(fadd(dy2, dx2), dz2);
        u32::from(thresh > d)
    }
});
