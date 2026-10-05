// original: 0x00b4e880 ped_task_gate_large (proposed)

/// Gate a ten-slot scan for `this`, then sweep the slots.
///
/// `this` carries a chain pointer at `+0x6c`, a frame pointer at `+0x20`
/// and an avoid pointer at `+0x118`. A chain of gates must all pass, else
/// byte 0 is returned: the mode global differs from 1, two globals agree,
/// a third differs from `0x12`, a readiness call returns at least 1 (below
/// that a logging pair may run first), an active chain needs `arg0`
/// nonzero, a feature call (when set with an active chain, a triple call
/// must also be nonzero, else byte 0 returns directly), two single calls
/// must be zero, another zero, and a frame call nonzero. Two getter calls
/// then feed a float-returning vtable slot (negative fails, NaN passes) and
/// a flag slot whose follow-up must be nonzero.
///
/// Nonzero `arg0` or `arg1` bypasses the rest with byte 1. Otherwise ten
/// frame slots are zeroed and a scan call fills the upper seven; a nonzero
/// scan result also bypasses with 1. A second getter plus a vtable check
/// set the sweep's excluded pointer and tag, then each slot is skipped when
/// null, `this`, the avoid pointer, or the excluded pointer; kind 4 (from
/// bits of `+0x28`) needs a clear tag and pool/byte checks, kind 2 with a
/// set tag measures a vtable-supplied vector against `0.1` (below skips),
/// and every surviving slot runs a nine-word report plus a verdict call
/// whose nonzero result returns 1 at once. Surviving all ten returns 1.
/// The stack cookie is omitted (its check is stubbed and unlogged); two
/// frame pointers passed to callees are skipped, pre-write zeroes
/// snapshotted for the slot region. Thiscall, two stack words, byte result.
lf_checker_rt::export!(thiscall, rw_00b4e880(this: u32, a0: u32, a1: u32) -> u32 {
    unsafe {
        const CHAIN: u32 = 0x6C;
        const FRAME: u32 = 0x20;
        const AVOID: u32 = 0x118;
        const CHAIN_FLAG: u32 = 0x0E;
        const G_MODE: u32 = 0x11F7060;
        const G_A: u32 = 0x12088B4;
        const G_B: u32 = 0xF1C040;
        const G_C: u32 = 0x1037720;
        const G_D: u32 = 0x18B6F1C;
        const G_COOKIE: u32 = 0x1057FB4;
        const LOG_FMT: u32 = 0xEAEA60;
        const D2_IMM: u32 = 0x1908EF0;
        const POOL: u32 = 0x1295CD8;
        const C_FAR: u32 = 0xFE879C;
        const C_W: u32 = 0xE9D494;
        const SCAN_F: u32 = 0x3E99999A;
        const ONE_F: u32 = 0x3F800000;
        const READY: u32 = 1;
        const LOG_A: u32 = 2;
        const LOG_B: u32 = 3;
        const FEATURE: u32 = 4;
        const TRIPLE: u32 = 5;
        const SINGLE: u32 = 6;
        const PROBE_A: u32 = 7;
        const PROBE_B: u32 = 8;
        const FRAME_CALL: u32 = 9;
        const GETTER: u32 = 10;
        const MEASURE: u32 = 11;
        const FLAGSLOT: u32 = 12;
        const FOLLOW: u32 = 13;
        const SCAN: u32 = 14;
        const VT4: u32 = 15;
        const VEC: u32 = 16;
        const REPORT: u32 = 17;
        const VERDICT: u32 = 18;
        const COOKIE: u32 = 19;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u32 {
            unsafe { (a as *const u8).read() as u32 }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u32 {
            unsafe { (a as *const u16).read_unaligned() as u32 }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn g(a: u32) -> u32 {
            unsafe { rd32(lf_checker_rt::relocated(a)) }
        }

        let r: u32 = (|| -> u32 {
        let _cookie: u32 = g(G_COOKIE);
        if g(G_MODE) == 1 {
            return 0;
        }
        if g(G_A) != g(G_B) {
            return 0;
        }
        if g(G_C) == 0x12 {
            return 0;
        }
        let ready: u32 = lf_checker_rt::callee_thiscall!(READY, u32, g(G_D));
        if (ready as i32) < 1 {
            let ch = rd32(this + CHAIN);
            if ch == 0 || rd8(ch + CHAIN_FLAG) == 0 {
                return 0;
            }
            let la: u32 = lf_checker_rt::callee_thiscall!(LOG_A, u32, ch);
            let mut scratch = 0u32;
            lf_checker_rt::callee_cdecl!(
                LOG_B, u32, &mut scratch as *mut u32 as u32,
                lf_checker_rt::relocated(LOG_FMT), la
            );
            return 0;
        }
        let ch0 = rd32(this + CHAIN);
        if ch0 != 0 && rd8(ch0 + CHAIN_FLAG) != 0 && a0 == 0 {
            return 0;
        }
        let feat: u32 = lf_checker_rt::callee_cdecl!(FEATURE, u32,);
        if feat as u8 != 0 {
            let ch1 = rd32(this + CHAIN);
            if ch1 != 0 && rd8(ch1 + CHAIN_FLAG) != 0 {
                let tr: u32 = lf_checker_rt::callee_cdecl!(TRIPLE, u32, 1, 0, 0);
                if tr as u8 == 0 {
                    return 0;
                }
            }
        }
        let s1: u32 = lf_checker_rt::callee_thiscall!(
            SINGLE, u32, lf_checker_rt::relocated(D2_IMM), 1, 0
        );
        if s1 as u8 == 0 {
            return 0;
        }
        let ch2 = rd32(this + CHAIN);
        if ch2 != 0 {
            let pa: u32 = lf_checker_rt::callee_thiscall!(PROBE_A, u32, ch2);
            if pa as u8 != 0 {
                return 0;
            }
        }
        let ch3 = rd32(this + CHAIN);
        if ch3 != 0 {
            let pb: u32 = lf_checker_rt::callee_thiscall!(PROBE_B, u32, ch3, 0x20);
            if pb as u8 != 0 {
                return 0;
            }
        }
        let frame = rd32(this + FRAME);
        let fc: u32 = lf_checker_rt::callee_cdecl!(FRAME_CALL, u32, frame.wrapping_add(0x30));
        if fc as u8 == 0 {
            return 0;
        }
        let g5: u32 = lf_checker_rt::callee_thiscall!(GETTER, u32, this);
        if g5 != 0 {
            let mf: extern "thiscall" fn(u32, u32) -> f32 = core::mem::transmute(
                rd32(rd32(g5) + 0x1C) as usize,
            );
            let f = mf(g5, this);
            if f < 0.0 {
                return 0;
            }
        }
        let fl: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(rd32(rd32(this) + 0xD4) as usize);
        let f2 = fl(this, ONE_F);
        let fol: u32 = lf_checker_rt::callee_thiscall!(FOLLOW, u32, f2);
        if fol as u8 == 0 {
            return 0;
        }
        if a0 != 0 || a1 != 0 {
            return 1;
        }
        let mut slots = [0u32; 10];
        let sc: u32 = lf_checker_rt::callee_cdecl!(
            SCAN, u32, frame.wrapping_add(0x30), SCAN_F, 1, 0xA,
            slots.as_mut_ptr().wrapping_add(3) as u32, 1, 1, 1
        );
        if sc as u8 != 0 {
            return 1;
        }
        let g6: u32 = lf_checker_rt::callee_thiscall!(GETTER, u32, this);
        let (mut xc, mut dl): (u32, u32) = (0, 0);
        if g6 != 0 {
            let v4: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(rd32(rd32(g6) + 4) as usize);
            if v4(g6) == 0 {
                xc = rd32(g6 + 0x1C);
                dl = 1;
            }
        }
        let avoid = rd32(this + AVOID);
        let far = rdf(lf_checker_rt::relocated(C_FAR));
        let cw = rd32(lf_checker_rt::relocated(C_W));
        let mut i = 0u32;
        while i < 10 {
            let s = slots[i as usize];
            i += 1;
            if s == 0 || s == this || s == avoid || s == xc {
                continue;
            }
            let kind = (rd32(s + 0x28) >> 6) & 0xF;
            if kind == 4 {
                if dl != 0 {
                    continue;
                }
                if (rd32(s + 0x24) >> 2) & 1 == 0 {
                    let idx = rd16(s + 0x2E) as u16 as i16 as i32 as u32;
                    let p = rd32(
                        lf_checker_rt::relocated(POOL)
                            .wrapping_add(idx.wrapping_mul(4)),
                    );
                    if rd32(p + 0x40) & 0x80000 == 0 {
                        // fall through to verdict calls
                    } else if rd8(s + 0x22A) != 2 {
                        continue;
                    }
                } else if rd8(s + 0x22A) != 2 {
                    continue;
                }
            } else if kind == 2 {
                if dl == 0 {
                    // fall through to verdict calls
                } else {
                    let vf: extern "thiscall" fn(u32, u32) -> u32 = core::mem::transmute(
                        rd32(rd32(s) + 0xEC) as usize,
                    );
                    let mut v = [0u32; 3];
                    let vr = vf(s, v.as_mut_ptr() as u32);
                    let d2 = add(
                        add(
                            mul(rdf(vr + 0), rdf(vr + 0)),
                            mul(rdf(vr + 4), rdf(vr + 4)),
                        ),
                        mul(rdf(vr + 8), rdf(vr + 8)),
                    );
                    if far > d2 {
                        continue;
                    }
                }
            }
            lf_checker_rt::callee_thiscall!(
                REPORT, u32, 0, s, slots[4], cw, 1, 0, 0, 0, 0, 0
            );
            let vd: u32 = lf_checker_rt::callee_thiscall!(VERDICT, u32, 0, 0);
            if vd as u8 != 0 {
                return 1;
            }
        }
        1
        })();
        lf_checker_rt::callee_cdecl!(COOKIE, u32,);
        r
    }
});
