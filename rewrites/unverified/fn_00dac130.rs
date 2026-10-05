// original: 0x00dac130 hassle_ped_sight_probe (proposed)
//
// cdecl/5 (world, pos, angle, gate, out): the largest sibling probe. When
// gate is zero a 2-word setup callee runs first; its answer (or gate
// itself) picks the blend constant (1.72 for answer 1, else 1.1). The
// sine/cosine helper pair, the shared 9-word probe and the same zeroed
// work region as the siblings follow (sub-views +0x20/+0x10/+0x30,
// constants (0.25, 0, 1, 0x8e, 0)). A nonzero probe answer returns its
// residue. Otherwise a second 9-word callee Q runs with an extra scalar
// between its views (a0, +0x10, 1.3, +0x08, +0x30, 0.25, 0, 0x8e, 0); the
// tail returns whether (picked - pos[2]) is below -0.1 and, when out is
// non-null, stores the picked value there. Float order is the original's
// SSE order.
lf_checker_rt::export!(cdecl, rw_00dac130(a0: u32, a1: u32, a2: u32, a3: u32, a4: u32) -> u32 {
    unsafe {
        const K_HI: u32 = 0x00ef023c; // 1.72
        const K_LO: u32 = 0x00fe8914; // 1.1
        const C06: u32 = 0x00fe8854; // 0.6
        const CN01: u32 = 0x00fe8d64; // -0.1
        const C01: u32 = 0x00fe879c; // 0.1
        const C13: u32 = 0x00fe892c; // 1.3
        const VEC: [u32; 3] = [0x001b4b320, 0x001b4b324, 0x001b4b328];
        const F025: u32 = 0x3e800000;
        const F13: u32 = 0x3fa66666;
        const D: u32 = 1;
        const S: u32 = 2;
        const C: u32 = 3;
        const P1: u32 = 4;
        const Q: u32 = 5;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        unsafe fn wrf(a: u32, v: f32) {
            unsafe { (a as *mut u32).write_unaligned(v.to_bits()) }
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
        unsafe fn grdf(va: u32) -> f32 {
            unsafe { f32::from_bits(rd32(lf_checker_rt::relocated(va))) }
        }

        let mut fr = [0u8; 0x90];
        let base = fr.as_mut_ptr() as u32;
        #[inline(always)]
        unsafe fn wrf_at(base: u32, at: u32, v: f32) {
            unsafe { ((base + at) as *mut u32).write_unaligned(v.to_bits()) }
        }
        #[inline(always)]
        unsafe fn rdf_at(base: u32, at: u32) -> f32 {
            unsafe { f32::from_bits(((base + at) as *const u32).read_unaligned()) }
        }
        #[inline(always)]
        unsafe fn wr32_at(base: u32, at: u32, v: u32) {
            unsafe { ((base + at) as *mut u32).write_unaligned(v) }
        }

        let gate = if a3 == 0 {
            lf_checker_rt::callee_cdecl!(D, u32, a0, a1)
        } else {
            a3
        };
        let k = if gate == 1 { grdf(K_HI) } else { grdf(K_LO) };
        wrf_at(base, 0x0c, k);
        let sin = f32::from_bits(lf_checker_rt::callee_cdecl!(S, u32, a2));
        wrf_at(base, 0x08, sin);
        let cos = f32::from_bits(lf_checker_rt::callee_cdecl!(C, u32, a2));
        let mut x5 = rdf_at(base, 0x08);
        let mut x3 = rdf(a1);
        let mut x4 = rdf(a1 + 4);
        let mut x2 = rdf(a1 + 8);
        x2 = add(x2, grdf(C06));
        let mut x1 = x5;
        x1 = mul(x1, grdf(CN01));
        x5 = mul(x5, k);
        let mut x6 = cos;
        let mut x0 = mul(cos, grdf(C01));
        x6 = mul(x6, k);
        x3 = sub(x3, x1);
        x1 = grdf(VEC[1]);
        x4 = sub(x4, x0);
        x0 = grdf(VEC[2]);
        // Stored here: the original spills x3 after the first sub only.
        wrf_at(base, 0x20, x3);
        x3 = sub(x3, x5);
        x6 = add(x6, x4);
        wrf_at(base, 0x28, x2);
        wrf_at(base, 0x18, x2);
        x2 = grdf(VEC[0]);
        wrf_at(base, 0x24, x4);
        wrf_at(base, 0x10, x3);
        wrf_at(base, 0x14, x6);
        // +0x07 flag byte, +0x30, +0x70, +0x74, +0x78, +0x80, +0x82 stay zero.
        wrf_at(base, 0x40, x2);
        wrf_at(base, 0x44, x1);
        wrf_at(base, 0x48, x0);
        wrf_at(base, 0x50, x2);
        wrf_at(base, 0x54, x1);
        wrf_at(base, 0x58, x0);
        wrf_at(base, 0x60, x2);
        wrf_at(base, 0x64, x1);
        wrf_at(base, 0x68, x0);
        wr32_at(base, 0x7c, 0xffff);
        let p1: u32 = lf_checker_rt::callee_cdecl!(
            P1, u32, a0, base + 0x20, base + 0x10, base + 0x30, F025, 0, 1, 0x8e, 0
        );
        if p1 & 0xff != 0 {
            return (p1 & !0xff) | ((base + 0x07) as *const u8).read() as u32;
        }
        let q: u32 = lf_checker_rt::callee_cdecl!(
            Q, u32, a0, base + 0x10, F13, base + 0x08, base + 0x30, F025, 0, 0x8e, 0
        );
        let picked = if q & 0xff != 0 {
            rdf_at(base, 0x08)
        } else {
            sub(rdf_at(base, 0x18), grdf(C13))
        };
        let d = sub(picked, rdf(a1 + 8));
        let r = if grdf(CN01) > d { 1 } else { 0 };
        if a4 != 0 {
            wrf(a4, picked);
        }
        r
    }
});
