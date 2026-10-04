// original: 0x00d82a30 steer_heading_and_gain (proposed)

/// Steer a heading toward a target point and derive a gain from a
/// provider-supplied 2D vector, writing a clamped heading delta, a gain, and
/// two zeroed-out slots.
///
/// `state` is the steering state: a vtable at `+0x00` whose slot `+0xec`
/// provides a 2D vector, a frame object at `+0x20`, a flag byte at `+0x2c`
/// (bit 0 selects the offset gain) and a limit byte at `+0xe6f`. The frame
/// holds a direction vector at `+0x10`/`+0x14` and a center point at
/// `+0x30`/`+0x34`. `target` is the target: when its word at `+0x20` is
/// non-null the aimed point is read at `+0x30`/`+0x34` of that object,
/// otherwise at `+0x10`/`+0x14` of the target itself. `out_angle`,
/// `out_gain`, `out_zero_dword` and `out_zero_byte` receive the results; the
/// return value is `out_zero_byte` unchanged.
///
/// Algorithm: the offset from the center to the aimed point is normalized
/// (a zero length yields a zero offset; a NaN length normalizes to NaN) and
/// scaled by -12.0 when the flag bit is set, else by 26.0, with the x
/// component negated. The first heading is taken between the re-aimed point
/// and the center, the second between the normalized frame direction (a zero
/// length yields the raw y component paired with 1.0; NaN normalizes to
/// NaN); their difference is wrapped into
/// [-pi, pi] and clamped into [-0.99, 0.99]. The gain compares the limit
/// against the length of the provided vector: a non-positive margin maps to
/// -0.1 above -5.0 else -0.2, a positive margin maps to 1.0 past a quarter
/// of the limit else ramps linearly. The float operation order is the
/// original's.
///
/// Original: 0x00d82a30 (cdecl, six stack words; plain `ret`).
lf_checker_rt::export!(cdecl, rw_00d82a30(state: u32, target: u32, out_angle: u32, out_gain: u32, out_zero_dword: u32, out_zero_byte: u32) -> u32 {
    unsafe {
        const FRAME_OFF: u32 = 0x20;
        const FLAGS_OFF: u32 = 0x2c;
        const LIMIT_OFF: u32 = 0xe6f;
        const DIR_X: u32 = 0x10;
        const DIR_Y: u32 = 0x14;
        const CENTER_X: u32 = 0x30;
        const CENTER_Y: u32 = 0x34;
        const TARGET_EXT: u32 = 0x20;
        const GAIN_FLAG: u8 = 0x01;
        const VEC_SLOT: u32 = 0xec;
        const GAIN_SET: f32 = -12.0;
        const GAIN_CLEAR: f32 = 26.0;
        const ONE: f32 = 1.0;
        const PI: f32 = f32::from_bits(0x4049_0fdb);
        const TWO_PI: f32 = f32::from_bits(0x40c9_0fdb);
        const NEG_PI: f32 = f32::from_bits(0xc049_0fdb);
        const NEG_FIVE: f32 = -5.0;
        const QUARTER: f32 = 0.25;
        const FOUR: f32 = 4.0;
        const NEG_TENTH: f32 = f32::from_bits(0xbdcc_cccd);
        const NEG_FIFTH: f32 = f32::from_bits(0xbe4c_cccd);
        const LO_CLAMP: f32 = f32::from_bits(0xbf7d_70a4); // -0.99
        const HI_CLAMP: f32 = f32::from_bits(0x3f7d_70a4); // 0.99
        const SIGN: u32 = 0x8000_0000;
        const HEADING_CALL1: u32 = 0;
        const HEADING_CALL2: u32 = 1;

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
        fn neg(a: f32) -> f32 {
            f32::from_bits(a.to_bits() ^ SIGN)
        }

        // Aimed point: through the extension object when present.
        let ext = rd32(target + TARGET_EXT);
        let aim = if ext != 0 { ext + CENTER_X } else { target + DIR_X };
        let frame = rd32(state + FRAME_OFF);
        // Offset from the center, normalized unless zero or NaN.
        let dx = sub(rdf(aim), rdf(frame + CENTER_X));
        let dy = sub(rdf(aim + 4), rdf(frame + CENTER_Y));
        let len2 = add(mul(dy, dy), mul(dx, dx));
        // The flag test only skips a zero length; NaN normalizes to NaN.
        let inv = if len2 == 0.0 {
            0.0
        } else {
            let root = core::hint::black_box(len2).sqrt();
            core::hint::black_box(ONE) / core::hint::black_box(root)
        };
        let gain = if rd8(state + FLAGS_OFF) & GAIN_FLAG != 0 { GAIN_SET } else { GAIN_CLEAR };
        let oy = mul(mul(inv, dy), gain);
        let ox = mul(neg(mul(inv, dx)), gain);
        // Re-aimed point and its heading against the center.
        let qx = add(rdf(aim), oy);
        let qy = add(rdf(aim + 4), ox);
        let vx = rdf(frame + DIR_X);
        let vy = rdf(frame + DIR_Y);
        let vlen = add(mul(vy, vy), mul(vx, vx)).sqrt();
        // Normalized frame direction; only a zero length keeps the raw y
        // paired with 1.0, NaN normalizes to NaN.
        let (n0, n1) = if vlen == 0.0 {
            (vy, ONE)
        } else {
            let s = core::hint::black_box(ONE) / core::hint::black_box(vlen);
            (mul(s, vy), mul(s, vx))
        };
        let hx = sub(qx, rdf(frame + CENTER_X));
        let hy = sub(qy, rdf(frame + CENTER_Y));
        let a1: f32 = lf_checker_rt::callee_cdecl!(HEADING_CALL1, f32, hx.to_bits(), hy.to_bits());
        let a2: f32 = lf_checker_rt::callee_cdecl!(HEADING_CALL2, f32, n1.to_bits(), n0.to_bits());
        // Wrapped heading difference.
        let mut d = sub(a1, a2);
        while d < NEG_PI {
            d = add(d, TWO_PI);
        }
        while d > PI {
            d = sub(d, TWO_PI);
        }
        // Provided vector against the limit.
        let mut pair = [a2.to_bits(), 0u32];
        let provider: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(rd32(rd32(state) + VEC_SLOT) as usize);
        let r = provider(state, pair.as_mut_ptr() as u32);
        let r0 = rdf(r);
        let r1 = rdf(r + 4);
        let m = add(mul(r1, r1), mul(r0, r0)).sqrt();
        let limit = rd8(state + LIMIT_OFF) as f32;
        let t = sub(limit, m);
        if t <= 0.0 {
            // Ordered comparison: NaN takes the other branch, as the
            // original's below-or-equal test does.
            wrf(out_gain, if NEG_FIVE <= t { NEG_TENTH } else { NEG_FIFTH });
        } else {
            let q = core::hint::black_box(t) / core::hint::black_box(limit);
            wrf(out_gain, if q > QUARTER { ONE } else { sub(ONE, mul(sub(QUARTER, q), FOUR)) });
        }
        wr32(out_zero_dword, 0);
        wrf(out_angle, d);
        (out_zero_byte as *mut u8).write(0);
        // Clamp into [-0.99, 0.99]; NaN falls to the lower bound.
        let mut c = rdf(out_angle);
        if !(c > LO_CLAMP) {
            c = LO_CLAMP;
        }
        if !(c < HI_CLAMP) {
            c = HI_CLAMP;
        }
        wrf(out_angle, c);
        out_zero_byte
    }
});
