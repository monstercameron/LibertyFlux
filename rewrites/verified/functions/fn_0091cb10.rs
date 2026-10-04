// original: 0x0091cb10 input_event_lookup (proposed)

/// Look up an input event by `(kind, sub)` and report it through `out`.
///
/// `kind` selects the family: 0 probes one code, 1-2 take the mouse string,
/// 3 maps button codes, 5 maps wheel directions, 14 reads two codes at once;
/// any other kind yields `NONE` (0xff). `sub` is the code within the family
/// (for kind 14 its two bytes are looked up separately). `s` is a word
/// string used as scratch by the callees and may be null on paths that never
/// pass it on; several paths return `NONE` when it is null. `out`, when
/// non-null, receives dwords at `+0`/`+4` and tag bytes at `+0x110`/`+0x111`
/// (1 = direct code, 2 = string copied by the callee); a null `out` takes
/// the string-appending paths instead. `f1`/`f2` (only their low bytes)
/// steer kind 14 between its one-code and two-code forms.
///
/// Work is delegated to scripted callees: the code resolver (callee 1, whose
/// zero answer means "absent" and whose answers above 0xff, compared signed,
/// are stored directly), the name object (callee 2, thiscall) and the
/// word-string copy/append pair (callees 3-9, one id per call shape so each
/// stub writes exactly what the real copy would). The gated names in kind 3
/// choose their plain variant unless the override byte is set or the mode
/// byte is set with the layout dword at 0x45. Every path ends by checking
/// the stack cookie (callee 10, which preserves registers) and returns the
/// code, 0x31 after a string call, 1 for the kind-0 fallback, or `NONE`.
///
/// String and object addresses are read-only image constants, embedded by
/// file address and relocated at load; the mode/layout/override bytes live
/// in writable game data. Cdecl, six stack words, result in `eax`.
lf_checker_rt::export!(cdecl, rw_0091cb10(kind: u32, sub: u32, s: u32, out: u32, f1: u32, f2: u32) -> u32 {
    unsafe {
        const NONE: u32 = 0xFF;
        const DONE: u32 = 0x31;
        const OBJ: u32 = 0x0116BFF0;
        const COOKIE: u32 = 0x01057FB4;
        const FLAG_MODE: u32 = 0x011609F6;
        const LAYOUT: u32 = 0x01160C40;
        const OVERRIDE: u32 = 0x016334C0;
        const LAYOUT_WANTED: u32 = 0x45;
        const NEG1: u32 = 0xFFFF_FFFF;
        const S_PLS_NONE: u32 = 0x00E85DDC;
        const S_MS_MOUSE: u32 = 0x00E85DE8;
        const S_MS_LEFT2: u32 = 0x00E85DF4;
        const S_MS_LEFT: u32 = 0x00E85E00;
        const S_MS_RIGHT2: u32 = 0x00E85E08;
        const S_MS_RIGHT: u32 = 0x00E85E14;
        const S_MS_MID2: u32 = 0x00E85E20;
        const S_MS_MID: u32 = 0x00E85E28;
        const S_MS_BUT1: u32 = 0x00E85E30;
        const S_MS_BUT2: u32 = 0x00E85E38;
        const S_MS_BUT3: u32 = 0x00E85E40;
        const S_MS_BUT4: u32 = 0x00E85E48;
        const S_MS_BUT5: u32 = 0x00E85E50;
        const S_AXISW_U: u32 = 0x00E85E58;
        const S_AXISW_D: u32 = 0x00E85E60;

        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
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
        unsafe fn aa(a0: u32, a1: u32, a2: u32) -> u32 {
            unsafe { lf_checker_rt::callee_cdecl!(1, u32, a0, a1, a2) }
        }
        #[inline(always)]
        unsafe fn aae(a0: u32, a1: u32, a2: u32) -> u32 {
            // Same resolver, other stub id: these sites pass the stack slot,
            // whose address is skipped and contents snapped.
            unsafe { lf_checker_rt::callee_cdecl!(11, u32, a0, a1, a2) }
        }
        #[inline(always)]
        unsafe fn tc(name: u32) -> u32 {
            unsafe {
                lf_checker_rt::callee_thiscall!(
                    2, u32, lf_checker_rt::relocated(OBJ), lf_checker_rt::relocated(name)
                )
            }
        }
        #[inline(always)]
        unsafe fn ef_a(dst: u32, src: u32) {
            unsafe { lf_checker_rt::callee_cdecl!(3, u32, dst, src, NEG1) };
        }
        #[inline(always)]
        unsafe fn ef_t(dst: u32, src: u32) {
            unsafe { lf_checker_rt::callee_cdecl!(4, u32, dst, src, NEG1) };
        }
        #[inline(always)]
        unsafe fn ef_e10(dst: u32, src: u32) {
            unsafe { lf_checker_rt::callee_cdecl!(5, u32, dst, src, NEG1) };
        }
        #[inline(always)]
        unsafe fn ef_es(dst: u32, src: u32) {
            unsafe { lf_checker_rt::callee_cdecl!(6, u32, dst, src, NEG1) };
        }
        #[inline(always)]
        unsafe fn ef_e50(dst: u32, src: u32) {
            unsafe { lf_checker_rt::callee_cdecl!(7, u32, dst, src, NEG1) };
        }
        #[inline(always)]
        unsafe fn ee1(dst: u32, src: u32) {
            unsafe { lf_checker_rt::callee_cdecl!(8, u32, dst, src) };
        }
        #[inline(always)]
        unsafe fn ee2(dst: u32, src: u32) {
            unsafe { lf_checker_rt::callee_cdecl!(9, u32, dst, src) };
        }
        #[inline(always)]
        unsafe fn gated(plain: u32, alt: u32) -> u32 {
            unsafe {
                let flag = rd8(lf_checker_rt::relocated(FLAG_MODE));
                let layout = rd32(lf_checker_rt::relocated(LAYOUT));
                let ovr = rd8(lf_checker_rt::relocated(OVERRIDE));
                if ovr != 0 || (flag != 0 && layout == LAYOUT_WANTED) {
                    plain
                } else {
                    alt
                }
            }
        }

        let ck = rd32(lf_checker_rt::relocated(COOKIE));
        let r = match kind {
            0 => {
                let b = aa(sub, s, 0);
                if b == 0 {
                    if sub != 0 {
                        NONE
                    } else {
                        let t = tc(S_PLS_NONE);
                        ef_t(s, t);
                        1
                    }
                } else if out == 0 {
                    b
                } else if (b as i32) <= 0xFF {
                    wr8(out + 0x110, 2);
                    ef_a(out + 0x10, s);
                    b
                } else {
                    wr32(out, b);
                    wr8(out + 0x110, 1);
                    b
                }
            }
            1 | 2 => {
                if s == 0 {
                    NONE
                } else {
                    let t = tc(S_MS_MOUSE);
                    ef_t(s, t);
                    DONE
                }
            }
            3 => {
                let name = match sub {
                    1 => gated(S_MS_LEFT2, S_MS_LEFT),
                    2 => gated(S_MS_RIGHT2, S_MS_RIGHT),
                    4 => gated(S_MS_MID2, S_MS_MID),
                    8 => S_MS_BUT1,
                    0x10 => S_MS_BUT2,
                    0x12 => S_MS_BUT3,
                    0x14 => S_MS_BUT4,
                    0x18 => S_MS_BUT5,
                    _ => 0,
                };
                if name == 0 || s == 0 {
                    NONE
                } else {
                    let t = tc(name);
                    ef_t(s, t);
                    DONE
                }
            }
            5 => {
                let name = match sub {
                    0 => S_AXISW_U,
                    1 => S_AXISW_D,
                    _ => 0,
                };
                if name == 0 || s == 0 {
                    NONE
                } else {
                    let t = tc(name);
                    ef_t(s, t);
                    DONE
                }
            }
            14 => {
                if (f1 & 0xFF) == 0 {
                    if s == 0 {
                        NONE
                    } else {
                        let mut local: u32 = 0;
                        let lp = &mut local as *mut u32 as u32;
                        let t1 = aae(sub & 0xFF, lp, 1);
                        if t1 == 0 {
                            NONE
                        } else {
                            if out != 0 {
                                if (t1 as i32) > 0xFF {
                                    wr8(out + 0x110, 1);
                                    wr32(out, t1);
                                } else {
                                    wr8(out + 0x110, 2);
                                    ef_e10(out + 0x10, lp);
                                }
                            } else {
                                ef_es(s, lp);
                                local = 0x2F;
                                ee1(s, lp);
                            }
                            let t2 = aae((sub >> 8) & 0xFF, lp, 1);
                            if t2 == 0 {
                                NONE
                            } else if out != 0 {
                                if (t2 as i32) > 0xFF {
                                    wr32(out + 4, t2);
                                    wr8(out + 0x111, 1);
                                } else {
                                    wr8(out + 0x111, 2);
                                    ef_e50(out + 0x50, lp);
                                }
                                DONE
                            } else {
                                ee2(s, lp);
                                DONE
                            }
                        }
                    }
                } else if s == 0 {
                    NONE
                } else if (f2 & 0xFF) != 0 {
                    if aa(sub & 0xFF, s, 0) == 0 { NONE } else { DONE }
                } else if aa((sub >> 8) & 0xFF, s, 0) == 0 {
                    NONE
                } else {
                    DONE
                }
            }
            _ => NONE,
        };
        lf_checker_rt::callee_thiscall!(10, u32, ck);
        r
    }
});
