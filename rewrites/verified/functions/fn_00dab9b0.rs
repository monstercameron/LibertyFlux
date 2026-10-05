// original: 0x00DAB9B0 ped_probe_two_pass (proposed)
//
// Two-pass probe check over a heading angle and a 3-float anchor.
//
// Arguments (cdecl, four stack words): `a0` is an opaque context pointer passed
// through to both probe calls; `a1` points to three floats (an anchor point);
// `a2` is a heading angle in radians; `a3` is a float passed through to both
// probe calls. Returns 1 in AL when the second probe answers zero after the
// first probe answered zero, else 0 (the upper bytes of EAX keep the last
// callee answer's upper bytes; scripts only use 0/1 so the result is 0/1).
//
// Behaviour: `s = sin(a2)*0.25`, `c = cos(a2)*0.25` (sine/cosine arrive via
// scripted callees taking the angle in XMM0). It builds a probe descriptor on
// the stack: three copies of the global direction vector (three floats in
// writable game data) plus header/footer words, and a five-word scratch block
// holding `s`, `c`, `s+anchor[0]`, `anchor[1]-c`, `anchor[2]-0.1`. The first
// probe call takes (context, scratch+8, a3, scratch+4, descriptor, 0.2f, 0,
// 0x8e, 0). If it answers nonzero the result is 0. Otherwise the scratch is
// updated (`s+t`, `d-c`) and the second probe runs with the same pointers; the
// result is 1 iff it answers zero. All float operations are in the original's
// operand order, pinned against reassociation.
lf_checker_rt::export!(cdecl, rw_00dab9b0(a0: u32, a1: u32, a2: u32, a3: u32) -> u32 {
    unsafe {
        const SIN_ID: u32 = 1;
        const COS_ID: u32 = 2;
        const PROBE1_ID: u32 = 3;
        const PROBE2_ID: u32 = 4;
        const GDIR_X: u32 = 0x01B4B320;
        const GDIR_Y: u32 = 0x01B4B324;
        const GDIR_Z: u32 = 0x01B4B328;
        const QUARTER: u32 = 0x00FE87E4;
        const TENTH: u32 = 0x00FE879C;
        const FIFTH_BITS: u32 = 0x3E4CCCCD;
        const PROBE_FLAGS: u32 = 0x8E;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
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

        let k_quarter = f32::from_bits(rd32(lf_checker_rt::relocated(QUARTER)));
        let k_tenth = f32::from_bits(rd32(lf_checker_rt::relocated(TENTH)));
        let gx = rd32(lf_checker_rt::relocated(GDIR_X));
        let gy = rd32(lf_checker_rt::relocated(GDIR_Y));
        let gz = rd32(lf_checker_rt::relocated(GDIR_Z));

        let anchor0 = f32::from_bits(rd32(a1));
        let anchor1 = f32::from_bits(rd32(a1 + 4));
        let anchor2 = f32::from_bits(rd32(a1 + 8));

        let sin_v = f32::from_bits(lf_checker_rt::callee_cdecl!(SIN_ID, u32, a2));
        let sin_k = mul(sin_v, k_quarter);
        let t_sum = add(sin_k, anchor0);
        let cos_v = f32::from_bits(lf_checker_rt::callee_cdecl!(COS_ID, u32, a2));
        let cos_k = mul(cos_v, k_quarter);
        let d1 = sub(anchor1, cos_k);
        let d2 = sub(anchor2, k_tenth);

        // Mirror of the original's frame: scratch words then descriptor.
        // Index = (original E0 offset) / 4; holes read as stack fill (0).
        let mut frame = [0u32; 29];
        frame[1] = sin_k.to_bits();
        frame[2] = cos_k.to_bits();
        frame[4] = t_sum.to_bits();
        frame[5] = d1.to_bits();
        frame[6] = d2.to_bits();
        frame[12] = gx;
        frame[13] = gy;
        frame[14] = gz;
        frame[16] = gx;
        frame[17] = gy;
        frame[18] = gz;
        frame[20] = gx;
        frame[21] = gy;
        frame[22] = gz;
        frame[27] = 0xFFFF;
        let p_sum = frame.as_mut_ptr().add(4) as u32;
        let p_hole = frame.as_mut_ptr().add(3) as u32;
        let p_desc = frame.as_mut_ptr().add(8) as u32;

        let r1: u32 = lf_checker_rt::callee_cdecl!(
            PROBE1_ID, u32, a0, p_sum, a3, p_hole, p_desc, FIFTH_BITS, 0, PROBE_FLAGS, 0
        );
        if r1 & 0xFF != 0 {
            return r1 & 0xFFFFFF00;
        }
        let s2 = add(sin_k, t_sum);
        frame[4] = s2.to_bits();
        let d1b = sub(d1, cos_k);
        frame[5] = d1b.to_bits();
        let r2: u32 = lf_checker_rt::callee_cdecl!(
            PROBE2_ID, u32, a0, p_sum, a3, p_hole, p_desc, FIFTH_BITS, 0, PROBE_FLAGS, 0
        );
        if r2 & 0xFF != 0 {
            r2 & 0xFFFFFF00
        } else {
            (r2 & 0xFFFFFF00) | 1
        }
    }
});
