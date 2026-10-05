// original: 0x00CF97D0 ladder_mount_pose (proposed)
//
// Builds the mount pose for a ladder climb: rotates the caller struct's basis
// by the sine/cosine of the angle at +0x50, blends it into the out struct,
// then submits a descriptor block to the mount validator.
//
// Arguments (thiscall, ecx=this plus two stack words): `this` (task object;
// angle f32 at +0x50, floats at +0xb0/+0xb4, int at +0x38), `a8` (opaque word
// forwarded to callee 4), `out` (12-byte out struct).
//
// Behaviour: callee 1 (thiscall/4 with caller cleanup, so `noclean`) answers
// three floats v0/v1/v2 for (outptr, 1.0, 9, 0x86); callees 2 and 3 are the
// xmm0-in/xmm0-out sine/cosine pair (v3 transports) of the angle. Two blends
// are formed, b1 = -cos*v1 + sin*v0 + v2*0 and b2 = v1*sin + cos*v0 + v2*0,
// plus m = (v1+v0)*0 + v2 (the *0 terms are real mulss, kept for NaN/Inf).
// The out struct takes [e0+b1, e4+b2, e8+m] where e0/e4/e8 are the int bits
// at +0xb0/+0xb4/+0x38 read as floats. A descriptor is then laid out (two
// copies of the blended pair, m+0.5, m-2.5, zeros, the three globals at
// 0x1B4B320/324/328 fanned out, tail constants) and passed as three
// overlapping pointers to callee 4 (cdecl/9, ecx ignored); when it answers
// nonzero, its out word (arg0+0x38) refreshes out+8. Returns callee 4's
// answer. One dead uninitialized slot ([E+0x3c] into dead [E+0x2c]) is
// intentionally not modelled. Float order is the original's, pinned with
// black_box; negation is a bit xor with the sign mask, matching xorps.
lf_checker_rt::export!(thiscall, rw_00CF97D0(this: u32, a8: u32, out: u32) -> u32 {
    unsafe {
        const ANGLE_OFF: u32 = 0x50;
        const E0_OFF: u32 = 0xb0;
        const E4_OFF: u32 = 0xb4;
        const E8_OFF: u32 = 0x38;
        const NEG_MASK_ADDR: u32 = 0x00FE8FA0;
        const HALF_ADDR: u32 = 0x00FE8830;
        const C25_ADDR: u32 = 0x00FE8A60;
        const G320_ADDR: u32 = 0x01B4B320;
        const G324_ADDR: u32 = 0x01B4B324;
        const G328_ADDR: u32 = 0x01B4B328;

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

        let neg_mask = (lf_checker_rt::global::<u32>(NEG_MASK_ADDR) as *const u32).read_unaligned();
        let half = f32::from_bits((lf_checker_rt::global::<u32>(HALF_ADDR) as *const u32).read_unaligned());
        let c25 = f32::from_bits((lf_checker_rt::global::<u32>(C25_ADDR) as *const u32).read_unaligned());
        let g320 = f32::from_bits((lf_checker_rt::global::<u32>(G320_ADDR) as *const u32).read_unaligned());
        let g324 = f32::from_bits((lf_checker_rt::global::<u32>(G324_ADDR) as *const u32).read_unaligned());
        let g328 = f32::from_bits((lf_checker_rt::global::<u32>(G328_ADDR) as *const u32).read_unaligned());

        let mut vv = [0u32; 3];
        lf_checker_rt::callee_thiscall!(1, u32, this, vv.as_mut_ptr() as u32, 0x3F80_0000, 9, 0x86);
        let v0 = f32::from_bits(vv[0]);
        let v1 = f32::from_bits(vv[1]);
        let v2 = f32::from_bits(vv[2]);

        let angle = rdf(this + ANGLE_OFF);
        let s = f32::from_bits(lf_checker_rt::callee_cdecl!(2, u32, angle.to_bits()));
        let c = f32::from_bits(lf_checker_rt::callee_cdecl!(3, u32, angle.to_bits()));

        let zero = 0.0f32;
        let neg_c = f32::from_bits(c.to_bits() ^ neg_mask);
        let v2t0 = mul(v2, zero);
        let b1 = add(add(mul(neg_c, v1), mul(s, v0)), v2t0);
        let b2 = add(add(mul(v1, s), mul(c, v0)), v2t0);
        let m = add(mul(add(v1, v0), zero), v2);

        let e0 = rd32(this + E0_OFF);
        let e4 = rd32(this + E4_OFF);
        let e8 = rd32(this + E8_OFF);
        wr32(out, e0);
        wr32(out + 4, e4);
        wr32(out + 8, e8);
        let f1 = add(f32::from_bits(e0), b1);
        let f2 = add(f32::from_bits(e8), m);
        let f0 = add(f32::from_bits(e4), b2);
        wrf(out + 8, f2);
        wrf(out, f1);
        wrf(out + 4, f0);

        // Descriptor block mirroring E+0x30..E+0xA3; gaps left zero (never snapped).
        let mut st = [0u32; 29];
        st[0] = f1.to_bits();
        st[1] = f0.to_bits();
        st[2] = add(f2, half).to_bits();
        st[4] = f1.to_bits();
        st[5] = f0.to_bits();
        st[6] = sub(f2, c25).to_bits();
        st[8] = 0;
        st[12] = g320.to_bits();
        st[13] = g324.to_bits();
        st[14] = g328.to_bits();
        st[16] = g320.to_bits();
        st[17] = g324.to_bits();
        st[18] = g328.to_bits();
        st[20] = g320.to_bits();
        st[21] = g324.to_bits();
        st[22] = g328.to_bits();
        st[24] = 0;
        st[25] = 0;
        st[26] = 0;
        st[27] = 0xffff;
        let p0 = st.as_mut_ptr() as u32;
        let ans = lf_checker_rt::callee_cdecl!(
            4, u32, p0, p0 + 0x10, 0x3DCC_CCCD, a8, p0 + 0x20, 0x8e, 0xFFFF_FFFF, 1, 0
        );
        if (ans & 0xFF) != 0 {
            wr32(out + 8, st[14]);
        }
        ans
    }
});
