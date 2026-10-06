// original: 0x00D86370 aim_assist_steer (proposed)

/// Steer an aim-assist correction from two world objects and a target point.
///
/// `a0` is the subject object, `a1` the reference object (both with an inner
/// state block at `+0x20` and a status word at `+0x1304`), `a2` points at the
/// target point (two floats), `a3`/`a4`/`a5` receive float results, `a6`
/// receives a zero byte, and `a7` is an integer whose low 16 bits the original
/// reuses as the high half of a float (see the quirk note below).
///
/// Behaviour in order: when `a1` is non-null, has status bits `0xC0` and the
/// substitute bit, it is replaced by the object at `+0xB30`. A direction is
/// read from the subject's inner block (`+0x10`, `+0x14`, negated when the
/// flag byte at `+0xE73` is set) and normalised, or set to (1, y) when its
/// length is exactly zero. Two heading helpers run (callee 1, on the target
/// offset and on the direction), then a nine-argument scorer (callee 2,
/// taking both headings among its arguments); the scorer's answer minus the
/// second heading is wrapped into [-pi, pi] and then discarded, because
/// callee 3 overwrites its slot through an out-pointer. Callee 3 fills three
/// frame slots whose values steer the rest of the function. A third heading
/// call is made and its answer dropped. From here on the function compares a
/// projection of (target - reference) onto the direction against 1: the far
/// path calls a virtual slot (`+0xEC`) on the reference object and blends
/// lengths with several clamped minima; the near path either returns early
/// (storing the frame slot, 0, and a ramp of the projection) or joins the
/// main path with a default radius. The main path then, when both status
/// words read 1 and the mode byte at `+0xE6E` is 10 or 11, calls the same
/// virtual slot on both objects, feeds the radius and a length into a
/// seven-argument scorer (callee 4), clamps a function of `*a5` into
/// `+0x12C8`, stores the frame slot to `*a3`, and either picks a countdown
/// from a helper (callee 5, in the executable's encrypted first megabyte, so
/// always intercepted) or runs a timestamp check against the global tick
/// with an unsigned 700-tick window. A trailing flag scales `*a3` and `*a4`
/// by -1.
///
/// Dead values reproduced faithfully: the wrapped angle is overwritten by
/// callee 3 before any read, and the scorer's answer slot is reused for the
/// projection. All float comparisons are the original's ordered
/// single-precision compares, so NaN takes the same side in every branch,
/// and every multi-operand float expression keeps the original's operand
/// order.
///
/// Original: 0x00D86370 (cdecl, eight stack words, no return value).
lf_checker_rt::export!(cdecl, rw_00D86370(a0: u32, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32, a6: u32, a7: u32) -> u32 {
    unsafe {
        const INNER: u32 = 0x20;
        const DIR_X: u32 = 0x10;
        const DIR_Y: u32 = 0x14;
        const DIR_Z: u32 = 0x18;
        const REF_X: u32 = 0x30;
        const REF_Y: u32 = 0x34;
        const FLAGS28: u32 = 0x28;
        const FLAG_MASK: u32 = 0x3C0;
        const FLAG_SUB: u32 = 0xC0;
        const FLAG_MAIN: u32 = 0x80;
        const SUB_BYTE: u32 = 0x26C;
        const SUB_BIT: u8 = 4;
        const ALT_OBJ: u32 = 0xB30;
        const MODE: u32 = 0xE6E;
        const MODE_A: u8 = 0x0A;
        const MODE_B: u8 = 0x0B;
        const MAG_BYTE: u32 = 0xE6F;
        const FLIP_BYTE: u32 = 0xE73;
        const STATUS: u32 = 0x1304;
        const STAMP: u32 = 0x1308;
        const OUT_CLAMP: u32 = 0x12C8;
        const OUT_COUNT: u32 = 0x12E5;
        const F1088: u32 = 0x1088;
        const VT_SLOT: u32 = 0xEC;
        const TS_WINDOW: u32 = 0x2BC;
        const GLOBAL_TICK: u32 = 0x11735B4;
        const SIGN_BIT: u32 = 0x8000_0000;
        const ABS_MASK: u32 = 0x7FFF_FFFF;
        const ONE: f32 = f32::from_bits(0x3F80_0000);
        const NEG_ONE: f32 = f32::from_bits(0xBF80_0000);
        const NEG_PI: f32 = f32::from_bits(0xC049_0FDB);
        const TWO_PI: f32 = f32::from_bits(0x40C9_0FDB);
        const PI: f32 = f32::from_bits(0x4049_0FDB);
        const THREE: f32 = f32::from_bits(0x4040_0000);
        const FOUR: f32 = f32::from_bits(0x4080_0000);
        const TENTH: f32 = f32::from_bits(0x3DCC_CCCD);
        const FIFTEEN: f32 = f32::from_bits(0x4170_0000);
        const TEN: f32 = f32::from_bits(0x4120_0000);
        const EIGHT: f32 = f32::from_bits(0x4100_0000);
        const TWELVE: f32 = f32::from_bits(0x4140_0000);
        const NEG_HALF1: f32 = f32::from_bits(0xBFC0_0000);
        const FOUR_TENTHS: f32 = f32::from_bits(0x3ECC_CCCD);
        const NINE_TENTHS: f32 = f32::from_bits(0x3F66_6666);
        const SEVEN_TENTHS: f32 = f32::from_bits(0x3F33_3333);
        const THREE_TENTHS: f32 = f32::from_bits(0x3E99_999A);
        const TEN_SEVENTHS: f32 = f32::from_bits(0x3FB6_DB6E);
        const TWENTIETH: f32 = f32::from_bits(0x3D4C_CCCD);

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
        fn negf(a: f32) -> f32 {
            f32::from_bits(a.to_bits() ^ SIGN_BIT)
        }
        #[inline(always)]
        fn absf(a: f32) -> f32 {
            f32::from_bits(a.to_bits() & ABS_MASK)
        }
        #[inline(always)]
        unsafe fn vcall(obj: u32, slot: *mut u32) -> u32 {
            unsafe {
                let f: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(rd32(rd32(obj).wrapping_add(VT_SLOT)) as usize);
                f(obj, slot as u32)
            }
        }
        #[inline(always)]
        unsafe fn atan2p(x: f32, y: f32) -> f32 {
            unsafe { lf_checker_rt::callee_cdecl!(1, f32, x.to_bits(), y.to_bits()) }
        }

        // Block A: reference-object substitution.
        let mut edi = a1;
        if edi != 0 {
            if (rd32(edi.wrapping_add(FLAGS28)) & FLAG_MASK) == FLAG_SUB {
                if (rd8(edi.wrapping_add(SUB_BYTE)) & SUB_BIT) != 0 {
                    let t = rd32(edi.wrapping_add(ALT_OBJ));
                    if t != 0 {
                        edi = t;
                    }
                }
            }
        }
        let esi = a0;
        wr8(a6, 0);

        // Block B: direction with optional sign flip, then normalise.
        let ecx = rd32(esi.wrapping_add(INNER));
        let mut x = rdf(ecx.wrapping_add(DIR_X));
        let mut y = rdf(ecx.wrapping_add(DIR_Y));
        if (rd8(esi.wrapping_add(FLIP_BYTE)) & 1) != 0 {
            x = negf(x);
            y = negf(y);
        }
        let len = add(mul(y, y), mul(x, x)).sqrt();
        let (nx, ny) = if len == 0.0 {
            (ONE, y)
        } else {
            let inv = div(ONE, len);
            (mul(inv, x), mul(y, inv))
        };

        // Block C: two heading calls.
        let h1 = atan2p(
            sub(rdf(a2), rdf(ecx.wrapping_add(REF_X))),
            sub(rdf(a2.wrapping_add(4)), rdf(ecx.wrapping_add(REF_Y))),
        );
        let h2 = atan2p(nx, ny);

        // Block D: scorer call, difference from the second heading, angle
        // wrap (dead: the next call overwrites the slot, but the wrap bound
        // is still honoured).
        let r: f32 = lf_checker_rt::callee_cdecl!(
            2, f32, esi, 0, h1.to_bits(), h2.to_bits(), ONE.to_bits(), 2, 1, 1, a7
        );
        let mut d = sub(r, h2);
        if d < NEG_PI {
            loop {
                d = add(d, TWO_PI);
                if !(NEG_PI > d) {
                    break;
                }
            }
        }
        if d > PI {
            loop {
                d = sub(d, TWO_PI);
                if !(d > PI) {
                    break;
                }
            }
        }

        // Block E: frame-slot filler, then a discarded heading call.
        let mut s0c = d.to_bits();
        let mut s1c: u32 = 0;
        let mut s18: u32 = 0;
        lf_checker_rt::callee_cdecl!(
            3, u32, esi,
            core::ptr::addr_of_mut!(s0c) as u32,
            core::ptr::addr_of_mut!(s1c) as u32,
            core::ptr::addr_of_mut!(s18) as u32
        );
        let ecx2 = rd32(esi.wrapping_add(INNER));
        let _h3 = atan2p(
            sub(rdf(a2), rdf(ecx2.wrapping_add(REF_X))),
            sub(rdf(a2.wrapping_add(4)), rdf(ecx2.wrapping_add(REF_Y))),
        );

        // Block F: projection of (target - reference) onto the direction.
        let dx = sub(rdf(a2), rdf(ecx2.wrapping_add(REF_X)));
        let dy = sub(rdf(a2.wrapping_add(4)), rdf(ecx2.wrapping_add(REF_Y)));
        let dot = add(mul(dx, nx), mul(dy, ny));
        let dist = add(mul(dx, dx), mul(dy, dy)).sqrt();
        // comiss+jbe: the near path takes unordered (NaN) too.
        if !(dot > ONE) {
            // Block H: near path.
            if FIFTEEN > dist {
                let eax = rd32(edi.wrapping_add(INNER));
                let dot3 = add(
                    add(
                        mul(rdf(ecx2.wrapping_add(DIR_Y)), rdf(eax.wrapping_add(DIR_Y))),
                        mul(rdf(ecx2.wrapping_add(DIR_X)), rdf(eax.wrapping_add(DIR_X))),
                    ),
                    mul(rdf(ecx2.wrapping_add(DIR_Z)), rdf(eax.wrapping_add(DIR_Z))),
                );
                if dot3 > SEVEN_TENTHS {
                    wrf(a3, f32::from_bits(s0c));
                    wr32(a4, 0);
                    if !(NEG_HALF1 > dot) {
                        let t = add(mul(mul(sub(ONE, dot), FOUR_TENTHS), NINE_TENTHS), TENTH);
                        wrf(a5, t);
                    } else {
                        wrf(a5, ONE);
                    }
                    return 0;
                }
            }
            // L3 joins the main path with a default radius.
            let e8 = EIGHT;
            fn_00d86370_main(esi, edi, a2, a3, a4, a5, dist, e8, dist, s0c, s1c, s18)
        } else {
            // Block G: far path through the reference object's virtual slot.
            let mut slot: u32 = 0;
            let p = vcall(edi, core::ptr::addr_of_mut!(slot));
            let lp = add(
                add(mul(rdf(p), rdf(p)), mul(rdf(p.wrapping_add(4)), rdf(p.wrapping_add(4)))),
                mul(rdf(p.wrapping_add(8)), rdf(p.wrapping_add(8))),
            );
            let mut x5 = dist;
            let m1x: f32;
            if !(FIFTEEN > dist) {
                m1x = FIFTEEN;
            } else {
                let x1 = sub(sub(dot, ONE), TENTH);
                let x4 = if !(x1 > FOUR) { x1 } else { FOUR };
                x5 = mul(dist, THREE);
                let t = add(x4, lp);
                m1x = if t > x5 { t } else { x5 };
            }
            let bval = rd8(esi.wrapping_add(MAG_BYTE)) as f32;
            let mut e8 = if bval > m1x { m1x } else { bval };
            let lp10 = add(lp, TEN);
            if !(lp10 > e8) {
                e8 = lp10;
            }
            fn_00d86370_main(esi, edi, a2, a3, a4, a5, dist, e8, x5, s0c, s1c, s18)
        }
    }
});

/// Shared tail of `rw_00D86370`: the L2/L4/L5 blocks. `x5` is the
/// path-dependent bound (three times the distance on the computed far path,
/// the distance itself everywhere else).
#[allow(clippy::too_many_arguments)]
unsafe fn fn_00d86370_main(
    esi: u32,
    edi: u32,
    a2: u32,
    a3: u32,
    a4: u32,
    a5: u32,
    _dist: f32,
    mut e8: f32,
    x5: f32,
    s0c: u32,
    mut s1c: u32,
    s18: u32,
) -> u32 {
    unsafe {
        const INNER: u32 = 0x20;
        const DIR_X: u32 = 0x10;
        const DIR_Y: u32 = 0x14;
        const REF_X: u32 = 0x30;
        const REF_Y: u32 = 0x34;
        const FLAGS28: u32 = 0x28;
        const FLAG_MASK: u32 = 0x3C0;
        const FLAG_MAIN: u32 = 0x80;
        const MODE: u32 = 0xE6E;
        const MODE_A: u8 = 0x0A;
        const MODE_B: u8 = 0x0B;
        const FLIP_BYTE: u32 = 0xE73;
        const STATUS: u32 = 0x1304;
        const STAMP: u32 = 0x1308;
        const OUT_CLAMP: u32 = 0x12C8;
        const OUT_COUNT: u32 = 0x12E5;
        const F1088: u32 = 0x1088;
        const VT_SLOT: u32 = 0xEC;
        const TS_WINDOW: u32 = 0x2BC;
        const GLOBAL_TICK: u32 = 0x11735B4;
        const SIGN_BIT: u32 = 0x8000_0000;
        const ABS_MASK: u32 = 0x7FFF_FFFF;
        const ONE: f32 = f32::from_bits(0x3F80_0000);
        const NEG_ONE: f32 = f32::from_bits(0xBF80_0000);
        const FOUR: f32 = f32::from_bits(0x4080_0000);
        const TEN: f32 = f32::from_bits(0x4120_0000);
        const TWELVE: f32 = f32::from_bits(0x4140_0000);
        const TENTH: f32 = f32::from_bits(0x3DCC_CCCD);
        const THREE_TENTHS: f32 = f32::from_bits(0x3E99_999A);
        const TEN_SEVENTHS: f32 = f32::from_bits(0x3FB6_DB6E);
        const TWENTIETH: f32 = f32::from_bits(0x3D4C_CCCD);

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
        fn negf(a: f32) -> f32 {
            f32::from_bits(a.to_bits() ^ SIGN_BIT)
        }
        #[inline(always)]
        fn absf(a: f32) -> f32 {
            f32::from_bits(a.to_bits() & ABS_MASK)
        }
        #[inline(always)]
        unsafe fn vcall(obj: u32, slot: *mut u32) -> u32 {
            unsafe {
                let f: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(rd32(rd32(obj).wrapping_add(VT_SLOT)) as usize);
                f(obj, slot as u32)
            }
        }

        let _ = a2;
        // Block I: gated second virtual call on the reference object.
        if rd32(esi.wrapping_add(STATUS)) == 1
            && (rd32(edi.wrapping_add(FLAGS28)) & FLAG_MASK) == FLAG_MAIN
            && rd32(edi.wrapping_add(STATUS)) == 1
        {
            let al = rd8(esi.wrapping_add(MODE));
            if al == MODE_A || al == MODE_B {
                let mut x4 = rdf(edi.wrapping_add(F1088));
                if al == MODE_B {
                    x4 = negf(x4);
                }
                let ecx = rd32(edi.wrapping_add(INNER));
                let base = if ecx != 0 { ecx.wrapping_add(REF_X) } else { edi.wrapping_add(DIR_X) };
                let eax2 = rd32(esi.wrapping_add(INNER));
                let d1 = sub(rdf(base.wrapping_add(4)), rdf(eax2.wrapping_add(REF_Y)));
                let d0 = sub(rdf(base), rdf(eax2.wrapping_add(REF_X)));
                let dot2 = add(mul(rdf(ecx.wrapping_add(DIR_Y)), d1), mul(rdf(ecx.wrapping_add(DIR_X)), d0));
                if (ONE > dot2) && (TEN > x5) && (x4 > TENTH) {
                    let mut slot: u32 = 0;
                    let q = vcall(edi, core::ptr::addr_of_mut!(slot));
                    let lq = add(
                        add(mul(rdf(q), rdf(q)), mul(rdf(q.wrapping_add(4)), rdf(q.wrapping_add(4)))),
                        mul(rdf(q.wrapping_add(8)), rdf(q.wrapping_add(8))),
                    )
                    .sqrt();
                    let mut t = sub(lq, FOUR);
                    if t < 0.0 {
                        t = 0.0;
                    }
                    if !(t > e8) {
                        e8 = t;
                    }
                }
            }
        }

        // Block J: virtual call on the subject, scorer, clamp, store.
        let mut slot: u32 = 0;
        let s = vcall(esi, core::ptr::addr_of_mut!(slot));
        let l3 = add(
            add(mul(rdf(s), rdf(s)), mul(rdf(s.wrapping_add(4)), rdf(s.wrapping_add(4)))),
            mul(rdf(s.wrapping_add(8)), rdf(s.wrapping_add(8))),
        )
        .sqrt();
        lf_checker_rt::callee_cdecl!(
            4, u32, e8.to_bits(), l3.to_bits(), a5, a4, esi,
            core::ptr::addr_of_mut!(s1c) as u32, s18
        );
        if rd32(esi.wrapping_add(STATUS)) == 1 {
            let xv = rdf(a5);
            if xv > THREE_TENTHS {
                let mut t = mul(sub(xv, THREE_TENTHS), TEN_SEVENTHS);
                if t < 0.0 {
                    t = 0.0;
                }
                if t > ONE {
                    t = ONE;
                }
                wrf(esi.wrapping_add(OUT_CLAMP), t);
            }
        }
        wrf(a3, f32::from_bits(s0c)); // MUT-SKIPSTORE

        // Block K: countdown or timestamp check.
        if rd32(esi.wrapping_add(STATUS)) == 1
            && (rd32(edi.wrapping_add(FLAGS28)) & FLAG_MASK) == FLAG_MAIN
            && rd32(edi.wrapping_add(STATUS)) == 1
        {
            let al = rd8(esi.wrapping_add(MODE));
            if al == MODE_A || al == MODE_B {
                let s0f = f32::from_bits(s0c);
                if !(TWENTIETH > absf(s0f)) {
                    // Block L: unsigned 700-tick window on the global tick.
                    let edx = rd32(lf_checker_rt::relocated(GLOBAL_TICK));
                    let diff = edx.wrapping_sub(rd32(esi.wrapping_add(STAMP)));
                    let take_store = diff < TS_WINDOW; // MUT-SIGNEDNESS
                    if take_store {
                        wr32(esi.wrapping_add(STAMP), edx);
                    } else if rd8(esi.wrapping_add(OUT_COUNT)) == 0 {
                        wr32(esi.wrapping_add(STAMP), edx);
                    } else {
                        wr32(a3, 0);
                        let al2 = rd8(esi.wrapping_add(OUT_COUNT));
                        if al2 != 0 {
                            wr8(esi.wrapping_add(OUT_COUNT), al2.wrapping_sub(1));
                        }
                    }
                } else {
                    let mut flag = al == MODE_A;
                    if s0f < 0.0 {
                        flag = !flag;
                    }
                    if !flag {
                        let r5: u32 = lf_checker_rt::callee_cdecl!(5, u32, 0, 6);
                        wr8(esi.wrapping_add(OUT_COUNT), (r5 as u8).wrapping_add(0x0A));
                    } else {
                        let mut slot2: u32 = 0;
                        let u = vcall(esi, core::ptr::addr_of_mut!(slot2));
                        let l2 = add(mul(rdf(u), rdf(u)), mul(rdf(u.wrapping_add(4)), rdf(u.wrapping_add(4))))
                            .sqrt();
                        if !(l2 > TWELVE) {
                            let r5: u32 = lf_checker_rt::callee_cdecl!(5, u32, 0, 3);
                            wr8(esi.wrapping_add(OUT_COUNT), (r5 as u8).wrapping_add(3));
                        } else {
                            wr8(esi.wrapping_add(OUT_COUNT), 0);
                        }
                    }
                }
            }
        }

        // Tail: trailing flag scales two outputs by -1.
        if (rd8(esi.wrapping_add(FLIP_BYTE)) & 1) != 0 {
            wrf(a3, mul(rdf(a3), NEG_ONE));
            wrf(a4, mul(rdf(a4), NEG_ONE));
        }
        0
    }
}
