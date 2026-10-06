// original: 0x00928A20 plane_from_point_normal_negated (proposed)

/// Build a plane row from a point and a negated normal.
///
/// `a` points to three floats (a point), `d` to three floats (a direction),
/// `c` is the destination matrix. The three direction components are
/// bitwise-negated (xor with the sign mask at `SIGN_MASK`) and stored at
/// `c+0x0c`, `c+0x1c`, `c+0x2c`. Then the dot product
/// `a.x*d.x + a.y*d.y + a.z*d.z` is formed in the original's order
/// (`(a.y*d.y + a.x*d.x) + a.z*d.z`), `w` is subtracted, and the result is
/// stored at `c+0x3c`. Float operation order is the original's, pinned with
/// `black_box`.
///
/// Original: 0x00928A20 (cdecl, four stack words: a, d, w bits, c).
/// Leaf: no calls.
lf_checker_rt::export!(cdecl, rw_00928A20(a: u32, d: u32, w: u32, c: u32) -> u32 {
    unsafe {
        const SIGN_MASK: u32 = 0x00FE_8FA0;
        #[inline(always)]
        fn mul(x: f32, y: f32) -> f32 {
            core::hint::black_box(x) * core::hint::black_box(y)
        }
        #[inline(always)]
        fn add(x: f32, y: f32) -> f32 {
            core::hint::black_box(x) + core::hint::black_box(y)
        }
        #[inline(always)]
        fn sub(x: f32, y: f32) -> f32 {
            core::hint::black_box(x) - core::hint::black_box(y)
        }
        #[inline(always)]
        unsafe fn rdf(p: u32) -> f32 {
            unsafe { f32::from_bits((p as *const u32).read_unaligned()) }
        }
        #[inline(always)]
        unsafe fn wrf(p: u32, v: f32) {
            unsafe { (p as *mut u32).write_unaligned(v.to_bits()) }
        }
        let mask = (lf_checker_rt::relocated(SIGN_MASK) as *const u32).read_unaligned();
        for i in 0..3u32 {
            let bits = (d.wrapping_add(i * 4) as *const u32).read_unaligned() ^ mask;
            (c.wrapping_add(0x0c + i * 0x10) as *mut u32).write_unaligned(bits);
        }
        let t0 = mul(rdf(a.wrapping_add(4)), rdf(d.wrapping_add(4)));
        let t1 = mul(rdf(a), rdf(d));
        let s1 = add(t0, t1);
        let t2 = mul(rdf(a.wrapping_add(8)), rdf(d.wrapping_add(8)));
        let s2 = add(s1, t2);
        let r = sub(s2, f32::from_bits(w));
        wrf(c.wrapping_add(0x3c), r);
        0
    }
});
