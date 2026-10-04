// original: 0x00656340 rage::spdShaft::vf2

/// Virtual method 2 of `rage::spdShaft`: advance one shaft-dynamics step for
/// the timestep `arg`.
///
/// `this` is the shaft object and `arg` the step length in seconds. The object
/// holds calibration floats (`+0x130`, `+0x138`, `+0x148`), a 4x4 table at
/// `+0xe0..+0x118`, scratch state at `+0x170..+0x17c` and `+0x1d0..+0x1dc`,
/// and two flag bytes (`+0x2c0`, `+0x2c2`). The step stores `arg` at `+0x10`,
/// scales it by the calibration (`arg / m148 * (2 * m138) - (2 * m138) * 0.5`)
/// and then branches on the flag at `+0x2c2`: when set, three seed values are
/// used directly (1, 0, 0), skipping the scaling multiplies below; otherwise
/// a length `x3 = x2*x2 + x6*x6 + x5*x5` is formed and, unless it is exactly
/// zero (then the factor is 0), its square root is taken through the helper
/// and the seeds are the root-normalised direction scaled by `1 / root`.
/// The tail combines the seeds with the table
/// into the scratch state, copies words between the scratch areas, clears the
/// flag at `+0x2c0` and returns the word at `+0x178`. The helper takes the
/// radicand on the stack and answers in st0; the original converts the answer
/// to float at once, so the stub answers a float directly (every observable
/// intermediate is a float, so nothing is lost).
///
/// Original: 0x00656340 (thiscall, one float stack word).
lf_checker_rt::export!(thiscall, rw_00656340(this: u32, arg: u32) -> u32 {
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
        let mut e_p2c: f32;
        let mut e_p30: f32;
        let mut e_p40: f32;
        let mut e_p50: f32;
        let mut e_p5c: f32;
        let mut e_p60: f32;
        let mut e_p6c: f32;
        let mut e_p70: f32;
        let mut a: u32;
        x1 = argval;
        x4 = 0.0;
        let flag = rd8(esi + 0x2c2);
        x6 = rdf(esi + 0x130);
        wrf(esi + 0x10, x1);
        x1 = div(x1, rdf(esi + 0x148));
        x0 = x6;
        x0 = mul(x0, two);
        x6 = neg(x6);
        e_p40 = x6;
        x0 = mul(x0, rdf(esi + 0x138));
        x1 = mul(x1, x0);
        x0 = mul(x0, half);
        x1 = sub(x1, x0);
        argval = x1;
        if flag == 0 {
        x2 = x6;
        x0 = x1;
        x0 = mul(x0, x4);
        x2 = mul(x2, x4);
        x5 = x1;
        x5 = mul(x5, neg_one);
        x2 = sub(x2, x0);
        x0 = x6;
        x0 = mul(x0, x6);
        e_p30 = x5;
        x3 = x2;
        x3 = mul(x3, x2);
        e_p2c = x2;
        x3 = add(x3, x0);
        x0 = x5;
        x0 = mul(x0, x5);
        x3 = add(x3, x0);
            if x3 == 0.0 {
            x0 = x4;
            } else {
            let sqrt_f = f32::from_bits(lf_checker_rt::callee_cdecl!(1, u32, x3.to_bits()));
            e_p10 = sqrt_f;
            x0 = one;
            x0 = div(x0, e_p10);
            x1 = argval;
            x2 = e_p2c;
            x5 = e_p30;
            x6 = e_p40;
            x4 = 0.0;
            }
        x7 = x0;
        x7 = mul(x7, x6);
        x2 = mul(x2, x0);
        x5 = mul(x5, x0);
        } else {
        x7 = one;
        x2 = x4;
        x5 = x4;
        }
        wrf(esi + 0x174, x2);
        wrf(esi + 0x178, x5);
        x2 = mul(x2, x4);
        wrf(esi + 0x170, x7);
        x1 = mul(x1, x7);
        x5 = mul(x5, x6);
        x1 = add(x1, x2);
        e_p10 = x7;
        x1 = add(x1, x5);
        wrf(esi + 0x17c, x1);
        wrf(esi + 0x1d0, x7);
        a = rd32(esi + 0x174);
        wr32(esi + 0x1d4, a);
        a = rd32(esi + 0x178);
        x3 = rdf(esi + 0x1d4);
        wr32(esi + 0x1d8, a);
        x0 = rdf(esi + 0x17c);
        x2 = rdf(esi + 0x1d8);
        wrf(esi + 0x1dc, x0);
        x1 = x0;
        e_p50 = x2;
        x1 = mul(x1, x7);
        x7 = x2;
        x2 = rdf(esi + 0xe0);
        x7 = mul(x7, x0);
        e_p30 = x2;
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
        e_p2c = x3;
        x3 = rdf(esi + 0x104);
        x2 = add(x2, x0);
        x0 = x4;
        x0 = mul(x0, x6);
        x4 = mul(x4, rdf(esi + 0x1d4));
        x2 = add(x2, rdf(esi + 0x110));
        e_p60 = x5;
        e_p40 = x1;
        x5 = mul(x5, e_p40);
        e_p70 = x2;
        x2 = rdf(esi + 0xe4);
        e_p5c = x2;
        x2 = mul(x2, x1);
        x1 = rdf(esi + 0x108);
        x2 = add(x2, x0);
        x0 = x3;
        x0 = mul(x0, x7);
        x2 = add(x2, x0);
        x2 = add(x2, rdf(esi + 0x114));
        e_p6c = x2;
        x2 = rdf(esi + 0xf8);
        x0 = x2;
        x0 = mul(x0, x6);
        x6 = e_p5c;
        x6 = mul(x6, e_p10);
        x5 = add(x5, x0);
        x0 = x1;
        x0 = mul(x0, x7);
        x7 = e_p30;
        x7 = mul(x7, e_p10);
        x5 = add(x5, x0);
        x0 = rdf(esi + 0xf0);
        x0 = mul(x0, e_p2c);
        x6 = add(x6, x4);
        x4 = rdf(esi + 0x1d8);
        x5 = add(x5, rdf(esi + 0x118));
        x7 = add(x7, x0);
        x0 = rdf(esi + 0x100);
        x0 = mul(x0, e_p50);
        x3 = mul(x3, x4);
        x7 = add(x7, x0);
        x0 = e_p60;
        x0 = mul(x0, e_p10);
        x6 = add(x6, x3);
        x2 = mul(x2, rdf(esi + 0x1d4));
        x1 = mul(x1, x4);
        x0 = add(x0, x2);
        wrf(esi + 0x1d4, x6);
        x6 = mul(x6, e_p6c);
        wrf(esi + 0x1d0, x7);
        x7 = mul(x7, e_p70);
        x0 = add(x0, x1);
        x6 = add(x6, x7);
        wrf(esi + 0x1d8, x0);
        x0 = mul(x0, x5);
        x6 = add(x6, x0);
        wrf(esi + 0x1dc, x6);
        wr8(esi + 0x2c0, 0);
        a
    }
});
