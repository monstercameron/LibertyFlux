// original: 0x0096F860 audio_dual_fade_update (proposed)

/// Step two gated fade slots toward their targets and push the results out.
///
/// `this` is a mixer object with two one-byte slot states at `+0x1268`
/// (a zeroed word first unless the gate below holds), per-slot control
/// dwords at `+0x126c` (an expiry time, a base level at `+8`, a floor at
/// `+0x10`, overlapping by one word between the slots), output levels at
/// `+0x134c`/`+0x1350` and a reference level at `+0x2a2c`.
///
/// A meter reading (callee 0) and a gate flag (callee 1, forced to 0/1
/// together with two global bytes) decide, with a threshold global and
/// the meter staying below 1.0, whether the states are cleared. Each
/// slot then runs a four-way switch on its state byte (anything above 3
/// leaves the slot's scratch at its zero fill): state 0 writes a zero
/// level; state 1 latches to state 2 with a fresh expiry when the sample
/// clock has reached the stored time, else interpolates between floor
/// and base by the remaining fraction of the first duration; state 2
/// latches to state 3 the same way and otherwise holds its base; state 3
/// clears to state 0 once expired, else scales its base by that fraction
/// of the second duration. Fractions convert the unsigned clock, time
/// and duration through double precision exactly as the original's
/// convert-and-add-table sequence does.
///
/// The two levels are stored to the outputs, smoothed one after the
/// other (callee 2), and both answers are sunk together (callee 3 takes
/// two stack words). Their maximum is smoothed again, then sunk with
/// the reference level's minimum (callees 2, 4); that maximum goes
/// through the smoother once more and is sunk (callees 2, 5); and a
/// final flag records whether both states are zero (callee 6). All float
/// operations keep the original's operand order. Returns nothing
/// (thiscall, no stack arguments, no return channel: EAX on return is
/// the last sink call's answer).
lf_checker_rt::export!(thiscall, rw_0096f860(this: u32) -> u32 {
    unsafe {
        const STATE_BASE: u32 = 0x1268;
        const CTL_BASE: u32 = 0x126c;
        const CTL_BASE_LEVEL: u32 = 0x08;
        const CTL_FLOOR: u32 = 0x10;
        const OUT_FIRST: u32 = 0x134c;
        const OUT_SECOND: u32 = 0x1350;
        const LEVEL_REF: u32 = 0x2a2c;
        const SMOOTH_A: u32 = 0x1284;
        const SMOOTH_B: u32 = 0x12ac;
        const SMOOTH_C: u32 = 0x12d4;
        const G_THRESHOLD: u32 = 0x103234c;
        const G_CLOCK: u32 = 0x11618fc;
        const G_HOLD: u32 = 0x11618f2;
        const G_GATE0: u32 = 0x1283049;
        const G_GATE1: u32 = 0x129576e;
        const G_DUR0: u32 = 0x10379b0;
        const G_DUR1: u32 = 0x10379ac;
        const G_DUR2: u32 = 0x10379b4;
        const THIS_METER: u32 = 0x128e310;
        const THIS_GATE: u32 = 0x128e400;
        const THIS_SINK: u32 = 0x115def0;
        const C_NEAR_ONE: u32 = 0xfe88bc;
        const C_ONE: u32 = 0xfe88e8;
        const C_U2F: u32 = 0xfe8f50;
        const CALLEE_METER: u32 = 0;
        const CALLEE_GATE: u32 = 1;
        const CALLEE_SMOOTH: u32 = 2;
        const CALLEE_SINK_A: u32 = 3;
        const CALLEE_SINK_B: u32 = 4;
        const CALLEE_SINK_C: u32 = 5;
        const CALLEE_FLAG: u32 = 6;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn rf(a: u32) -> f32 {
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
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn div(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) / core::hint::black_box(b)
        }
        #[inline(always)]
        unsafe fn g32(va: u32) -> u32 {
            unsafe { (lf_checker_rt::global::<u32>(va) as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn g8(va: u32) -> u8 {
            unsafe { (lf_checker_rt::global::<u8>(va) as *const u8).read() }
        }

        /// Unsigned word to float exactly as the original: signed convert
        /// to double, add the table's 0.0 or 2^32 by the sign bit (both
        /// steps exact), round once to float.
        #[inline(always)]
        unsafe fn u2f(x: u32) -> f32 {
            unsafe {
                let t = ((lf_checker_rt::relocated(C_U2F) + (x >> 31) * 8) as *const f64)
                    .read_unaligned();
                ((x as i32) as f64 + t) as f32
            }
        }

        let mem = f32::from_bits(g32(G_THRESHOLD));
        let clock = g32(G_CLOCK);
        let f: f32 = lf_checker_rt::callee_thiscall!(CALLEE_METER, f32, lf_checker_rt::relocated(THIS_METER),);
        let gate: u32 = lf_checker_rt::callee_thiscall!(CALLEE_GATE, u32, lf_checker_rt::relocated(THIS_GATE),);
        let mut al: u8 = gate as u8;
        if al != 0 || g8(G_GATE0) != 0 || g8(G_GATE1) != 0 {
            al = 1;
        }
        let near_one = f32::from_bits(g32(C_NEAR_ONE));
        let one = f32::from_bits(g32(C_ONE));
        if !(near_one > mem) && g8(G_HOLD) == 0 && !(f >= one) && al == 0 {
            // Gate holds: keep the states as they are.
        } else {
            ((this + STATE_BASE) as *mut u16).write_unaligned(0);
        }
        // A slot the switch skips (state above 3) keeps the zero stack fill.
        let mut slot = [0.0f32, 0.0f32];
        for i in 0..2u32 {
            let st_addr = this + STATE_BASE + i;
            let ctl = this + CTL_BASE + i * 4;
            let st = rd8(st_addr);
            if st > 3 {
                continue;
            }
            match st {
                0 => {
                    slot[i as usize] = 0.0;
                }
                1 => {
                    let t = rd32(ctl);
                    if clock < t {
                        let frac = div(sub(u2f(t), u2f(clock)), u2f(g32(G_DUR1)));
                        let base = rf(ctl + CTL_BASE_LEVEL);
                        let lo = rf(ctl + CTL_FLOOR);
                        slot[i as usize] = add(mul(sub(one, frac), sub(base, lo)), lo);
                    } else {
                        ((st_addr) as *mut u8).write(2);
                        wr32(ctl, clock.wrapping_add(g32(G_DUR0)));
                        slot[i as usize] = rf(ctl + CTL_BASE_LEVEL);
                    }
                }
                2 => {
                    let t = rd32(ctl);
                    if clock >= t {
                        ((st_addr) as *mut u8).write(3);
                        wr32(ctl, clock.wrapping_add(g32(G_DUR2)));
                    }
                    slot[i as usize] = rf(ctl + CTL_BASE_LEVEL);
                }
                _ => {
                    let t = rd32(ctl);
                    if clock < t {
                        let frac = div(sub(u2f(t), u2f(clock)), u2f(g32(G_DUR2)));
                        let inner = sub(one, frac);
                        slot[i as usize] = mul(sub(one, inner), rf(ctl + CTL_BASE_LEVEL));
                    } else {
                        ((st_addr) as *mut u8).write(0);
                        slot[i as usize] = 0.0;
                    }
                }
            }
        }
        let a1: f32 = lf_checker_rt::callee_thiscall!(CALLEE_SMOOTH, f32, this + SMOOTH_A, slot[1].to_bits());
        let a2: f32 = lf_checker_rt::callee_thiscall!(CALLEE_SMOOTH, f32, this + SMOOTH_A, slot[0].to_bits());
        let _: u32 = lf_checker_rt::callee_thiscall!(
            CALLEE_SINK_A,
            u32,
            lf_checker_rt::relocated(THIS_SINK),
            a2.to_bits(),
            a1.to_bits()
        );
        wrf(this + OUT_FIRST, slot[0]);
        wrf(this + OUT_SECOND, slot[1]);
        let mut m = slot[0];
        if !(slot[0] > slot[1]) {
            m = slot[1];
        }
        let a3: f32 = lf_checker_rt::callee_thiscall!(CALLEE_SMOOTH, f32, this + SMOOTH_B, m.to_bits());
        let h = rf(this + LEVEL_REF);
        let mut n = h;
        if !(a3 > h) {
            n = a3;
        }
        let _: u32 = lf_checker_rt::callee_thiscall!(CALLEE_SINK_B, u32, lf_checker_rt::relocated(THIS_SINK), n.to_bits());
        let a4: f32 = lf_checker_rt::callee_thiscall!(CALLEE_SMOOTH, f32, this + SMOOTH_C, m.to_bits());
        let _: u32 = lf_checker_rt::callee_thiscall!(CALLEE_SINK_C, u32, lf_checker_rt::relocated(THIS_SINK), a4.to_bits());
        let done_zero = rd8(this + STATE_BASE) == 0 && rd8(this + STATE_BASE + 1) == 0;
        let _: u32 = lf_checker_rt::callee_thiscall!(
            CALLEE_FLAG,
            u32,
            lf_checker_rt::relocated(THIS_SINK),
            if done_zero { 0 } else { 1 }
        );
        0
    }
});
