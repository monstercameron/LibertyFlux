// original: 0x00A218A0 ped_task_turn_update (proposed)

/// Turn-rate tracker with a decaying step: advances an object's turn state
/// and pulls a target point back along a scaled direction.
///
/// `this` is the task object (blend rate float at `+0x2C4`, flag byte at
/// `+0x38C`), `a1` points at three floats (a target point, updated in
/// place), `a2` is a parameter block (inner pointer at `+0x20`, angle float
/// at `+0x1C`), `a3` is a float gate value. All floats are single precision
/// and every arithmetic step below runs in the original's SSE order.
///
/// Behaviour: when flag bit 0x80 is set the rate is first reset to 1.0.
/// When `a3` is ordered-greater than the gate constant (0.02), the rate
/// grows by the step constant (0.01) and is clamped to at most 1.0 (an
/// unordered or over-one sum stores 1.0). A direction triple is then taken
/// either from the inner block (`+0x10/+0x14/+0x18`) when the inner pointer
/// is non-null, or from the two trig callees (negated sine, cosine, 0.0 of
/// the angle) otherwise. Each component is scaled by 0.1 and then by the
/// rate, in that lane order, and subtracted from the target point.
/// Returns `a1`. Original is thiscall with three stack words.
lf_checker_rt::export!(thiscall, rw_00A218A0(this: u32, a1: u32, a2: u32, a3: u32) -> u32 {
    unsafe {
        const RATE_OFF: u32 = 0x2C4;
        const FLAG_OFF: u32 = 0x38C;
        const FLAG_RESET: u8 = 0x80;
        const INNER_OFF: u32 = 0x20;
        const ANGLE_OFF: u32 = 0x1C;
        const DIR_X: u32 = 0x10;
        const DIR_Y: u32 = 0x14;
        const DIR_Z: u32 = 0x18;
        const RATE_GATE: f32 = f32::from_bits(0x3CA3_D70A); // 0.02
        const RATE_STEP: f32 = f32::from_bits(0x3C23_D70A); // 0.01
        const DIR_SCALE: f32 = f32::from_bits(0x3DCC_CCCD); // 0.1
        const ONE: f32 = 1.0;
        const SIGN: u32 = 0x8000_0000;
        const SIN_CALLEE: u32 = 1;
        const COS_CALLEE: u32 = 2;

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
        fn neg(a: f32) -> f32 {
            f32::from_bits(a.to_bits() ^ SIGN)
        }

        if rd8(this + FLAG_OFF) & FLAG_RESET != 0 {
            wrf(this + RATE_OFF, ONE);
        }
        if f32::from_bits(a3) > RATE_GATE {
            let mut r = add(rdf(this + RATE_OFF), RATE_STEP);
            if !(ONE > r) {
                r = ONE;
            }
            wrf(this + RATE_OFF, r);
        }
        let inner = rd32(a2 + INNER_OFF);
        let (dx, dy, dz) = if inner != 0 {
            (rdf(inner + DIR_X), rdf(inner + DIR_Y), rdf(inner + DIR_Z))
        } else {
            let ang = rdf(a2 + ANGLE_OFF);
            let s_bits: u32 = lf_checker_rt::callee_cdecl!(SIN_CALLEE, u32, ang.to_bits());
            let c_bits: u32 = lf_checker_rt::callee_cdecl!(COS_CALLEE, u32, ang.to_bits());
            (neg(f32::from_bits(s_bits)), f32::from_bits(c_bits), 0.0)
        };
        let k = rdf(this + RATE_OFF);
        let mut dx = mul(dx, DIR_SCALE);
        let mut dy = mul(dy, DIR_SCALE);
        dx = mul(dx, k);
        let mut dz = mul(dz, DIR_SCALE);
        dy = mul(dy, k);
        dz = mul(dz, k);
        wrf(a1, sub(rdf(a1), dx));
        wrf(a1 + 4, sub(rdf(a1 + 4), dy));
        wrf(a1 + 8, sub(rdf(a1 + 8), dz));
        a1
    }
});
