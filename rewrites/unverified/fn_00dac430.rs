// original: 0x00dac430 hassle_ped_path_probe (proposed)
//
// cdecl/3 (world, pos, angle): probe twice through the shared 9-word probe
// callee, returning 1 when both probes answer false, else 0 with the last
// answer's residue in the upper bits.
//
// `pos` points at three floats; `angle` feeds a sine/cosine helper pair
// (helpers S then C, argument in xmm0, float result in xmm0, both scripted).
// The frame holds one zeroed 0xa4-byte work region (offsets below are from
// its base): a header triple of the shared .data vector at +0x54..0x7c,
// flag words at +0x84..0x96, the sine spill at +0x00 and the probe inputs
// around it. Probe 1 passes sub-views at +0x24/+0x04/+0x44 with constants
// (0, 0x8e, 1, 1, 0.02); when it answers false, probe 2 rebuilds the lower
// inputs and passes +0x04/+0x34/+0x44 with the same constants. The float
// operation order is the original's SSE order.
lf_checker_rt::export!(cdecl, rw_00dac430(a0: u32, a1: u32, a2: u32) -> u32 {
    unsafe {
        const VEC: [u32; 3] = [0x001b4b320, 0x001b4b324, 0x001b4b328];
        const K1: u32 = 0x00e90944; // -0.025
        const K2: u32 = 0x00fe8740; // 0.025
        const F1: u32 = 0x3c23d70a; // probe constant
        const S: u32 = 1;
        const C: u32 = 2;
        const P1: u32 = 3;
        const P2: u32 = 4;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
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

        let mut fr = [0u8; 0xa4];
        let base = fr.as_mut_ptr() as u32;
        #[inline(always)]
        unsafe fn wrf(base: u32, at: u32, v: f32) {
            unsafe { ((base + at) as *mut u32).write_unaligned(v.to_bits()) }
        }
        #[inline(always)]
        unsafe fn rdf_at(base: u32, at: u32) -> f32 {
            unsafe { f32::from_bits(((base + at) as *const u32).read_unaligned()) }
        }
        #[inline(always)]
        unsafe fn wr32(base: u32, at: u32, v: u32) {
            unsafe { ((base + at) as *mut u32).write_unaligned(v) }
        }

        // Header: shared vector triple + flags (fresh zeros elsewhere).
        let (vx, vy, vz) = (grdf(VEC[0]), grdf(VEC[1]), grdf(VEC[2]));
        wrf(base, 0x54, vx);
        wrf(base, 0x58, vy);
        wrf(base, 0x5c, vz);
        wrf(base, 0x64, vx);
        wrf(base, 0x68, vy);
        wrf(base, 0x6c, vz);
        wrf(base, 0x74, vx);
        wrf(base, 0x78, vy);
        wrf(base, 0x7c, vz);
        wr32(base, 0x90, 0xffff);
        // +0x84,+0x88,+0x8c,+0x94,+0x96 stay zero (memset).

        let sin = f32::from_bits(lf_checker_rt::callee_cdecl!(S, u32, a2));
        wrf(base, 0x00, sin);
        let cos = f32::from_bits(lf_checker_rt::callee_cdecl!(C, u32, a2));
        let mut x1 = rdf_at(base, 0x00);
        let mut x4 = rdf(a1 + 8);
        let mut x2 = rdf(a1);
        let mut x3 = rdf(a1 + 4);
        x1 = mul(x1, grdf(K1));
        let mut x5 = cos;
        let k2 = grdf(K2);
        x4 = add(x4, k2);
        x5 = mul(x5, k2);
        x2 = sub(x2, x1);
        x3 = sub(x3, x5);
        wrf(base, 0x0c, x4);
        x4 = sub(x4, k2);
        wrf(base, 0x04, x2);
        wrf(base, 0x20, x5);
        wrf(base, 0x08, x3);
        wrf(base, 0x24, x2);
        wrf(base, 0x28, x3);
        wrf(base, 0x2c, x4);
        let p1: u32 = lf_checker_rt::callee_cdecl!(
            P1, u32, a0, base + 0x24, base + 0x04, base + 0x44, F1, 1, 1, 0x8e, 0
        );
        if p1 & 0xff != 0 {
            return p1 & !0xff;
        }
        let mut y1 = rdf_at(base, 0x00);
        let mut y0 = rdf_at(base, 0x0c);
        y1 = mul(y1, k2);
        wrf(base, 0x3c, y0);
        y0 = sub(rdf_at(base, 0x04), y1);
        wrf(base, 0x34, y0);
        y0 = add(rdf_at(base, 0x20), rdf_at(base, 0x08));
        wrf(base, 0x38, y0);
        let p2: u32 = lf_checker_rt::callee_cdecl!(
            P2, u32, a0, base + 0x04, base + 0x34, base + 0x44, F1, 1, 1, 0x8e, 0
        );
        if p2 & 0xff != 0 {
            return p2 & !0xff;
        }
        (p2 & !0xff) | 1
    }
});
