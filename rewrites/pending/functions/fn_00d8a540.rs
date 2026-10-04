// original: 0x00d8a540 ui_aim_angle_clamp (proposed)

/// Clamp two aim angles toward a target direction, unless an override object
/// matches.
///
/// `a` is the actor object (matrix pointer at `+0x20`, fallback position at
/// `+0x10`), `b` is the aim-state object (vtable at `+0`, matrix pointer at
/// `+0x20`, mode byte at `+0xe6e`, flag byte at `+0xf1f`), `p1` and `p0` point
/// to the two angle floats to update.
///
/// Gates: when the mode byte is 2, callee 0 is asked for the override object
/// and nothing happens if it equals `a`; when the mode byte is 6, callee 1 is
/// asked the same question about the sub-object at `b+0xe48`. Otherwise the
/// virtual slot at `+0x64` of `b` gives a parameter block whose second float,
/// scaled by 0.9, seeds an input triple passed with a frame out-buffer and
/// the matrix pointer to callee 3; the two out floats are subtracted from the
/// actor position (or the fallback when the actor has no matrix) and the
/// resulting pair goes through callee 4, whose x87 return is kept. The pair's
/// length must reach 1.0 or nothing happens.
///
/// Then a magnitude `m` is formed from the first float of a second parameter
/// block (`* 2.4 + 0.8`, divided by the length, halved), and the x87 value
/// minus each stored angle is wrapped to `[-pi, pi]` and made non-negative.
/// If `m` exceeds the wrapped difference for `p0`, `p0` is set to the x87
/// value minus `m` (wrapped upward), callee 5 runs, and its low byte decides
/// whether flag bit `0x10` is set on `b`. The same comparison for `p1` stores
/// `m` plus the x87 value (wrapped downward) and runs callee 5 again.
///
/// Float order is the original's; comparisons follow `comiss` semantics
/// (unordered counts as not-greater). Callee 4 returns its float on the x87
/// stack. Original: 0x00d8a540 (cdecl, four stack words, no return value).
lf_checker_rt::export!(cdecl, rw_00d8a540(a: u32, b: u32, p1: u32, p0: u32) -> u32 {
    unsafe {
        const MODE: u32 = 0xe6e;
        const SUB: u32 = 0xe48;
        const FLAG: u32 = 0xf1f;
        const FLAG_BIT: u8 = 0x10;
        const MATRIX: u32 = 0x20;
        const POS_FALLBACK: u32 = 0x10;
        const VT_SLOT: u32 = 0x64;
        const C_SCALE: u32 = 0x00fe88bc; // 0.9
        const C_ONE: u32 = 0x00fe88e8; // 1.0
        const C_GAIN: u32 = 0x00fe8a58; // 2.4
        const C_BIAS: u32 = 0x00fe8898; // 0.8
        const C_NEG_PI: u32 = 0x00fe8dc4;
        const C_TWO_PI: u32 = 0x00fe8aec;
        const C_PI: u32 = 0x00fe8aa0;
        const C_HALF: u32 = 0x00fe8830; // 0.5
        const SIGN: u32 = 0x8000_0000;

        #[inline(always)]
        unsafe fn rd32(x: u32) -> u32 {
            unsafe { (x as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(x: u32) -> u8 {
            unsafe { (x as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn rdf(x: u32) -> f32 {
            unsafe { f32::from_bits(rd32(x)) }
        }
        #[inline(always)]
        unsafe fn wr8(x: u32, v: u8) {
            unsafe { (x as *mut u8).write(v) }
        }
        #[inline(always)]
        unsafe fn wrf(x: u32, v: f32) {
            unsafe { (x as *mut u32).write_unaligned(v.to_bits()) }
        }
        #[inline(always)]
        unsafe fn glob(va: u32) -> f32 {
            unsafe { f32::from_bits(rd32(lf_checker_rt::relocated(va))) }
        }
        #[inline(always)]
        fn mul(x: f32, y: f32) -> f32 {
            core::hint::black_box(x) * core::hint::black_box(y)
        }
        #[inline(always)]
        fn add(x: f32, y: f32) -> f32 {
            core::hint::black_box(x) + core::hint::black_box(y)
        }
        #[inline(always)]
        fn sub(x: f32, y: f32) -> f32 {
            core::hint::black_box(x) - core::hint::black_box(y)
        }
        #[inline(always)]
        fn div(x: f32, y: f32) -> f32 {
            core::hint::black_box(x) / core::hint::black_box(y)
        }
        #[inline(always)]
        fn abs_neg(x: f32) -> f32 {
            f32::from_bits(x.to_bits() ^ SIGN)
        }
        #[inline(always)]
        unsafe fn params(obj: u32) -> u32 {
            unsafe {
                let slot: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(rd32(rd32(obj) + VT_SLOT) as usize);
                slot(obj)
            }
        }
        #[inline(always)]
        unsafe fn wrap(mut x: f32) -> f32 {
            unsafe {
                while x < glob(C_NEG_PI) {
                    x = add(x, glob(C_TWO_PI));
                }
                while x > glob(C_PI) {
                    x = sub(x, glob(C_TWO_PI));
                }
                if x < 0.0 {
                    x = abs_neg(x);
                }
                x
            }
        }
        #[inline(always)]
        unsafe fn maybe_flag(ret: u32, obj: u32) {
            unsafe {
                if ret as u8 != 0 {
                    wr8(obj + FLAG, rd8(obj + FLAG) | FLAG_BIT);
                }
            }
        }

        if rd8(b + MODE) == 2 {
            let t: u32 = lf_checker_rt::callee_cdecl!(0, u32, 0u32);
            if t == a {
                return 0;
            }
        }
        if rd8(b + MODE) == 6 {
            let t: u32 = lf_checker_rt::callee_thiscall!(1, u32, b.wrapping_add(SUB));
            if t == a {
                return 0;
            }
        }
        let seed = mul(rdf(params(b).wrapping_add(4)), glob(C_SCALE));
        let mut out = [0u32; 2];
        let inp = [0u32, seed.to_bits(), 0u32];
        lf_checker_rt::callee_cdecl!(
            3,
            u32,
            out.as_mut_ptr() as u32,
            rd32(b + MATRIX),
            inp.as_ptr() as u32
        );
        let (out0, out1) = (f32::from_bits(out[0]), f32::from_bits(out[1]));
        let mat = rd32(a + MATRIX);
        let px = if mat != 0 { mat.wrapping_add(0x30) } else { a.wrapping_add(POS_FALLBACK) };
        let dx = sub(rdf(px), out0);
        let qx = if mat != 0 { mat.wrapping_add(0x30) } else { a.wrapping_add(POS_FALLBACK) };
        let dy = sub(rdf(qx.wrapping_add(4)), out1);
        let kept: f32 = lf_checker_rt::callee_cdecl!(4, f32, dx.to_bits(), dy.to_bits());
        let len = add(mul(dx, dx), mul(dy, dy)).sqrt();
        if glob(C_ONE) > len {
            return 0;
        }
        let mut m = mul(rdf(params(b)), glob(C_GAIN));
        m = add(m, glob(C_BIAS));
        m = div(m, len);
        let d0 = wrap(sub(kept, rdf(p0)));
        m = mul(m, glob(C_HALF));
        if m > d0 {
            let mut v = sub(kept, m);
            wrf(p0, v);
            while v < glob(C_NEG_PI) {
                v = add(v, glob(C_TWO_PI));
            }
            wrf(p0, v);
            let t: u32 = lf_checker_rt::callee_cdecl!(5, u32, a);
            maybe_flag(t, b);
        }
        let d1 = wrap(sub(kept, rdf(p1)));
        if !(m > d1) {
            return 0;
        }
        let mut v = add(m, kept);
        wrf(p1, v);
        while v > glob(C_PI) {
            v = sub(v, glob(C_TWO_PI));
        }
        wrf(p1, v);
        let t: u32 = lf_checker_rt::callee_cdecl!(5, u32, a);
        maybe_flag(t, b);
        0
    }
});
