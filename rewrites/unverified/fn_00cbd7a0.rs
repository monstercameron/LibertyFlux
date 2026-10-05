// original: 0x00CBD7A0 ped_task_update_heading (proposed)

/// Ped task heading update: pick a facing bucket for the ped's current
/// motion and record the choice in the shared task table.
///
/// `this` is the task object. It reads the limit floats at `+0x14`/`+0x18`,
/// the rate at `+0x28`, the accumulator at `+0x30` (multiplied in place on
/// the task path), the flag byte at `+0x52`, the timestamp at `+0x74`, the
/// radius at `+0x70`, and the mover object at `+0x24` (heading delta from
/// `+0xAA4` minus `+0xAA0`, slot table at `+0x78`). `a0` is the command
/// object (flag word at `+0x378`, words at `+0`/`+4`); `a1` points at two
/// input floats; `a2` is unread scratch; `a3` is a threshold float whose
/// low byte doubles as a skip flag on the table path.
///
/// Behaviour: the heading delta is normalised through callee 1 and the
/// threshold is range-checked against the absolute inputs. When it is in
/// range the task path runs: a clamped rate is stored through the command
/// picked by callee 3, or slot objects from callee 4 are driven to -1.0
/// through callee 5, and the function returns 1. Otherwise the table path
/// runs: an angle in degrees comes back from callee 6, is negated and
/// wrapped into `[0, 360)`, and is placed into one of eight 45-degree
/// buckets scanned with the polled enables from callees 7 and 8. The
/// bucket is emitted either through callee 9 or written directly as a
/// 32-byte record (id, -1, three floats, zeros, tail float) at
/// `count*32` past the table base while the count is below 9; the
/// function returns 0.
///
/// All float comparisons use ordered SSE semantics: `comiss` branches map
/// to `!(a >= b)` style tests and the `lahf`/`test`/`jnp` idiom maps to
/// ordered equality (`a == b`, false for NaN). Float operation order is
/// the original's, pinned with `black_box` helpers. Callee 9's object
/// argument is the unrelocated table-base constant, passed through
/// unchanged. Original: 0x00CBD7A0 (thiscall, four stack words).
lf_checker_rt::export!(thiscall, rw_00CBD7A0(this: u32, a0: u32, a1: u32, _a2: u32, a3: u32) -> u32 {
    unsafe {
        const C_NORM: u32 = 1; // cdecl/1 f32st0: heading-delta normalise
        const C_GUARD: u32 = 2; // thiscall/0 al: timeout guard on mover
        const C_TASK: u32 = 3; // cdecl/5 u32: pick command object
        const C_SLOT: u32 = 4; // thiscall/1 u32: slot object lookup
        const C_APPLY: u32 = 5; // thiscall/1 ignored: drive slot to -1.0
        const C_ANGLE: u32 = 6; // cdecl/4 f32st0: angle in degrees
        const C_POLL1: u32 = 7; // cdecl/1 al: first bucket enable
        const C_POLL2: u32 = 8; // cdecl/0 al: second bucket enable
        const C_EMIT: u32 = 9; // thiscall/8 ignored: emit table record

        const V_INIT_HI: u32 = 0x00FE8AB8; // 4.0
        const V_INIT_LO: u32 = 0x00FE8830; // 0.5
        const V_ONE: u32 = 0x00FE88E8; // 1.0
        const V_TWO: u32 = 0x00FE8A24; // 2.0
        const V_THREE: u32 = 0x00FE8A94; // 3.0
        const V_FULL: u32 = 0x00FE8C1C; // 360.0
        const V_STEP: u32 = 0x00FE8B64; // 45.0
        const V_HALF_CIRC: u32 = 0x00FE8BA8; // 90.0
        const V_INV90: u32 = 0x00E8B7F4; // 1/90
        const V_BIAS: u32 = 0x00FE87B4; // 0.15
        const V_EPS: u32 = 0x00ED917C; // pi/1800
        const V_NEG_EPS: u32 = 0x00ED918C; // -pi/1800
        const V_G8: u32 = 0x00105143_0; // 8.0
        const V_G025: u32 = 0x00105143_4; // 0.25
        const V_G15: u32 = 0x00105143_8; // 1.5
        const V_G275: u32 = 0x00105143_C; // 2.75
        const V_GTIME: u32 = 0x011735B4;
        const V_GFLOAT: u32 = 0x0171BF8C;
        const V_GCOUNT: u32 = 0x0171BFB0;
        const V_TABLE_A: u32 = 0x00ED913C;
        const V_TABLE_B: u32 = 0x00ED915C;
        const RECORD_BASE: u32 = 0x0171BFB4;
        const EMIT_THIS: u32 = 0x0171BFB0; // unrelocated constant, as passed
        const FOUR_BITS: u32 = 0x4080_0000;
        const NEG_ONE_BITS: u32 = 0xBF80_0000;
        const ONE_BITS: u32 = 0x3F80_0000;
        const P2_BITS: u32 = 0x3E4C_CCCD; // 0.2
        const ABS_MASK: u32 = 0x7FFF_FFFF;
        const SIGN_MASK: u32 = 0x8000_0000;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write(v) }
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
        unsafe fn gf(va: u32) -> f32 {
            unsafe { lf_checker_rt::global::<f32>(va).read() }
        }
        #[inline(always)]
        unsafe fn gu(va: u32) -> u32 {
            unsafe { lf_checker_rt::global::<u32>(va).read() }
        }
        #[inline(always)]
        fn fabsf(x: f32) -> f32 {
            f32::from_bits(x.to_bits() & ABS_MASK)
        }
        #[inline(always)]
        fn fneg(x: f32) -> f32 {
            f32::from_bits(x.to_bits() ^ SIGN_MASK)
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
        // comiss branch conditions (jb/jbe/ja/jae), NaN-aware.
        #[inline(always)]
        fn below(a: f32, b: f32) -> bool {
            !(a >= b)
        }
        #[inline(always)]
        fn below_eq(a: f32, b: f32) -> bool {
            !(a > b)
        }
        #[inline(always)]
        fn above(a: f32, b: f32) -> bool {
            a > b
        }
        #[inline(always)]
        fn above_eq(a: f32, b: f32) -> bool {
            !(a < b)
        }
        // lahf/test/jnp idiom: ordered equality.
        #[inline(always)]
        fn ord_eq(a: f32, b: f32) -> bool {
            a == b
        }
        #[inline(always)]
        unsafe fn emit(arg0: u32, arg1: f32, arg7: f32) {
            unsafe {
                let _: u32 = lf_checker_rt::callee_thiscall!(
                    C_EMIT, u32, EMIT_THIS, arg0, arg1.to_bits(), 0, 0, 0, 0, 0xFFFF_FFFF,
                    arg7.to_bits()
                );
            }
        }
        #[inline(always)]
        unsafe fn store_record(idx: u32, id: u32, mid: f32, tail: f32) {
            unsafe {
                let base = lf_checker_rt::relocated(RECORD_BASE).wrapping_add(idx.wrapping_shl(5));
                ((base) as *mut u32).write(id);
                ((base.wrapping_add(4)) as *mut u32).write(0xFFFF_FFFF);
                ((base.wrapping_add(8)) as *mut u32).write(mid.to_bits());
                ((base.wrapping_add(12)) as *mut u32).write(0);
                ((base.wrapping_add(16)) as *mut u32).write(0);
                ((base.wrapping_add(20)) as *mut u32).write(0);
                ((base.wrapping_add(24)) as *mut u32).write(0);
                ((base.wrapping_add(28)) as *mut u32).write(tail.to_bits());
            }
        }
        #[inline(always)]
        unsafe fn slot(ecx: u32, arg: u32) -> u32 {
            unsafe { lf_checker_rt::callee_thiscall!(C_SLOT, u32, ecx, arg) }
        }
        #[inline(always)]
        unsafe fn apply(ecx: u32) {
            unsafe {
                let _: u32 = lf_checker_rt::callee_thiscall!(C_APPLY, u32, ecx, NEG_ONE_BITS);
            }
        }
        #[inline(always)]
        unsafe fn task(a0: u32, a1: u32, a2: u32, a3: u32, a4: u32) -> u32 {
            unsafe { lf_checker_rt::callee_cdecl!(C_TASK, u32, a0, a1, a2, a3, a4) }
        }

        // ---- entry ----
        let init: f32 = if rd8(this.wrapping_add(0x52)) & 1 != 0 {
            gf(V_INIT_HI)
        } else {
            gf(V_INIT_LO)
        };
        let fabs0 = fabsf(rdf(a1));
        let fabs1 = fabsf(rdf(a1.wrapping_add(4)));
        let mid = rd32(this.wrapping_add(0x24));
        let diff = sub(rdf(mid.wrapping_add(0xAA4)), rdf(mid.wrapping_add(0xAA0)));
        let norm: f32 = lf_checker_rt::callee_cdecl!(C_NORM, f32, diff.to_bits());
        let arg3f = f32::from_bits(a3);
        let flag_a: u8 = if below(arg3f, fabs0) {
            0
        } else if above_eq(arg3f, fabs1) {
            1
        } else {
            0
        };
        let t14 = fabsf(rdf(this.wrapping_add(0x14)));
        let flag_b: u8 = if below(arg3f, t14) {
            0
        } else if above_eq(arg3f, fabsf(rdf(this.wrapping_add(0x18)))) {
            1
        } else {
            0
        };
        let gtime = gu(V_GTIME);
        let dt = gtime.wrapping_sub(rd32(this.wrapping_add(0x74)));
        let cl: u8 = if dt <= 250 {
            0
        } else {
            let r: u32 =
                lf_checker_rt::callee_thiscall!(C_GUARD, u32, rd32(this.wrapping_add(0x24)));
            if (r as u8) == 0 { 0 } else { 1 }
        };
        let edx_saved = rd32(rd32(this.wrapping_add(0x24)).wrapping_add(0x78));
        if flag_a == 0 {
            // ================= table path =================
            let q2 = rdf(a1.wrapping_add(4));
            let q1 = rdf(a1);
            let mut x1 = add(mul(q1, q1), mul(q2, q2));
            x1 = core::hint::black_box(x1).sqrt();
            let t70 = rdf(this.wrapping_add(0x70));
            if !above(t70, x1) {
                x1 = t70;
            }
            let one = gf(V_ONE);
            let two = gf(V_TWO);
            let three = gf(V_THREE);
            let x3: f32 = if below(one, x1) {
                if below(x1, two) { sub(x1, one) } else { one }
            } else {
                0.0
            };
            let mut a0slot: f32 = if below_eq(one, x1) { sub(one, x3) } else { x1 };
            let f8: f32 = if below_eq(one, x1) {
                0.0
            } else {
                sub(one, a0slot)
            };
            let mut o14: f32 = one;
            if below_eq(x1, two) {
                o14 = 0.0;
            } else if !above_eq(x1, three) {
                o14 = sub(x1, two);
            }
            let ang_raw: f32 = lf_checker_rt::callee_cdecl!(
                C_ANGLE, f32, q1.to_bits(), q2.to_bits(), 0, 0
            );
            let mut o28: f32 = 0.0;
            let mut ang = fneg(ang_raw);
            if above(0.0, ang) {
                ang = add(ang, gf(V_FULL));
            }
            let mut lo = 0.0f32;
            let mut hi = gf(V_STEP);
            let mut a2slot: f32 = x3;
            let a3lo: u8 = (a3 & 0xFF) as u8;
            let mut esi: u32 = 0;
            loop {
                let p1: u32 = lf_checker_rt::callee_cdecl!(C_POLL1, u32, 1);
                let al: u8 = if (p1 as u8) != 0 {
                    1
                } else {
                    let p2: u32 = lf_checker_rt::callee_cdecl!(C_POLL2, u32);
                    if (p2 as u8) != 0 { 1 } else { 0 }
                };
                let mut o1: f32;
                let mut x6: f32;
                if !ord_eq(o14, 0.0) && al == 0 {
                    if esi == 0 || esi == 1 || esi == 6 || esi == 7 {
                        let mut t = if esi == 0 || esi == 1 {
                            sub(gf(V_HALF_CIRC), ang)
                        } else {
                            add(sub(ang, gf(V_FULL)), gf(V_HALF_CIRC))
                        };
                        t = mul(t, gf(V_INV90));
                        t = mul(t, o14);
                        t = add(t, gf(V_BIAS));
                        // clamp into o28, then the o28-zero check
                        let direct: bool;
                        if below_eq(one, t) {
                            o28 = one;
                            direct = false;
                        } else if above(t, 0.0) {
                            o28 = t;
                            direct = false;
                        } else {
                            o28 = 0.0;
                            direct = true;
                        }
                        if direct {
                            x6 = a0slot;
                            o1 = a2slot;
                        } else if !ord_eq(o28, 0.0) {
                            o1 = sub(one, o28);
                            x6 = 0.0;
                            a0slot = 0.0;
                            a2slot = o1;
                        } else {
                            x6 = a0slot;
                            o1 = a2slot;
                        }
                    } else {
                        o28 = 0.0;
                        x6 = a0slot;
                        o1 = a2slot;
                    }
                } else {
                    x6 = a0slot;
                    o1 = a2slot;
                }
                // bucket tests
                if ord_eq(ang, lo) {
                    // angle on the low edge: direct record writes
                    let gfloat = gf(V_GFLOAT);
                    let mut count = gu(V_GCOUNT);
                    if !ord_eq(x6, 0.0) && count < 9 {
                        let id = gu(V_TABLE_A.wrapping_add(esi.wrapping_mul(4)));
                        store_record(count, id, x6, gfloat);
                        count = count.wrapping_add(1);
                        wr32(lf_checker_rt::relocated(V_GCOUNT), count);
                    }
                    if !ord_eq(o1, 0.0) && a3lo == 0 && count < 9 {
                        let id = gu(V_TABLE_B.wrapping_add(esi.wrapping_mul(4)));
                        store_record(count, id, o1, gfloat);
                        count = count.wrapping_add(1);
                        wr32(lf_checker_rt::relocated(V_GCOUNT), count);
                    }
                    if !ord_eq(o28, 0.0) && a3lo == 0 && count < 9 {
                        store_record(count, 0x32, o28, gfloat);
                        count = count.wrapping_add(1);
                    }
                    wr32(lf_checker_rt::relocated(V_GCOUNT), count);
                    if !ord_eq(f8, 0.0) && count < 9 {
                        store_record(count, 0x0F, f8, 1.0);
                        wr32(lf_checker_rt::relocated(V_GCOUNT), count.wrapping_add(1));
                    }
                    return 0;
                }
                if ord_eq(ang, hi) {
                    // angle on the high edge: one direct write plus emits
                    let gfloat = gf(V_GFLOAT);
                    let mut count = gu(V_GCOUNT);
                    if !ord_eq(x6, 0.0) && count < 9 {
                        let id = gu(V_TABLE_A
                            .wrapping_add((esi.wrapping_add(1) & 7).wrapping_mul(4)));
                        store_record(count, id, x6, gfloat);
                        count = count.wrapping_add(1);
                        wr32(lf_checker_rt::relocated(V_GCOUNT), count);
                    }
                    if !ord_eq(o1, 0.0) && a3lo == 0 {
                        let id = gu(V_TABLE_B
                            .wrapping_add((esi.wrapping_add(1) & 7).wrapping_mul(4)));
                        emit(id, o1, gfloat);
                    }
                    if !ord_eq(o28, 0.0) && a3lo == 0 {
                        emit(0x32, o28, gfloat);
                    }
                    if !ord_eq(f8, 0.0) && count < 9 {
                        store_record(count, 0x0F, f8, 1.0);
                        wr32(lf_checker_rt::relocated(V_GCOUNT), count.wrapping_add(1));
                    }
                    return 0;
                }
                if below_eq(ang, lo) || !above(hi, ang) {
                    // outside the bucket: advance and scan on
                    let old_hi = hi;
                    hi = add(hi, gf(V_STEP));
                    lo = old_hi;
                    esi = esi.wrapping_add(1);
                    if esi >= 8 {
                        let count = gu(V_GCOUNT);
                        if !ord_eq(f8, 0.0) && count < 9 {
                            store_record(count, 0x0F, f8, 1.0);
                            wr32(lf_checker_rt::relocated(V_GCOUNT), count.wrapping_add(1));
                        }
                        return 0;
                    }
                    continue;
                }
                // angle inside (lo, hi): interpolate and emit
                let ft: f32 = if ord_eq(lo, hi) {
                    one
                } else {
                    core::hint::black_box(sub(ang, lo))
                        / core::hint::black_box(sub(hi, lo))
                };
                let gfloat = gf(V_GFLOAT);
                if !ord_eq(x6, 0.0) {
                    emit(
                        gu(V_TABLE_A.wrapping_add(esi.wrapping_mul(4))),
                        mul(sub(one, ft), x6),
                        gfloat,
                    );
                    emit(
                        gu(V_TABLE_A
                            .wrapping_add((esi.wrapping_add(1) & 7).wrapping_mul(4))),
                        mul(ft, a0slot),
                        gfloat,
                    );
                }
                let o1b = a2slot;
                if !ord_eq(o1b, 0.0) && a3lo == 0 {
                    emit(
                        gu(V_TABLE_B.wrapping_add(esi.wrapping_mul(4))),
                        mul(sub(one, ft), o1b),
                        gfloat,
                    );
                    emit(
                        gu(V_TABLE_B
                            .wrapping_add((esi.wrapping_add(1) & 7).wrapping_mul(4))),
                        mul(ft, a2slot),
                        gfloat,
                    );
                }
                if !ord_eq(o28, 0.0) && a3lo == 0 {
                    emit(0x32, o28, gfloat);
                }
                let count = gu(V_GCOUNT);
                if !ord_eq(f8, 0.0) && count < 9 {
                    store_record(count, 0x0F, f8, 1.0);
                    wr32(lf_checker_rt::relocated(V_GCOUNT), count.wrapping_add(1));
                }
                return 0;
            }
        } else {
            // ================= task path =================
            let f378 = rd32(a0.wrapping_add(0x378));
            if ((f378 >> 8) & 1) != 0 && cl == 0 {
                let t28 = rdf(this.wrapping_add(0x28));
                if !below_eq(t28, gf(V_EPS)) {
                    let t = fabsf(mul(gf(V_G8), norm));
                    let g025 = gf(V_G025);
                    let cv = if above(g025, t) {
                        g025
                    } else if below_eq(t, gf(V_G15)) {
                        t
                    } else {
                        gf(V_G15)
                    };
                    let r: u32 = task(
                        edx_saved,
                        rd32(a0),
                        0x10,
                        FOUR_BITS,
                        rd32(a0.wrapping_add(4)),
                    );
                    if r != 0 {
                        wrf(r.wrapping_add(0x54), cv);
                    }
                    wrf(
                        this.wrapping_add(0x30),
                        mul(rdf(this.wrapping_add(0x30)), gf(V_G275)),
                    );
                    return 1;
                } else if !below_eq(gf(V_NEG_EPS), t28) {
                    let t = fabsf(mul(gf(V_G8), norm));
                    let g025 = gf(V_G025);
                    let cv = if above(g025, t) {
                        g025
                    } else if below_eq(t, gf(V_G15)) {
                        t
                    } else {
                        gf(V_G15)
                    };
                    let r: u32 = task(
                        edx_saved,
                        rd32(a0),
                        0x11,
                        FOUR_BITS,
                        rd32(a0.wrapping_add(4)),
                    );
                    if r != 0 {
                        wrf(r.wrapping_add(0x54), cv);
                    }
                    wrf(
                        this.wrapping_add(0x30),
                        mul(rdf(this.wrapping_add(0x30)), gf(V_G275)),
                    );
                    return 1;
                } else {
                    let p10: u32 = slot(edx_saved, 0x10);
                    if p10 != 0 {
                        apply(p10);
                    }
                    let p11: u32 = slot(edx_saved, 0x11);
                    if p11 != 0 {
                        apply(p11);
                    }
                }
            }
            let f2 = rd32(a0.wrapping_add(0x378));
            if ((f2 >> 8) & 1) != 0 {
                let m = rd32(this.wrapping_add(0x24));
                wr32(m.wrapping_add(0xAA0), rd32(m.wrapping_add(0xAA4)));
            }
            let q10: u32 = slot(edx_saved, 0x10);
            let q11: u32 = slot(edx_saved, 0x11);
            let f3 = rd32(a0.wrapping_add(0x378));
            let (edx2, a3s): (u32, u32) = if ((f3 >> 8) & 1) != 0 {
                (q10, q11)
            } else {
                if q10 != 0 {
                    apply(q10);
                }
                if q11 != 0 {
                    apply(q11);
                }
                (q11, 0)
            };
            let f4 = rd32(a0.wrapping_add(0x378));
            let bit10 = ((f4 >> 10) & 1) != 0;
            let ecx2: u32 = if bit10 { rd32(a0) } else { 0xFFFF_FFFF };
            let eax2: u32 = if bit10 {
                rd32(a0.wrapping_add(4))
            } else {
                0xFFFF_FFFF
            };
            let to_alt = cl != 0
                || rd8(this.wrapping_add(0x52)) & 1 != 0
                || (flag_b != 0 && edx2 == 0 && a3s == edx2);
            if !to_alt {
                if !(ecx2 == 0xFFFF_FFFF && eax2 == 0xFFFF_FFFF) {
                    let _: u32 = task(edx_saved, ecx2, 0x4B, init.to_bits(), eax2);
                }
            } else {
                let r: u32 = slot(edx_saved, 0x4B);
                if r == 0 {
                    let _: u32 = task(
                        edx_saved,
                        rd32(a0),
                        0x0F,
                        init.to_bits(),
                        rd32(a0.wrapping_add(4)),
                    );
                } else if ((rd8(r.wrapping_add(0x46)) >> 2) & 1) == 0
                    && (ecx2 != 0xFFFF_FFFF || eax2 != ecx2)
                {
                    let _: u32 = task(edx_saved, ecx2, 0x4B, ONE_BITS, eax2);
                } else {
                    let _: u32 = task(
                        edx_saved,
                        rd32(a0),
                        0x0F,
                        P2_BITS,
                        rd32(a0.wrapping_add(4)),
                    );
                }
            }
            wrf(
                this.wrapping_add(0x30),
                mul(rdf(this.wrapping_add(0x30)), gf(V_G275)),
            );
            return 1;
        }
    }
});
