// original: 0x00644DD0 rage::ptxDomainSphere::vf4

/// Sample a sphere domain's emission velocity at slot `out`, drawing a
/// random direction and radius from the shared generator state.
///
/// `this` is the domain object. Three 23-bit fractions are drawn from
/// the generator (words at `RNG_LO`/`RNG_HI`, multiplier `0x5CDCF...A7`
/// with carry folded back in), each mapped to `[-K3, +K3]`-ish range as
/// `((frac * K1) * K2) - K3` with shared constants K1..K3. The vector's
/// squared length is taken as `(x*x + y*y) + z*z` and, unless exactly
/// zero, the square-root callee (id 1) converts it to a length whose
/// reciprocal (K3 over the root) normalises the direction. A fourth
/// fraction scales the span between the radii at `+0x130`/`+0x134`:
/// `r = ((frac * K1) * (r0 - r1)) + r1`.
///
/// The output is the scaled direction run through the basis stored in
/// the object (columns at `+0xb0/+0xb4/+0xb8`, `+0xc0/+0xc4/+0xc8`,
/// `+0xd0/+0xd4/+0xd8`) plus the centre at `+0xe0..+0xe8`:
/// `out.x = c0*(r*ny) + b0*(r*nx) + d0*(r*nz) + e0`, and likewise for y
/// and z. The fourth output word copies an uninitialised frame word,
/// which the contract fixes to zero with a defined stack fill. When
/// `ctx` is non-null and the flag byte at `+0x125` is clear, `ctx[0..2]`
/// are subtracted from the output triple.
///
/// Original: 0x00644DD0 (thiscall, two stack words: output pointer for
/// four words, adjust pointer or null). Returns the output pointer. The
/// only compares are a null check, a flag-byte test and a zero test on
/// the squared length. Integer arithmetic is exact 32/64-bit wrapping;
/// float operation order is the original's.
lf_checker_rt::export!(thiscall, rw_00644dd0(this: u32, out: u32, ctx: u32) -> u32 {
    unsafe {
        const RNG_LO: u32 = 0x011101A0;
        const RNG_HI: u32 = 0x011101A4;
        const RNG_MULT: u64 = 0x5CDCFAA7;
        const RNG_MASK: u32 = 0x7FFFFF;
        const K1: u32 = 0x00FE864C;
        const K2: u32 = 0x00FE8A24;
        const K3: u32 = 0x00FE88E8;
        const BASIS_B: u32 = 0xb0;
        const BASIS_C: u32 = 0xc0;
        const BASIS_D: u32 = 0xd0;
        const CENTRE: u32 = 0xe0;
        const RADIUS0: u32 = 0x130;
        const RADIUS1: u32 = 0x134;
        const FLAG_SUB: u32 = 0x125;
        const CALLEE_SQRT: u32 = 1;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        unsafe fn wrf(a: u32, v: f32) {
            unsafe { wr32(a, v.to_bits()) }
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
        #[inline(always)]
        fn div(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) / core::hint::black_box(b)
        }

        let k1 = rdf(lf_checker_rt::relocated(K1));
        let k2 = rdf(lf_checker_rt::relocated(K2));
        let k3 = rdf(lf_checker_rt::relocated(K3));

        // First draw: three fractions; generator state advances twice.
        let lo = rd32(lf_checker_rt::relocated(RNG_LO));
        let hi = rd32(lf_checker_rt::relocated(RNG_HI));
        let p1 = (lo as u64).wrapping_mul(RNG_MULT);
        let (eax1, c1) = (p1 as u32).overflowing_add(hi);
        let edi1 = ((p1 >> 32) as u32).wrapping_add(c1 as u32);
        let p2 = (eax1 as u64).wrapping_mul(RNG_MULT);
        let f1 = (eax1 & RNG_MASK) as f32;
        let (eax2, c2) = (p2 as u32).overflowing_add(edi1);
        let esi2 = ((p2 >> 32) as u32).wrapping_add(c2 as u32);
        let f2 = (eax2 & RNG_MASK) as f32;
        let p3 = (eax2 as u64).wrapping_mul(RNG_MULT);
        let (lo_new, c3) = (p3 as u32).overflowing_add(esi2);
        let hi_new = ((p3 >> 32) as u32).wrapping_add(c3 as u32);
        let f3 = (lo_new & RNG_MASK) as f32;

        let x = sub(mul(mul(f1, k1), k2), k3);
        let y = sub(mul(mul(f2, k1), k2), k3);
        let z = sub(mul(mul(f3, k1), k2), k3);
        let sq = add(add(mul(x, x), mul(y, y)), mul(z, z));
        // Same ucomiss/lahf/test/jnp shape as the vortex sampler: the
        // root is skipped exactly when the sum is zero.
        let inv = if sq == 0.0 {
            0.0f32
        } else {
            let root: f64 = lf_checker_rt::callee_cdecl!(CALLEE_SQRT, f64, sq.to_bits());
            div(k3, root as f32)
        };
        let nx = mul(inv, x);
        let ny = mul(inv, y);
        let nz = mul(inv, z);

        // Second draw: one fraction; generator state advances again.
        let p4 = (lo_new as u64).wrapping_mul(RNG_MULT);
        let (lo2, c4) = (p4 as u32).overflowing_add(hi_new);
        let hi2 = ((p4 >> 32) as u32).wrapping_add(c4 as u32);
        let f4 = (lo2 & RNG_MASK) as f32;
        wr32(lf_checker_rt::relocated(RNG_LO), lo2);
        wr32(lf_checker_rt::relocated(RNG_HI), hi2);

        let r1 = rdf(this.wrapping_add(RADIUS1));
        let d = sub(rdf(this.wrapping_add(RADIUS0)), r1);
        let r = add(mul(mul(f4, k1), d), r1);
        let rnx = mul(r, nx);
        let rny = mul(r, ny);
        let rnz = mul(r, nz);

        let mut ox = add(
            add(
                mul(rdf(this.wrapping_add(BASIS_C)), rny),
                mul(rdf(this.wrapping_add(BASIS_B)), rnx),
            ),
            mul(rdf(this.wrapping_add(BASIS_D)), rnz),
        );
        ox = add(ox, rdf(this.wrapping_add(CENTRE)));
        let mut oy = add(
            add(
                mul(rdf(this.wrapping_add(BASIS_C + 4)), rny),
                mul(rdf(this.wrapping_add(BASIS_B + 4)), rnx),
            ),
            mul(rdf(this.wrapping_add(BASIS_D + 4)), rnz),
        );
        oy = add(oy, rdf(this.wrapping_add(CENTRE + 4)));
        let mut oz = add(
            add(
                mul(rdf(this.wrapping_add(BASIS_C + 8)), rny),
                mul(rdf(this.wrapping_add(BASIS_B + 8)), rnx),
            ),
            mul(rdf(this.wrapping_add(BASIS_D + 8)), rnz),
        );
        oz = add(oz, rdf(this.wrapping_add(CENTRE + 8)));

        wrf(out, ox);
        wrf(out.wrapping_add(4), oy);
        wr32(out.wrapping_add(12), 0);
        wrf(out.wrapping_add(8), oz);
        if ctx != 0 && rd8(this.wrapping_add(FLAG_SUB)) == 0 {
            ox = sub(ox, rdf(ctx));
            oy = sub(oy, rdf(ctx.wrapping_add(4)));
            oz = sub(oz, rdf(ctx.wrapping_add(8)));
            wrf(out, ox);
            wrf(out.wrapping_add(4), oy);
            wrf(out.wrapping_add(8), oz);
        }
        out
    }
});
