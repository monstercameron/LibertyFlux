// original: 0x00dac610 hassle_ped_cover_probe (proposed)
//
// cdecl/3 (world, pos, angle): like its sibling probe but the angle is
// first shifted by half-pi (so the helper pair runs in quadrature), a
// lazily cached constant joins the first probe's inputs, and the answer
// polarity is inverted (nonzero probe answers continue).
//
// The cache word at game .data holds a 0.4-ish constant (default from
// .rdata, stored on first use under flag bit 0). The bumped angle is
// written back to the caller's a2 slot, then feeds helper S (negated into
// the frame) and helper C. Probe 1 (falsy answer returns the residue, like
// the sibling's inverted logic) passes sub-views +0x20/+0x30/+0x50 with
// constants (0, 0x8e, 0, 0, ~0.086); probe 2 scales the spilled pair by
// 0.22 and -2.0, folds them into the inputs and passes +0x10/+0x30/+0x50.
// Returns the setne of the last probe answer. Float order is the
// original's SSE order.
lf_checker_rt::export!(cdecl, rw_00dac610(a0: u32, a1: u32, a2: u32) -> u32 {
    unsafe {
        const FLAG2: u32 = 0x017a6534;
        const CACHE2: u32 = 0x017a6530;
        const CDEF: u32 = 0x00fe881c; // 0.4
        const PI2: u32 = 0x00fe8978; // pi/2
        const K02: u32 = 0x00fe87d0; // 0.2
        const K022: u32 = 0x00eb2294; // 0.22
        const KN2: u32 = 0x00fe8db0; // -2.0
        const VEC: [u32; 3] = [0x001b4b320, 0x001b4b324, 0x001b4b328];
        const F6: u32 = 0x3dae147b; // probe constant
        const S: u32 = 1;
        const C: u32 = 2;
        const P1: u32 = 3;
        const P2: u32 = 4;
        const SIGN: u32 = 0x8000_0000;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
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
        unsafe fn grdf(va: u32) -> f32 {
            unsafe { f32::from_bits(rd32(lf_checker_rt::relocated(va))) }
        }

        let mut fr = [0u8; 0xb0];
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

        // Lazy constant (flag bit 0).
        let flag_addr = lf_checker_rt::relocated(FLAG2);
        let flag0 = rd32(flag_addr);
        let cc = if flag0 & 1 != 0 {
            rdf(lf_checker_rt::relocated(CACHE2))
        } else {
            let v = grdf(CDEF);
            wr32(flag_addr, flag0 | 1);
            wrf(lf_checker_rt::relocated(CACHE2), v);
            v
        };
        wrf_at(base, 0x1c, cc);
        // Bumped angle (also written back to the a2 slot on the original;
        // the stack check is off for that slot, the value is observed
        // through both helpers' logged xmm0).
        let a2b = add(f32::from_bits(a2), grdf(PI2));
        let neg_sin = f32::from_bits(
            lf_checker_rt::callee_cdecl!(S, u32, a2b.to_bits()) ^ SIGN,
        );
        wrf_at(base, 0x00, neg_sin);
        let cos = f32::from_bits(lf_checker_rt::callee_cdecl!(C, u32, a2b.to_bits()));
        let mut x2 = rdf_at(base, 0x00);
        let mut x1 = cos;
        let k02 = grdf(K02);
        let mut x3 = rdf(a1 + 8);
        wrf_at(base, 0x4c, x1);
        x2 = mul(x2, k02);
        x1 = mul(x1, k02);
        x2 = add(x2, rdf(a1));
        x3 = add(x3, k02);
        x1 = add(x1, rdf(a1 + 4));
        wrf_at(base, 0x28, x3);
        x3 = sub(x3, cc);
        wrf_at(base, 0x20, x2);
        wrf_at(base, 0x24, x1);
        wrf_at(base, 0x30, x2);
        wrf_at(base, 0x34, x1);
        wrf_at(base, 0x38, x3);
        let (vx, vy, vz) = (grdf(VEC[0]), grdf(VEC[1]), grdf(VEC[2]));
        wrf_at(base, 0x60, vx);
        wrf_at(base, 0x64, vy);
        wrf_at(base, 0x68, vz);
        wrf_at(base, 0x70, vx);
        wrf_at(base, 0x74, vy);
        wrf_at(base, 0x78, vz);
        wrf_at(base, 0x80, vx);
        wrf_at(base, 0x84, vy);
        wrf_at(base, 0x88, vz);
        wr32_at(base, 0x9c, 0xffff);
        // +0x50,+0x90,+0x94,+0x98,+0xa0,+0xa2 stay zero (memset).
        let p1: u32 = lf_checker_rt::callee_cdecl!(
            P1, u32, a0, base + 0x20, base + 0x30, base + 0x50, F6, 0, 0, 0x8e, 0
        );
        if p1 & 0xff == 0 {
            return p1;
        }
        let mut y1 = rdf_at(base, 0x00);
        let mut y2 = rdf_at(base, 0x4c);
        let k022 = grdf(K022);
        let kn2 = grdf(KN2);
        y1 = mul(y1, k022);
        y2 = mul(y2, k022);
        y1 = mul(y1, kn2);
        y2 = mul(y2, kn2);
        let mut y0 = y1;
        y0 = add(y0, rdf_at(base, 0x20));
        y1 = add(y1, rdf_at(base, 0x30));
        wrf_at(base, 0x20, y0);
        y0 = y2;
        y0 = add(y0, rdf_at(base, 0x24));
        y2 = add(y2, rdf_at(base, 0x34));
        wrf_at(base, 0x24, y0);
        wrf_at(base, 0x30, y1);
        wrf_at(base, 0x34, y2);
        let p2: u32 = lf_checker_rt::callee_cdecl!(
            P2, u32, a0, base + 0x10, base + 0x30, base + 0x50, F6, 0, 0, 0x8e, 0
        );
        if p2 & 0xff != 0 {
            (p2 & !0xff) | 1
        } else {
            p2 & !0xff
        }
    }
});
