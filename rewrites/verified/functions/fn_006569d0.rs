// original: 0x006569D0 rage::spdShaft::vf4

/// Virtual method 4 of `rage::spdShaft`: advance one shaft-dynamics step for
/// the timestep `arg` on the fourth channel.
///
/// `this` is the shaft object and `arg` the step length in seconds. The object
/// holds calibration floats (`+0x130`, `+0x138`/`+0x13C`, `+0x148`/`+0x14C`),
/// a 4x4 table at `+0xE0`..`+0x118`, scratch state, and two flag bytes
/// (`+0x2C0`, `+0x2C2`). The step stores `arg` at `+0x18`, scales it by the calibration (`arg / m14C *
/// (2 * m13C) - (2 * m13C) * 0.5`)
/// and then branches on the flag at `+0x2C2`: when set, the seeds (1, 0, 0) are used directly; otherwise
/// a length `len = x*x + y*y + z*z` is formed and, unless it is exactly
/// zero (then the factor is 0), its square root is taken through the helper
/// and the seeds are the root-normalised direction scaled by `1 / root`.
/// (Zero is decided the way the original's `ucomiss`/`lahf`/`test`/`jp`
/// sequence decides it: the square-root path runs for any non-zero length,
/// a NaN included, so a plain `== 0.0` test matches.) The tail combines the
/// seeds with the table into the scratch state (seeds at `+0x190`/`+0x194`/`+0x198`/`+0x19C` and `+0x1F0`..`+0x1FC`),
/// copies words between the scratch areas, clears the flag at `+0x2C0` and
/// returns the word at the second seed slot. The helper takes the radicand on
/// the stack and answers in st0; the original converts the answer to float at
/// once, so the stub answers a float directly (every observable intermediate
/// is a float, so nothing is lost).
///
/// Original: 0x006569D0 (thiscall, one float stack word).
lf_checker_rt::export!(thiscall, rw_006569D0(this: u32, arg: u32) -> u32 {
    unsafe {
        const SIGN: u32 = 0x8000_0000;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
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
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
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
        fn div(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) / core::hint::black_box(b)
        }
        #[inline(always)]
        fn neg(a: f32) -> f32 {
            f32::from_bits(a.to_bits() ^ SIGN)
        }

        let esi = this;
        let mut argval = f32::from_bits(arg);
        let two = f32::from_bits(*lf_checker_rt::global::<u32>(0xfe8a24));
        let half = f32::from_bits(*lf_checker_rt::global::<u32>(0xfe8830));
        let one = f32::from_bits(*lf_checker_rt::global::<u32>(0xfe88e8));
        let neg_one = f32::from_bits(*lf_checker_rt::global::<u32>(0xfe8d94));
        let mut x0: f32;
        let mut x1: f32;
        let mut x2: f32;
        let mut x3: f32;
        let mut x4: f32;
        let mut x5: f32;
        let mut x6: f32;
        let mut x7: f32;
        let mut e_p10: f32;
        let mut e_p28: f32;
        let mut e_p30: f32;
        let mut e_p40: f32;
        let mut e_p50: f32;
        let mut e_p6c: f32;
        let mut e_p70: f32;
        let mut e_p7c: f32;
        let mut e_p80: f32;
        let mut a: u32;

        x1 = argval;
        x4 = 0.0;
        let flag = rd8(esi + 0x2c2);
        x7 = rdf(esi + 0x130);
        wrf(esi + 0x18, x1);
        x1 = div(x1, rdf(esi + 0x14c));
        x0 = x7;
        x0 = mul(x0, two);
        x7 = neg(x7);
        e_p50 = x7;
        x0 = mul(x0, rdf(esi + 0x13c));
        x1 = mul(x1, x0);
        x0 = mul(x0, half);
        x1 = sub(x1, x0);
        x1 = neg(x1);
        argval = x1;
        if flag == 0 {
        x5 = x1;
        x0 = x7;
        x0 = mul(x0, x4);
        x5 = mul(x5, x4);
        x3 = x7;
        x3 = mul(x3, neg_one);
        x5 = sub(x5, x0);
        x6 = x1;
        x2 = x3;
        x2 = mul(x2, x3);
        x0 = x5;
        x0 = mul(x0, x5);
        e_p30 = x5;
        e_p28 = x3;
        x2 = add(x2, x0);
        x0 = x6;
        x0 = mul(x0, x6);
        e_p40 = x6;
        x2 = add(x2, x0);
            if x2 == 0.0 {
            x0 = x4;
            } else {
            let sqrt_f = f32::from_bits(lf_checker_rt::callee_cdecl!(1, u32, x2.to_bits()));
            e_p10 = sqrt_f;
            x0 = one;
            x0 = div(x0, e_p10);
            x1 = argval;
            x3 = e_p28;
            x5 = e_p30;
            x6 = e_p40;
            x7 = e_p50;
            x4 = 0.0;
            }
        x2 = x0;
        x2 = mul(x2, x5);
        x3 = mul(x3, x0);
        x6 = mul(x6, x0);
        } else {
        x3 = neg_one;
        x2 = x4;
        x6 = x4;
        }
        wrf(esi + 0x194, x3);
        wrf(esi + 0x198, x6);
        x0 = x2;
        x0 = mul(x0, x4);
        wrf(esi + 0x190, x2);
        x3 = mul(x3, x1);
        x6 = mul(x6, x7);
        x3 = add(x3, x0);
        e_p10 = x2;
        x3 = add(x3, x6);
        wrf(esi + 0x19c, x3);
        wrf(esi + 0x1f0, x2);
        a = rd32(esi + 0x194);
        wr32(esi + 0x1f4, a);
        a = rd32(esi + 0x198);
        x3 = rdf(esi + 0x1f4);
        wr32(esi + 0x1f8, a);
        x0 = rdf(esi + 0x19c);
        x2 = rdf(esi + 0x1f8);
        wrf(esi + 0x1fc, x0);
        x1 = x0;
        x1 = mul(x1, e_p10);
        x7 = x2;
        x7 = mul(x7, x0);
        e_p28 = x2;
        x2 = rdf(esi + 0xe0);
        e_p40 = x2;
        x6 = x3;
        x6 = mul(x6, x0);
        x0 = rdf(esi + 0xf0);
        x2 = mul(x2, x1);
        x0 = mul(x0, x6);
        x4 = rdf(esi + 0xf4);
        x5 = rdf(esi + 0xe8);
        x2 = add(x2, x0);
        x0 = rdf(esi + 0x100);
        x0 = mul(x0, x7);
        e_p30 = x3;
        x3 = rdf(esi + 0x104);
        x2 = add(x2, x0);
        x0 = x4;
        x0 = mul(x0, x6);
        x4 = mul(x4, rdf(esi + 0x1f4));
        x2 = add(x2, rdf(esi + 0x110));
        e_p70 = x5;
        e_p50 = x1;
        x5 = mul(x5, e_p50);
        e_p80 = x2;
        x2 = rdf(esi + 0xe4);
        e_p6c = x2;
        x2 = mul(x2, x1);
        x1 = rdf(esi + 0x108);
        x2 = add(x2, x0);
        x0 = x3;
        x0 = mul(x0, x7);
        x2 = add(x2, x0);
        x2 = add(x2, rdf(esi + 0x114));
        e_p7c = x2;
        x2 = rdf(esi + 0xf8);
        x0 = x2;
        x0 = mul(x0, x6);
        x6 = e_p6c;
        x6 = mul(x6, e_p10);
        x5 = add(x5, x0);
        x0 = x1;
        x0 = mul(x0, x7);
        x7 = e_p40;
        x7 = mul(x7, e_p10);
        x5 = add(x5, x0);
        x0 = rdf(esi + 0xf0);
        x0 = mul(x0, e_p30);
        x6 = add(x6, x4);
        x4 = rdf(esi + 0x1f8);
        x5 = add(x5, rdf(esi + 0x118));
        x7 = add(x7, x0);
        x0 = rdf(esi + 0x100);
        x0 = mul(x0, e_p28);
        x3 = mul(x3, x4);
        x7 = add(x7, x0);
        x0 = e_p70;
        x6 = add(x6, x3);
        x2 = mul(x2, rdf(esi + 0x1f4));
        x0 = mul(x0, e_p10);
        x1 = mul(x1, x4);
        x0 = add(x0, x2);
        wrf(esi + 0x1f4, x6);
        x6 = mul(x6, e_p7c);
        wrf(esi + 0x1f0, x7);
        x7 = mul(x7, e_p80);
        x0 = add(x0, x1);
        x6 = add(x6, x7);
        wrf(esi + 0x1f8, x0);
        x0 = mul(x0, x5);
        x6 = add(x6, x0);
        wrf(esi + 0x1fc, x6);
        wr8(esi + 0x2c0, 0);
        a
    }
});
