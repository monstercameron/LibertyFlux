// original: 0x0092B5E0 ortho_projection_row_from_extents (proposed)

/// Build one orthographic-projection-style matrix row from two extent vectors.
///
/// `p` and `q` point to three floats each, `out` to a 64-byte matrix.
/// The first two output scales are `UNIT / max(-p.x, q.x)` and
/// `UNIT / max(-p.y, q.y)`, where each max keeps the second operand unless
/// the first is ordered-greater (a NaN on either side takes the second
/// operand, matching the original's `comiss`/`ja`). The third scale is
/// `UNIT / (q.z - p.z)`. The scales land at `out+0x00`, `+0x14`, `+0x28`;
/// the translation row at `+0x30`, `+0x34`, `+0x38` holds
/// `(scale0 * 0, scale1 * 0, -p.z * scale2)`; `out+0x3c` is 1.0 and every
/// other word is 0. `UNIT` is the float at `UNIT_ADDR`, the negation is a
/// bitwise xor with the mask at `SIGN_MASK`. Float operation order is the
/// original's, pinned with `black_box`.
///
/// Original: 0x0092B5E0 (cdecl, three stack words: p, q, out). Leaf: no calls.
lf_checker_rt::export!(cdecl, rw_0092B5E0(p: u32, q: u32, out: u32) -> u32 {
    unsafe {
        const SIGN_MASK: u32 = 0x00FE_8FA0;
        const UNIT_ADDR: u32 = 0x00FE_88E8;
        #[inline(always)]
        fn above(x: f32, y: f32) -> bool {
            core::hint::black_box(x) > core::hint::black_box(y)
        }
        #[inline(always)]
        fn div(x: f32, y: f32) -> f32 {
            core::hint::black_box(x) / core::hint::black_box(y)
        }
        #[inline(always)]
        fn mul(x: f32, y: f32) -> f32 {
            core::hint::black_box(x) * core::hint::black_box(y)
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
        let unit = rdf(lf_checker_rt::relocated(UNIT_ADDR));
        let neg = |addr: u32| f32::from_bits((addr as *const u32).read_unaligned() ^ mask);
        let mut s0 = rdf(q);
        let npx = neg(p);
        if !above(s0, npx) {
            s0 = npx;
        }
        let mut s1 = rdf(q.wrapping_add(4));
        let npy = neg(p.wrapping_add(4));
        if !above(s1, npy) {
            s1 = npy;
        }
        let dz = sub(rdf(q.wrapping_add(8)), rdf(p.wrapping_add(8)));
        let r0 = div(unit, s0);
        let r1 = div(unit, s1);
        let r2 = div(unit, dz);
        wrf(out, r0);
        for off in [4u32, 8, 0x0c, 0x10] {
            (out.wrapping_add(off) as *mut u32).write_unaligned(0);
        }
        wrf(out.wrapping_add(0x14), r1);
        for off in [0x18u32, 0x1c, 0x20, 0x24] {
            (out.wrapping_add(off) as *mut u32).write_unaligned(0);
        }
        wrf(out.wrapping_add(0x28), r2);
        (out.wrapping_add(0x2c) as *mut u32).write_unaligned(0);
        let zero = 0.0f32;
        wrf(out.wrapping_add(0x30), mul(r0, zero));
        wrf(out.wrapping_add(0x34), mul(r1, zero));
        wrf(out.wrapping_add(0x38), mul(neg(p.wrapping_add(8)), r2));
        (out.wrapping_add(0x3c) as *mut u32).write_unaligned(0x3f80_0000);
        0
    }
});
