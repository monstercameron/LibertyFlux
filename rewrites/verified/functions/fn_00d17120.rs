// original: 0x00D17120 CTaskSimplePlayAnimAndSlideOutOfCover::vf13

/// Slide-out-of-cover task update: steer the ped toward its cover exit
/// point and scale its animation channel weights.
///
/// `this` is the task, `arg0` the ped. Returns 0 at once when the task's
/// done flag (+0x30) is set, when the ped's slide handle (+0xd68) is null,
/// or when the cover solver (callee 1) rejects the move. Otherwise the
/// planar offset between the task anchor (+0x20/+0x24) and the ped's exit
/// point (through +0x20, +0x30/+0x34) is normalized by 1/length into a unit
/// step (zero for a zero offset); the step feeds the solver and the blend
/// setup (callee 3), whose out-parameter and x87 result drive the rest: the
/// result is clamped to [1, 2] (NaN lands on 2) and scales the three
/// channel weights at +0xa90, while the out-parameter's absolute value goes
/// through a frameless float helper (callee 4, argument and result in
/// XMM0) and is folded with pi/2 (added for a negative input, subtracted
/// from it otherwise), scaled by 2/pi, clamped at 0 and stored at +0x50.
/// When the adjust flag (+0x54) is set the scaled channels are additionally
/// offset by the solver block's (callee 2) first three words times the
/// clamp factor. Sets the done flag and returns 1.
///
/// Original: 0x00D17120 (thiscall, two stack words of which only the first
/// is read, returns a byte in al).
lf_checker_rt::export!(thiscall, rw_00D17120(this: u32, arg0: u32, _arg1: u32) -> u32 {
    unsafe {
        const C_ONE: u32 = 0xfe88e8;
        const C_TWO: u32 = 0xfe8a24;
        const C_HALF_PI: u32 = 0xfe8978;
        const C_TWO_OVER_PI: u32 = 0xe98888;
        const CALL_SOLVER: u32 = 1;
        const CALL_SOLVER_BLOCK: u32 = 2;
        const CALL_BLEND: u32 = 3;
        const CALL_FLOAT_HELPER: u32 = 4;
        const SIGN_BIT: u32 = 0x8000_0000;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
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
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
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
        #[inline(always)]
        fn neg(a: f32) -> f32 {
            f32::from_bits(a.to_bits() ^ SIGN_BIT)
        }

        if rd8(this + 0x30) != 0 {
            return 0;
        }
        let exit = rd32(arg0 + 0x20);
        let dx = sub(rdf(this + 0x20), rdf(exit + 0x30));
        let dy = sub(rdf(this + 0x24), rdf(exit + 0x34));
        let len_sq = add(mul(dx, dx), mul(dy, dy));
        let one = f32::from_bits(rd32(lf_checker_rt::relocated(C_ONE)));
        let n = if len_sq != 0.0 { div(one, len_sq.sqrt()) } else { 0.0 };
        let dxu = mul(dx, n);
        let dyu = mul(n, dy);
        let z = mul(n, 0.0);
        let inner = rd32(arg0 + 0xd68);
        if inner == 0 {
            return 0;
        }
        let mut step = [dxu.to_bits(), dyu.to_bits(), z.to_bits()];
        let mut scratch = [0u32; 4];
        let ok: u32 = lf_checker_rt::callee_cdecl!(
            CALL_SOLVER,
            u32,
            inner,
            arg0,
            step.as_ptr() as u32,
            scratch.as_ptr() as u32,
            0
        );
        if (ok & 0xff) == 0 {
            return 0;
        }
        let mut scratch2 = [0u32; 4];
        let blk: u32 = lf_checker_rt::callee_thiscall!(
            CALL_SOLVER_BLOCK,
            u32,
            inner,
            scratch2.as_ptr() as u32,
            step.as_ptr() as u32
        );
        let b0 = f32::from_bits(rd32(blk));
        let b1 = f32::from_bits(rd32(blk + 4));
        let b2 = f32::from_bits(rd32(blk + 8));
        let _b3 = rd32(blk + 12);
        let flagobj = rd32(this + 4);
        let bit = ((rd8(flagobj + 0x35) >> 1) & 1) as u32;
        let mut block_words = [b0.to_bits(), b1.to_bits(), b2.to_bits()];
        let mut step2 = [dxu.to_bits(), dyu.to_bits(), z.to_bits()];
        let mut fx = [0u32; 1];
        let t_raw: f32 = lf_checker_rt::callee_cdecl!(
            CALL_BLEND,
            f32,
            bit,
            block_words.as_ptr() as u32,
            step2.as_ptr() as u32,
            fx.as_mut_ptr() as u32,
            0
        );
        let mut t = t_raw;
        if t > one {
            t = add(sub(t, one), one);
        }
        let two = f32::from_bits(rd32(lf_checker_rt::relocated(C_TWO)));
        if !(two > t) {
            t = two;
        }
        if !(t > one) {
            t = one;
        }
        let fxc = f32::from_bits(fx[0]);
        let ax = if 0.0 > fxc { neg(fxc) } else { fxc };
        let r = f32::from_bits(lf_checker_rt::callee_cdecl!(CALL_FLOAT_HELPER, u32, ax.to_bits()));
        let half_pi = f32::from_bits(rd32(lf_checker_rt::relocated(C_HALF_PI)));
        let mut g = if 0.0 > fxc { add(r, half_pi) } else { sub(half_pi, r) };
        g = mul(g, f32::from_bits(rd32(lf_checker_rt::relocated(C_TWO_OVER_PI))));
        if !(g > 0.0) {
            g = 0.0;
        }
        wrf(this + 0x50, g);
        let s0 = mul(rdf(arg0 + 0xa90), t);
        let s1 = mul(rdf(arg0 + 0xa94), t);
        let s2 = mul(rdf(arg0 + 0xa98), t);
        wrf(arg0 + 0xa90, s0);
        wrf(arg0 + 0xa94, s1);
        wrf(arg0 + 0xa98, s2);
        if rd8(this + 0x54) != 0 {
            wrf(arg0 + 0xa90, sub(s0, mul(b0, t)));
            wrf(arg0 + 0xa94, sub(s1, mul(b1, t)));
            wrf(arg0 + 0xa98, sub(s2, mul(b2, t)));
        }
        wr8(this + 0x30, 1);
        1
    }
});
