// original: 0x00ccf900 melee_aim_compute
/// Compute the melee aim pair into two out-params, or store nothing.
///
/// Reads the gate float at `this+0x64`: when it is `<= +0.0` (ordered) the
/// function returns without writing anything. Otherwise it normalises the
/// direction (`this+0x50`, `this+0x54`) by its length (a zero or NaN length
/// takes careful IEEE paths through square roots and a reciprocal), folds
/// the normalised pair with a data row (`[link+0x20]`, words at `+0`,
/// `+4`, `+0x10`, `+0x14`), negates the first output, scales both by
/// `this+0x60` times the global at 0x0105195c, and stores them to the second
/// and third stack arguments. Every float operation runs in the original's
/// operand order with pinned evaluation. Thiscall with three stack args.
export!(thiscall, rw_00ccf900(this: u32, link: u32, out_a: u32, out_b: u32) -> u32 {
    unsafe {
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn div(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) / core::hint::black_box(b)
        }
        #[inline(always)]
        fn sqrt(a: f32) -> f32 {
            core::hint::black_box(a).sqrt()
        }
        #[inline(always)]
        unsafe fn rd(a: u32) -> f32 {
            unsafe { f32::from_bits((a as *const u32).read_unaligned()) }
        }
        const GATE_OFF: u32 = 0x64;
        const DIR_X_OFF: u32 = 0x50;
        const DIR_Y_OFF: u32 = 0x54;
        const SCALE_OFF: u32 = 0x60;
        const ROW_OFF: u32 = 0x20;
        const GAIN_G: u32 = 0x0105195c;
        const ONE_G: u32 = 0x00fe88e8;
        const MASK_G: u32 = 0x00fe8fa0;
        const ZERO: f32 = 0.0;
        let gate = rd(this.wrapping_add(GATE_OFF));
        if gate <= ZERO {
            return out_b;
        }
        let ax = rd(this.wrapping_add(DIR_X_OFF));
        let ay = rd(this.wrapping_add(DIR_Y_OFF));
        let len_sq = add(mul(ax, ax), mul(ay, ay));
        let inv = if len_sq != ZERO {
            div(f32::from_bits(*global::<u32>(ONE_G)), sqrt(len_sq))
        } else {
            ZERO
        };
        let nx = mul(inv, ax);
        let ny = mul(inv, ay);
        let zed = mul(inv, ZERO);
        let norm_sq = add(add(mul(nx, nx), mul(ny, ny)), mul(zed, zed));
        let (o1, o2) = if sqrt(norm_sq) > ZERO {
            let row = (link.wrapping_add(ROW_OFF) as *const u32).read_unaligned();
            let gain = mul(rd(this.wrapping_add(SCALE_OFF)), f32::from_bits(*global::<u32>(GAIN_G)));
            let zc = mul(zed, ZERO);
            let m10 = mul(rd(row.wrapping_add(0x10)), nx);
            let m00 = mul(rd(row), nx);
            let mut v2 = add(mul(rd(row.wrapping_add(0x14)), ny), m10);
            let m04 = mul(rd(row.wrapping_add(0x04)), ny);
            v2 = add(v2, zc);
            let mut v6 = add(m00, m04);
            v2 = f32::from_bits(v2.to_bits() ^ *global::<u32>(MASK_G));
            v6 = add(v6, zc);
            v2 = mul(v2, gain);
            v6 = mul(v6, gain);
            (v2, v6)
        } else {
            (ZERO, ZERO)
        };
        (out_a as *mut u32).write_unaligned(o1.to_bits());
        (out_b as *mut u32).write_unaligned(o2.to_bits());
        out_b
    }
});
