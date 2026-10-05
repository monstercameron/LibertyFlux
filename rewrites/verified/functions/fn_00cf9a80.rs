// original: 0x00CF9A80 ladder_probe_steps (proposed)
//
// Probes up to eight steps along the ladder direction, accumulating the
// sine/cosine advance into two point pairs until the step validator declines.
//
// Arguments (thiscall, ecx=this, no stack words): `this` (task object; angle
// f32 at +0x50, base point (x,z) at +0x70/+0x74, rail offset at +0x98).
//
// Behaviour: callees 1 and 2 are the xmm0-in/xmm0-out cosine/sine pair (v3
// transports) of the angle; the cosine is negated. Two accumulators start at
// the base point and grow by -cos*0.125 in x and sin*0.125 in z per accepted
// step; two constants hold rail+1.0 and rail+2.0 (two adds, not one). Each
// iteration calls callee 3 (cdecl/8, ecx ignored) with the two triples and
// constants (0.125, 0, 0x8e, 0, 0, 0); a nonzero answer returns 0 at once,
// otherwise the accumulators advance. Eight accepted steps return 1.
// Float order is the original's, pinned with black_box.
lf_checker_rt::export!(thiscall, rw_00CF9A80(this: u32) -> u32 {
    unsafe {
        const ANGLE_OFF: u32 = 0x50;
        const X_OFF: u32 = 0x70;
        const Z_OFF: u32 = 0x74;
        const RAIL_OFF: u32 = 0x98;
        const ONE_ADDR: u32 = 0x00FE88E8;
        const STEP_ADDR: u32 = 0x00FE87A4;
        const NEG_MASK_ADDR: u32 = 0x00FE8FA0;

        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits((a as *const u32).read_unaligned()) }
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }

        let one = f32::from_bits((lf_checker_rt::global::<u32>(ONE_ADDR) as *const u32).read_unaligned());
        let step = f32::from_bits((lf_checker_rt::global::<u32>(STEP_ADDR) as *const u32).read_unaligned());
        let neg_mask = (lf_checker_rt::global::<u32>(NEG_MASK_ADDR) as *const u32).read_unaligned();

        let mut t0 = [0u32; 3];
        let mut t1 = [0u32; 3];
        t0[0] = rdf(this + X_OFF).to_bits();
        t1[0] = t0[0];
        t0[1] = rdf(this + Z_OFF).to_bits();
        t1[1] = t0[1];
        let r1 = add(rdf(this + RAIL_OFF), one);
        t0[2] = r1.to_bits();
        t1[2] = add(r1, one).to_bits();

        let angle = rdf(this + ANGLE_OFF);
        let cos = f32::from_bits(lf_checker_rt::callee_cdecl!(1, u32, angle.to_bits()));
        let neg_cos = f32::from_bits(cos.to_bits() ^ neg_mask);
        let sin = f32::from_bits(lf_checker_rt::callee_cdecl!(2, u32, angle.to_bits()));

        for _ in 0..8u32 {
            let ans = lf_checker_rt::callee_cdecl!(
                3, u32, t0.as_mut_ptr() as u32, t1.as_mut_ptr() as u32,
                0x3E00_0000, 0, 0x8e, 0, 0, 0
            );
            if ans != 0 {
                return 0;
            }
            let d1 = mul(neg_cos, step);
            let d2 = mul(sin, step);
            t0[0] = add(d1, f32::from_bits(t0[0])).to_bits();
            t1[0] = add(d1, f32::from_bits(t1[0])).to_bits();
            t0[1] = add(d2, f32::from_bits(t0[1])).to_bits();
            t1[1] = add(d2, f32::from_bits(t1[1])).to_bits();
        }
        1
    }
});
