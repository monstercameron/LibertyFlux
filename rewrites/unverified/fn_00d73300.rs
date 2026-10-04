// original: 0x00d73300 montage_menu_navigate (proposed)

/// Advance the montage-menu state machine by one navigation step.
///
/// `this` is the menu object (inner state at `+0x04`, selected row at
/// `+0x3c`, toggle flags at `+0x50..=0x54`); `delta` is the signed step.
/// The probe pair resolves the current entry (`esi`: bytes at `+0x00..=0x09`,
/// flag word at `+0x0c`, fields at `+0x10`/`+0x20`/`+0x24`) and the reference
/// entry (`edi`, may be null). Two latch flags record whether the current
/// entry's first byte was 9 or 10. The row selects one of seventeen arms
/// (rows outside 1..=17 return the reference probe's answer unchanged):
/// toggles of the flag bytes, modular stepping of the small fields with
/// wrap/clamp rules, div/mod navigation with div-by-constant, a percent
/// computation through the wide apply call, a resolve-and-compare through
/// the pool pair, and clamped stepping of the wide fields against globals.
/// Arms that change a compared field refresh it through the touch call.
/// Every arm first logs through the trace call. Returns the last callee
/// answer on the taken path.
///
/// Original: 0x00d73300 (thiscall, one stack word). All divisions are by
/// nonzero constants with fitting quotients, so no fault path exists; the
/// two power-of-two remainder sequences are C-truncated `% 4`. The row-15
/// arm loops while its value is 1 or 2, which never terminates for a
/// nonnegative multiple-of-4 step: such steps are excluded from the proof
/// inputs (see the contract) and noted in the result.
lf_checker_rt::export!(thiscall, rw_00d73300(this: u32, delta_u: u32) -> u32 {
    unsafe {
        const INNER_OFF: u32 = 0x04;
        const ROW_OFF: u32 = 0x3c;
        const FLAG_A_OFF: u32 = 0x50;
        const FLAG_B_OFF: u32 = 0x51;
        const FLAG_C_OFF: u32 = 0x52;
        const FLAG_D_OFF: u32 = 0x53;
        const FLAG_E_OFF: u32 = 0x54;
        const TRACE_THIS: u32 = 0x01176888;
        const NESTED_THIS: u32 = 0x0116bff0;
        const POOL_THIS: u32 = 0x0103e498;
        const WIDE_THIS: u32 = 0x01033130;
        const LIMIT_G: u32 = 0x01797674;
        const CLAMP_A_G: u32 = 0x01176d2c;
        const CLAMP_B_G: u32 = 0x01176d28;
        const ENTRY_PROBE: u32 = 1;
        const REF_PROBE: u32 = 2;
        const TRACE: u32 = 3;
        const TOUCH: u32 = 4;
        const POOL_FLAG: u32 = 5;
        const POOL_RESOLVE: u32 = 6;
        const POOL_COMPARE: u32 = 7;
        const MODE_RESET: u32 = 8;
        const MODE_SET: u32 = 9;
        const NESTED_APPLY: u32 = 10;
        const WIDE_APPLY: u32 = 11;

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
        unsafe fn or32(a: u32, v: u32) {
            unsafe { wr32(a, rd32(a) | v) }
        }
        #[inline(always)]
        unsafe fn trace(retv: &mut u32, what: u32) {
            unsafe {
                *retv = lf_checker_rt::callee_thiscall!(
                    TRACE, u32,
                    lf_checker_rt::relocated(TRACE_THIS),
                    lf_checker_rt::relocated(what)
                );
            }
        }

        let delta = delta_u as i32;
        let inner = rd32(this.wrapping_add(INNER_OFF));
        let esi: u32 = lf_checker_rt::callee_thiscall!(ENTRY_PROBE, u32, inner);
        let edi: u32 = lf_checker_rt::callee_thiscall!(REF_PROBE, u32, inner);
        let first = rd8(esi);
        let latched_9 = first == 9;
        let latched_10 = first == 10;
        let row = rd32(this.wrapping_add(ROW_OFF)).wrapping_sub(1);
        let mut retv = edi;
        if row > 0x10 {
            return retv;
        }
        match row {
            // Toggle flag A.
            0 => {
                trace(&mut retv, 0x00eeaf8c);
                let v = u8::from(rd8(this.wrapping_add(FLAG_A_OFF)) == 0);
                wr8(this.wrapping_add(FLAG_A_OFF), v);
                retv = (retv & 0xffff_ff00) | (v as u32);
            }
            // Step field 8 modulo 21 (nonpositive remainders become 20),
            // touch on change, reset the mode pair when it lands on zero.
            1 => {
                trace(&mut retv, 0x00eeb01c);
                let t = (rd8(esi.wrapping_add(8)) as i32).wrapping_add(delta);
                let mut r = t % 21;
                if r < 0 {
                    r = 20;
                }
                let dl = r as u8;
                wr8(esi.wrapping_add(8), dl);
                retv = 0x14;
                let changed = if edi != 0 {
                    rd8(edi.wrapping_add(8)) != dl
                } else {
                    dl != 0
                };
                if changed {
                    retv = lf_checker_rt::callee_thiscall!(TOUCH, u32, inner, 0);
                }
                if rd8(esi.wrapping_add(8)) == 0 {
                    retv = lf_checker_rt::callee_cdecl!(MODE_RESET, u32,);
                    retv = lf_checker_rt::callee_cdecl!(MODE_SET, u32, 1);
                }
            }
            // Toggle flag B.
            2 => {
                trace(&mut retv, 0x00eeafd4);
                let v = u8::from(rd8(this.wrapping_add(FLAG_B_OFF)) == 0);
                wr8(this.wrapping_add(FLAG_B_OFF), v);
                retv = (retv & 0xffff_ff00) | (v as u32);
            }
            // Percent field: (25*delta + byte) mod 250 with the negative
            // rule, through the nested/wide apply pair, touch on change.
            3 => {
                trace(&mut retv, 0x00eeb040);
                let t = delta
                    .wrapping_mul(25)
                    .wrapping_add(rd8(esi.wrapping_add(1)) as i32);
                let mut r = t % 250;
                if !(r > 0) && delta < 0 {
                    r = 0xe1;
                }
                wr8(esi.wrapping_add(1), r as u8);
                let nested = lf_checker_rt::relocated(NESTED_THIS);
                let applied: u32 = lf_checker_rt::callee_thiscall!(
                    NESTED_APPLY, u32,
                    nested, lf_checker_rt::relocated(0x00eeb064)
                );
                retv = applied;
                retv = lf_checker_rt::callee_thiscall!(
                    WIDE_APPLY, u32,
                    lf_checker_rt::relocated(WIDE_THIS),
                    applied, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0xffff_ffff
                );
                if rd8(esi.wrapping_add(1)) == 0 {
                    wr8(esi.wrapping_add(1), 0x19);
                }
                let changed = if edi != 0 {
                    rd8(edi.wrapping_add(1)) != rd8(esi.wrapping_add(1))
                } else {
                    rd8(esi.wrapping_add(1)) != 0x64
                };
                if changed {
                    retv = lf_checker_rt::callee_thiscall!(TOUCH, u32, inner, 1);
                }
            }
            // Toggle flag C.
            4 => {
                trace(&mut retv, 0x00eeafb0);
                let v = u8::from(rd8(this.wrapping_add(FLAG_C_OFF)) == 0);
                wr8(this.wrapping_add(FLAG_C_OFF), v);
                retv = (retv & 0xffff_ff00) | (v as u32);
            }
            // Clamp wide field A against its global.
            5 => {
                trace(&mut retv, 0x00eeb14c);
                let g = rd32(lf_checker_rt::global::<u32>(CLAMP_A_G) as u32) as i32;
                let mut v = (rd32(esi.wrapping_add(0x20)) as i32).wrapping_add(delta);
                retv = (g.wrapping_add(v)) as u32;
                if (g.wrapping_add(v)) > 8 {
                    v = 0;
                } else if v < 0 {
                    v = 8i32.wrapping_sub(g);
                }
                or32(esi.wrapping_add(0x0c), 8);
                wr32(esi.wrapping_add(0x20), v as u32);
            }
            // Clamp wide field B against its global.
            6 => {
                trace(&mut retv, 0x00eeb170);
                let g = rd32(lf_checker_rt::global::<u32>(CLAMP_B_G) as u32) as i32;
                let mut v = (rd32(esi.wrapping_add(0x24)) as i32).wrapping_add(delta);
                retv = (g.wrapping_add(v)) as u32;
                if (g.wrapping_add(v)) > 8 {
                    v = 0;
                } else if v < 0 {
                    v = 8i32.wrapping_sub(g);
                }
                or32(esi.wrapping_add(0x0c), 0x10);
                wr32(esi.wrapping_add(0x24), v as u32);
            }
            // Flip bit 2 of byte 3.
            7 => {
                trace(&mut retv, 0x00eeb194);
                let old = rd8(esi.wrapping_add(3));
                or32(esi.wrapping_add(0x0c), 0x20);
                wr8(esi.wrapping_add(3), old ^ 4);
                retv = (retv & 0xffff_ff00) | (old as u32);
            }
            // Toggle flag D; clearing it also clears flag E.
            8 => {
                trace(&mut retv, 0x00eeaff8);
                let v = u8::from(rd8(this.wrapping_add(FLAG_D_OFF)) == 0);
                wr8(this.wrapping_add(FLAG_D_OFF), v);
                retv = (retv & 0xffff_ff00) | (v as u32);
                if v == 0 {
                    wr8(this.wrapping_add(FLAG_E_OFF), 0);
                }
            }
            // Navigate the head pair, honouring the latches.
            9 => {
                trace(&mut retv, 0x00eeb7f4);
                if !latched_9 && !latched_10 {
                    wr8(esi, 9);
                    wr8(esi.wrapping_add(4), 1);
                } else {
                    wr8(this.wrapping_add(FLAG_E_OFF), 0);
                    wr8(esi, 8);
                    wr8(esi.wrapping_add(4), 0);
                    if rd8(esi.wrapping_add(5)) == 3 {
                        wr8(esi.wrapping_add(5), 0);
                    }
                }
                let same = if edi != 0 {
                    rd8(edi.wrapping_add(4)) == rd8(esi.wrapping_add(4))
                } else {
                    rd8(esi.wrapping_add(4)) == 8
                };
                if !same {
                    retv = lf_checker_rt::callee_thiscall!(TOUCH, u32, inner, 6);
                } else if edi != 0 {
                    retv = (retv & 0xffff_ff00) | (rd8(edi.wrapping_add(4)) as u32);
                } else {
                    retv = (retv & 0xffff_ff00) | u32::from(latched_9);
                }
            }
            // Divide-step the head byte, then settle the state byte.
            10 => {
                trace(&mut retv, 0x00eeaf68);
                let mut dl: u8;
                if latched_9 {
                    dl = 0x0a;
                } else if latched_10 {
                    dl = 9;
                } else {
                    let t = (rd8(esi) as i32).wrapping_add(delta);
                    let mut r = t % 9;
                    if r < 1 {
                        if delta > 0 {
                            r = 1;
                        } else if delta < 0 {
                            r = 8;
                        }
                    }
                    dl = r as u8;
                }
                wr8(esi, dl);
                if latched_9 || latched_10 {
                    let a = (dl as u32).wrapping_sub(1);
                    if a == 8 {
                        let s5 = rd8(esi.wrapping_add(5));
                        if s5 == 2 || s5 == 1 {
                            wr8(esi.wrapping_add(5), 0);
                        }
                    } else if rd8(esi.wrapping_add(5)) == 3 {
                        wr8(esi.wrapping_add(5), 0);
                    }
                }
                let a1 = (dl as u32).wrapping_sub(1);
                if a1 != 8 && (rd8(esi.wrapping_add(3)) & 2) == 0 {
                    let p: u32 = lf_checker_rt::callee_thiscall!(
                        POOL_FLAG, u32,
                        lf_checker_rt::relocated(POOL_THIS)
                    );
                    retv = p;
                    or32(p.wrapping_add(0x39b), 2);
                }
                let a2 = (rd8(esi) as u32).wrapping_sub(1);
                if a2 == 9 && rd8(esi.wrapping_add(5)) == 3 {
                    wr8(esi.wrapping_add(5), 2);
                }
                if a2 == 8 {
                    let s5 = rd8(esi.wrapping_add(5));
                    if s5 == 2 || s5 == 1 {
                        wr8(esi.wrapping_add(5), 3);
                    }
                }
                retv = rd8(esi) as u32;
            }
            // Resolve through the pool pair and touch on field change.
            11 => {
                trace(&mut retv, 0x00eeb074);
                let f10 = rd32(esi.wrapping_add(0x10));
                let pool = lf_checker_rt::relocated(POOL_THIS);
                let r: u32 = lf_checker_rt::callee_thiscall!(
                    POOL_RESOLVE, u32,
                    pool, esi, f10, delta_u
                );
                retv = lf_checker_rt::callee_thiscall!(
                    POOL_COMPARE, u32,
                    r, esi, f10, delta_u
                );
                let same = if edi != 0 {
                    rd32(edi.wrapping_add(0x10)) == rd32(esi.wrapping_add(0x10))
                } else {
                    rd32(esi.wrapping_add(0x10)) == 0xffff_ffff
                };
                if !same {
                    retv = lf_checker_rt::callee_thiscall!(TOUCH, u32, inner, 8);
                } else if edi != 0 {
                    retv = rd32(edi.wrapping_add(0x10));
                }
            }
            // Step field 9 modulo 3 (negatives become 2).
            12 => {
                trace(&mut retv, 0x00eeb0e0);
                let t = (rd8(esi.wrapping_add(9)) as i32).wrapping_add(delta);
                let r = t % 3;
                or32(esi.wrapping_add(0x0c), 0x2000);
                wr8(esi.wrapping_add(9), (if r < 0 { 2 } else { r }) as u8);
                retv = 2;
            }
            // Step field 2 by five, clamped to the global limit.
            13 => {
                trace(&mut retv, 0x00eeb098);
                let limit =
                    rd32(lf_checker_rt::global::<u32>(LIMIT_G) as u32) as i32;
                let first = rd8(esi.wrapping_add(2)) as i32;
                let mut v = delta.wrapping_mul(5).wrapping_add(first);
                if v < 0x0f && delta < 0 {
                    v = limit;
                    retv = first as u32;
                } else if v > limit {
                    if delta > 0 {
                        v = 0x0f;
                    }
                    retv = 0x0f;
                } else {
                    retv = first as u32;
                }
                or32(esi.wrapping_add(0x0c), 0x200);
                wr8(esi.wrapping_add(2), v as u8);
            }
            // Step the state byte modulo 4 with latch remapping.
            14 => {
                trace(&mut retv, 0x00eeb104);
                let mut v = ((rd8(esi.wrapping_add(5)) as i32).wrapping_add(delta)) % 4;
                if !latched_9 && v == 3 {
                    v = (delta.wrapping_add(3)) % 4;
                }
                if latched_9 {
                    while v == 2 || v == 1 {
                        v = (v.wrapping_add(delta)) % 4;
                    }
                }
                if v < 0 {
                    v = 3;
                    if !latched_9 {
                        v = (delta.wrapping_add(3)) % 4;
                    }
                    if latched_9 {
                        while v == 2 || v == 1 {
                            v = (v.wrapping_add(delta)) % 4;
                        }
                    }
                }
                let flag = if edi != 0 {
                    let e = (rd8(edi) as u32).wrapping_sub(1);
                    e == 8 || e == 9
                } else {
                    false
                };
                let c = v as u8;
                if v == 0 && (!flag || rd8(edi.wrapping_add(5)) == 0) {
                    wr8(this.wrapping_add(FLAG_E_OFF), 0);
                } else {
                    wr8(this.wrapping_add(FLAG_E_OFF), 1);
                }
                or32(esi.wrapping_add(0x0c), 0x800);
                wr8(esi.wrapping_add(5), c);
                retv = if edi != 0 {
                    u32::from(flag)
                } else {
                    delta_u & 0xffff_ff00
                };
            }
            // Step field 6 modulo 4, or toggle it against the reference.
            15 => {
                trace(&mut retv, 0x00eeb128);
                let a5 = rd8(esi.wrapping_add(5));
                if a5 != 0 && edi != 0 && rd8(edi.wrapping_add(5)) != 0 {
                    let r = ((rd8(esi.wrapping_add(6)) as i32).wrapping_add(delta)) % 4;
                    if r < 0 {
                        wr8(esi.wrapping_add(6), 3);
                        retv = 3;
                    } else {
                        wr8(esi.wrapping_add(6), r as u8);
                        retv = r as u32;
                    }
                } else if a5 != 0 {
                    let v = if rd8(esi.wrapping_add(6)) == 0 { 2u32 } else { 0 };
                    wr8(esi.wrapping_add(6), v as u8);
                    retv = v;
                } else if edi != 0 && rd8(edi.wrapping_add(5)) != 0 {
                    let v = u32::from(rd8(esi.wrapping_add(6)) == 0);
                    wr8(esi.wrapping_add(6), v as u8);
                    retv = v;
                } else {
                    wr8(esi.wrapping_add(6), 0);
                    retv = 0;
                }
                or32(esi.wrapping_add(0x0c), 0x1000);
            }
            // Step field 7 modulo 4 (negatives become 3).
            16 => {
                trace(&mut retv, 0x00eeb0bc);
                let t = (rd8(esi.wrapping_add(7)) as i32).wrapping_add(delta);
                let r = t % 4;
                let kept = if r < 0 { 3 } else { r };
                or32(esi.wrapping_add(0x0c), 0x400);
                wr8(esi.wrapping_add(7), kept as u8);
                retv = kept as u32;
            }
            _ => {}
        }
        retv
    }
});
