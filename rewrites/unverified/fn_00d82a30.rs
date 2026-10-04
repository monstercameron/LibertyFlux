// original: 0x00D82A30 pointer_angle_strength (proposed)

/// Aim angle and strength from two world objects, written to four out-pointers.
///
/// `obj` is the aiming object: `+0x00` points to its vtable (the probe hook
/// lives in slot `+0xec`), `+0x20` points to a base block, `+0x2c` bit 0
/// selects the offset scale, `+0xe6f` is a level byte. `other` is the target:
/// `+0x20` points to an anchor block holding a position at `+0x30`/`+0x34`,
/// or is null, in which case the position is read from `other+0x10`/`+0x14`.
///
/// Algorithm: offset the target position sideways from the base-to-target
/// direction (perpendicular unit vector times -12.0 when the mode bit is set,
/// 26.0 otherwise), take the heading of that point minus the heading of the
/// normalized base direction (both through the heading callee), wrap the
/// difference into [-pi, pi] and clamp it into [-0.99, 0.99] for `out_angle`.
/// `out_strength` is 1.0 when the level exceeds the probe distance by more
/// than a quarter of the level, a ramp down from there, -0.1 when the probe
/// reaches past the level but not far, else -0.2. `out_zero` gets 0 and
/// `out_flag` gets a 0 byte. Returns `out_flag`.
///
/// Original: 0x00D82A30 (cdecl, six stack words). The two heading calls take
/// (x, y) float bits and answer an f32 in st0; the probe hook is thiscall
/// with one stack word (a pointer to scratch the hook fills) and answers a
/// pointer to two floats. Float operation order is the original's, including
/// the reversed addends of both length squares and the negated-then-scaled
/// perpendicular.
lf_checker_rt::export!(cdecl, rw_00d82a30(obj: u32, other: u32, out_angle: u32, out_strength: u32, out_zero: u32, out_flag: u32) -> u32 {
    unsafe {
        const OBJ_BASE: u32 = 0x20;
        const OBJ_MODE: u32 = 0x2c;
        const OBJ_LEVEL: u32 = 0xe6f;
        const OTHER_ANCHOR: u32 = 0x20;
        const ANCHOR_X: u32 = 0x30;
        const OTHER_X: u32 = 0x10;
        const BASE_DIR_X: u32 = 0x10;
        const BASE_X: u32 = 0x30;
        const VT_PROBE: u32 = 0xec;
        const HEADING_FIRST: u32 = 1;
        const HEADING_SECOND: u32 = 2;
        const ONE: f32 = 1.0;
        const SCALE_NEAR: f32 = -12.0;
        const SCALE_FAR: f32 = 26.0;
        const NEG_PI: f32 = f32::from_bits(0xC049_0FDB);
        const TWO_PI: f32 = f32::from_bits(0x40C9_0FDB);
        const PI: f32 = f32::from_bits(0x4049_0FDB);
        const NEG5: f32 = -5.0;
        const QUARTER: f32 = 0.25;
        const FOUR: f32 = 4.0;
        const NEG01: f32 = f32::from_bits(0xBDCC_CCCD);
        const NEG02: f32 = f32::from_bits(0xBE4C_CCCD);
        const LO: f32 = f32::from_bits(0xBF7D_70A4);
        const HI: f32 = f32::from_bits(0x3F7D_70A4);
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
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
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
            // The xor must stay after the producing operation: without the
            // black_box LLVM sinks it into a multiply operand (and may swap
            // the operands), which changes NaN payloads on this worker.
            f32::from_bits(core::hint::black_box(a.to_bits()) ^ SIGN)
        }

        // Target position: anchor block when present, else inside `other`.
        let anchor = rd32(other + OTHER_ANCHOR);
        let p = if anchor != 0 { anchor + ANCHOR_X } else { other + OTHER_X };
        let px = rdf(p);
        let base = rd32(obj + OBJ_BASE);
        let py = rdf(p + 4);
        let dx = sub(px, rdf(base + BASE_X));
        let dy = sub(py, rdf(base + BASE_X + 4));

        // Perpendicular unit offset, scaled by mode.
        let len2 = add(mul(dy, dy), mul(dx, dx));
        let inv = if len2 != 0.0 { div(ONE, len2.sqrt()) } else { 0.0 };
        let mut ox = mul(inv, dy);
        let mut oy = neg(mul(inv, dx));
        let k = if rd8(obj + OBJ_MODE) & 1 != 0 { SCALE_NEAR } else { SCALE_FAR };
        ox = mul(ox, k);
        oy = mul(oy, k);

        // Offset point and normalized base direction.
        let ax = add(px, ox);
        let ay = add(py, oy);
        let dirx = rdf(base + BASE_DIR_X);
        let diry = rdf(base + BASE_DIR_X + 4);
        let blen = add(mul(diry, diry), mul(dirx, dirx)).sqrt();
        let (h2x, h2y) = if blen != 0.0 {
            let r = div(ONE, blen);
            (mul(r, dirx), mul(r, diry))
        } else {
            (ONE, diry)
        };

        let h1: f32 = lf_checker_rt::callee_cdecl!(
            HEADING_FIRST, f32,
            sub(ax, rdf(base + BASE_X)).to_bits(),
            sub(ay, rdf(base + BASE_X + 4)).to_bits()
        );
        let h2: f32 =
            lf_checker_rt::callee_cdecl!(HEADING_SECOND, f32, h2x.to_bits(), h2y.to_bits());

        // Wrapped angle difference.
        let mut d = sub(h1, h2);
        while NEG_PI > d {
            d = add(d, TWO_PI);
        }
        while d > PI {
            d = sub(d, TWO_PI);
        }

        // Probe hook through the vtable; answers two floats.
        let slot = rd32(rd32(obj) + VT_PROBE);
        let probe: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(slot as usize);
        let mut buf = [0u32; 2];
        let rp = probe(obj, buf.as_mut_ptr() as u32);
        let rx = rdf(rp);
        let ry = rdf(rp + 4);

        // Strength from probe distance against the level byte.
        let dist = add(mul(ry, ry), mul(rx, rx)).sqrt();
        let level = rd8(obj + OBJ_LEVEL) as f32;
        let gap = sub(level, dist);
        // `comiss 0, gap; jb` takes the positive branch for NaN too (CF=1
        // when unordered), hence the negated `>=` rather than `<`.
        let strength = if !(0.0 >= gap) {
            let ratio = div(gap, level);
            if ratio > QUARTER {
                ONE
            } else {
                sub(ONE, mul(sub(QUARTER, ratio), FOUR))
            }
        } else if NEG5 > gap {
            NEG02
        } else {
            NEG01
        };
        wrf(out_strength, strength);

        wr32(out_zero, 0);
        wrf(out_angle, d);
        (out_flag as *mut u8).write(0);
        let mut a = rdf(out_angle);
        if !(a > LO) {
            a = LO;
        }
        if !(HI > a) {
            a = HI;
        }
        wrf(out_angle, a);
        out_flag
    }
});
