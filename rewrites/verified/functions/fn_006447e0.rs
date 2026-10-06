// original: 0x006447E0 rage::ptxDomainVortex::vf4

/// Sample a vortex domain's swirl velocity at the point `inp` into `out`.
///
/// The offset from the domain centre (`+0xe0..+0xe8`) to `inp` is formed
/// per component, its squared length taken as
/// `(dx*dx + dy*dy) + dz*dz`, and, unless that sum is exactly zero, the
/// square-root callee (id 1) converts it to a length whose reciprocal
/// (shared constant over the root) normalises the offset. The output is
/// the cross product of the axis stored at `+0xc0..+0xc8` with the
/// normalised offset: `x = ay*nz - ...` in full,
/// `out.x = c8*ny - c4*nz`, `out.y = c0*nz - c8*nx`,
/// `out.z = c4*nx - c0*ny`. A zero-length offset normalises with a zero
/// reciprocal instead of calling the root. A null `inp` writes a zero
/// vector without touching the object.
///
/// Original: 0x006447E0 (thiscall, two stack words: output pointer,
/// input pointer or null). Returns the output pointer. The only
/// comparisons are a null check and a zero test on the squared length,
/// so no signedness question arises. Float operation order is the
/// original's.
lf_checker_rt::export!(thiscall, rw_006447e0(this: u32, out: u32, inp: u32) -> u32 {
    unsafe {
        const CENTRE: u32 = 0xe0;
        const AXIS: u32 = 0xc0;
        const SHARED_ONE: u32 = 0x00FE88E8;
        const CALLEE_SQRT: u32 = 1;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
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

        if inp == 0 {
            wr32(out, 0);
            wr32(out.wrapping_add(4), 0);
            wr32(out.wrapping_add(8), 0);
            return out;
        }
        let dx = sub(rdf(inp), rdf(this.wrapping_add(CENTRE)));
        let dy = sub(rdf(inp.wrapping_add(4)), rdf(this.wrapping_add(CENTRE + 4)));
        let dz = sub(rdf(inp.wrapping_add(8)), rdf(this.wrapping_add(CENTRE + 8)));
        let sq = add(add(mul(dx, dx), mul(dy, dy)), mul(dz, dz));
        // ucomiss + lahf + test + jnp skips the root exactly when the
        // sum is +0 or -0; a sum of squares is never -0, and NaN takes
        // the root path, matching `== 0.0`.
        let inv = if sq == 0.0 {
            0.0f32
        } else {
            let root: f64 = lf_checker_rt::callee_cdecl!(CALLEE_SQRT, f64, sq.to_bits());
            div(rdf(lf_checker_rt::relocated(SHARED_ONE)), root as f32)
        };
        let nx = mul(inv, dx);
        let ny = mul(inv, dy);
        let nz = mul(inv, dz);
        let c0 = rdf(this.wrapping_add(AXIS));
        let c4 = rdf(this.wrapping_add(AXIS + 4));
        let c8 = rdf(this.wrapping_add(AXIS + 8));
        wrf(out, sub(mul(c8, ny), mul(c4, nz)));
        wrf(out.wrapping_add(4), sub(mul(c0, nz), mul(c8, nx)));
        wrf(out.wrapping_add(8), sub(mul(c4, nx), mul(c0, ny)));
        out
    }
});
