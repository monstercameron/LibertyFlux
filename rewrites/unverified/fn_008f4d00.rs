// original: 0x008f4d00 input_device_poll (proposed)

/// Poll an input device object: sample four axes, sweep the key matrix,
/// run the zero-chain, and synthesize a key event when armed.
///
/// `obj` is the device object. The function reads four sampler values into
/// `+0x3A70..+0x3A7C`, configures up to six sub-units, then sweeps 188 key
/// slots: two slot ranges skip outright; 22 slots call the key handler only
/// when a scale query on the slot answers nonzero; four slots consult two
/// global enables; two slots compare key bytes with an UNSIGNED `> 0x7F`
/// bound and three with `!= 0x80`, marking `+0x328B` and `+0x328D`; the rest
/// call the handler on `> 0x7F` without marking. A layout query can mark
/// `+0x328A` and `+0x328D`; a mix gate and a flag table (read at computed
/// data addresses) can mark `+0x328B` and `+0x328D`. Then a zero-chain over
/// five query results and an indexed status table either raises the armed
/// flag `+0x328C` or continues; the tail toggles `+0x328E`, fills a 0x200-byte
/// report buffer with a zero or marker pattern, and, when armed and the
/// stored time plus 0x17700 is still UNSIGNED-below the clock, synthesizes
/// one F8 key press through the input queue. All callers ignore the return
/// value, so the contract compares no return channel.
///
/// Thiscall: object in ECX, no stack words.
const OBJ_KEYS: u32 = 0x269C;
const OBJ_CLS0: u32 = 0x26AC;
const OBJ_CLS2: u32 = 0x26AE;
const OBJ_CLS3: u32 = 0x26AF;
const OBJ_DIRTY0: u32 = 0x275C;
const OBJ_DIRTY1: u32 = 0x277C;
const OBJ_DIRTY2: u32 = 0x287C;
const OBJ_DIRTY3: u32 = 0x289C;
const OBJ_UNIT: u32 = 0x3288;
const OBJ_SEEN9: u32 = 0x3289;
const OBJ_SEENB: u32 = 0x328B;
const OBJ_ARMED: u32 = 0x328C;
const OBJ_MARKED: u32 = 0x328D;
const OBJ_TOGGLE: u32 = 0x328E;
const OBJ_REPFLAG: u32 = 0x3296;
const OBJ_REPORT: u32 = 0x329C;
const OBJ_TIME: u32 = 0x32A0;
const OBJ_INDEX: u32 = 0x32A8;
const OBJ_AXES: u32 = 0x3A70;
const G_UNIT_EN: u32 = 0x11609F6;
const G_MISC_EN: u32 = 0x117E6DA;
const G_UNIT_ARG: u32 = 0x11735B4;
const G_CMP_A: u32 = 0x117E6E0;
const G_CMP_B: u32 = 0x117E6DC;
const G_LAYOUT_SEL: u32 = 0x118D114;
const G_STATUS_TAB: u32 = 0x118D470;
const G_SCAN_MIX1: u32 = 0x18B7A88;
const G_SCAN_MIX0: u32 = 0x18B7A84;
const G_SCAN_PAGE: u32 = 0x18B7DA0;
const G_SCAN_TAB: u32 = 0x18B7A90;
const G_ALT_END: u32 = 0x1160EBC;
const G_CLOCK: u32 = 0x1173594;
const C_SAMPLER: u32 = 1;
const C_AXISMAP: u32 = 2;
const C_SCALE: u32 = 3;
const C_UNITCFG: u32 = 4;
const C_KEYHAND: u32 = 5;
const C_LAYOUTQ: u32 = 6;
const C_QUERY: u32 = 7;
const C_STATEQ: u32 = 8;
const C_MSGEXTRA: u32 = 9;
const C_SENDIN: u32 = 10;

#[inline(always)]
unsafe fn mrd8(a: u32) -> u8 {
    unsafe { (a as *const u8).read() }
}

#[inline(always)]
unsafe fn rd32(a: u32) -> u32 {
    unsafe { (a as *const u32).read_unaligned() }
}

#[inline(always)]
unsafe fn wr8(a: u32, v: u8) {
    unsafe { (a as *mut u8).write(v) }
}

#[inline(always)]
unsafe fn mwr16(a: u32, v: u16) {
    unsafe { (a as *mut u16).write_unaligned(v) }
}

#[inline(always)]
unsafe fn wr32(a: u32, v: u32) {
    unsafe { (a as *mut u32).write_unaligned(v) }
}

/// Fill 0x200 bytes at `buf` with the zero pattern: a zero byte and a zero
/// dword per 8-byte group (bytes 1..3 of each group keep their old values).
#[inline(always)]
unsafe fn fill_zero(buf: u32) {
    unsafe {
        let mut off = 0u32;
        while off < 0x200 {
            wr8(buf.wrapping_add(off), 0);
            wr32(buf.wrapping_add(off).wrapping_add(4), 0);
            off += 8;
        }
    }
}

/// Fill 0x200 bytes at `buf` with the marker pattern: 0xFF then a zero
/// dword per 8-byte group.
#[inline(always)]
unsafe fn fill_mark(buf: u32) {
    unsafe {
        let mut off = 0u32;
        while off < 0x200 {
            wr8(buf.wrapping_add(off), 0xFF);
            wr32(buf.wrapping_add(off).wrapping_add(4), 0);
            off += 8;
        }
    }
}

lf_checker_rt::export!(thiscall, rw_008f4d00(obj: u32) -> u32 {
    unsafe {
        use lf_checker_rt as rt;
        // Four sampler reads; each block's second pushed word belongs to the
        // call after next (the axis mapper's hidden argument, then the
        // previous scale argument).
        let s0 = rt::callee_cdecl!(C_SAMPLER, u32, 1, 0);
        let d0 = rt::callee_thiscall!(C_AXISMAP, u32, s0, 0);
        let v0 = rt::callee_cdecl!(C_SCALE, u32, d0);
        let s1 = rt::callee_cdecl!(C_SAMPLER, u32, 1, 0);
        let d1 = rt::callee_thiscall!(C_AXISMAP, u32, s1, 0);
        let v1 = rt::callee_cdecl!(C_SCALE, u32, d1);
        let s2 = rt::callee_cdecl!(C_SAMPLER, u32, 1, d1);
        let a2 = s2.wrapping_add(0x2B48);
        let v2 = rt::callee_cdecl!(C_SCALE, u32, a2);
        let s3 = rt::callee_cdecl!(C_SAMPLER, u32, 1, a2);
        let v3 = rt::callee_cdecl!(C_SCALE, u32, s3.wrapping_add(0x2B38));
        wr32(obj + OBJ_AXES, v0);
        wr32(obj + OBJ_AXES + 4, v1);
        wr32(obj + OBJ_AXES + 8, v2);
        wr32(obj + OBJ_AXES + 12, v3);
        // Sub-unit configuration burst.
        let g_arg = rd32(rt::relocated(G_UNIT_ARG));
        if mrd8(rt::relocated(G_UNIT_EN)) == 0 {
            rt::callee_thiscall!(C_UNITCFG, u32, obj, g_arg);
            rt::callee_thiscall!(C_UNITCFG, u32, obj + 0x7B8, g_arg);
            rt::callee_thiscall!(C_UNITCFG, u32, obj + 0xF70, g_arg);
            rt::callee_thiscall!(C_UNITCFG, u32, obj + 0x1EE0, g_arg);
        }
        if mrd8(rt::relocated(G_MISC_EN)) == 0 {
            rt::callee_thiscall!(C_UNITCFG, u32, obj + 0x32AC, g_arg);
        }
        rt::callee_thiscall!(C_UNITCFG, u32, obj + 0x1728, g_arg);
        wr32(obj + OBJ_UNIT, 0);
        // Key-matrix sweep over 188 slots.
        let g_cmp_a = rd32(rt::relocated(G_CMP_A));
        let g_cmp_b = rd32(rt::relocated(G_CMP_B));
        let mut edi = 0u32;
        while edi < 0xBC {
            let use_table = if edi < 0x40 {
                true
            } else if edi <= 0x54 {
                false
            } else if edi < 0x61 {
                true
            } else if edi > 0x63 {
                true
            } else {
                false
            };
            if use_table {
                let slot = obj.wrapping_add(OBJ_KEYS).wrapping_add(edi.wrapping_mul(0x10));
                if edi.wrapping_sub(0xC) > 0xAE {
                    if (mrd8(slot) ^ mrd8(slot + 2)) > 0x7F {
                        rt::callee_thiscall!(C_KEYHAND, u32, obj);
                    }
                } else {
                    match edi {
                        0x0C | 0x0D | 0x0E | 0x0F | 0x10 | 0x11 | 0x12 | 0x13 | 0x18 | 0x19
                        | 0x1A | 0x1B | 0x1E | 0x1F | 0x20 | 0x21 | 0x22 | 0x23 | 0x24 | 0x25
                        | 0xB9 | 0xBA => {
                            if rt::callee_cdecl!(C_SCALE, u32, slot.wrapping_sub(4)) != 0 {
                                rt::callee_thiscall!(C_KEYHAND, u32, obj);
                            }
                        }
                        0x58 | 0x8B => {
                            if g_cmp_a != 0 {
                                rt::callee_thiscall!(C_KEYHAND, u32, obj);
                                wr8(obj + OBJ_SEEN9, 1);
                                wr8(obj + OBJ_MARKED, 1);
                            }
                        }
                        0x59 | 0x8C => {
                            if g_cmp_b != 0 {
                                rt::callee_thiscall!(C_KEYHAND, u32, obj);
                                wr8(obj + OBJ_SEEN9, 1);
                                wr8(obj + OBJ_MARKED, 1);
                            }
                        }
                        0x5D | 0x5E => {
                            if (mrd8(slot) ^ mrd8(slot + 2)) > 0x7F {
                                rt::callee_thiscall!(C_KEYHAND, u32, obj);
                                wr8(obj + OBJ_SEENB, 1);
                                wr8(obj + OBJ_MARKED, 1);
                            }
                        }
                        0x5C | 0x5F | 0x60 => {
                            if (mrd8(slot) ^ mrd8(slot + 2)) != 0x80 {
                                rt::callee_thiscall!(C_KEYHAND, u32, obj);
                                wr8(obj + OBJ_SEENB, 1);
                                wr8(obj + OBJ_MARKED, 1);
                            }
                        }
                        _ => {
                            if (mrd8(slot) ^ mrd8(slot + 2)) > 0x7F {
                                rt::callee_thiscall!(C_KEYHAND, u32, obj);
                            }
                        }
                    }
                }
            }
            edi += 1;
        }
        // Layout query gate.
        if (mrd8(obj + OBJ_CLS0) ^ mrd8(obj + OBJ_CLS2)) > 0x7F
            && rt::callee_thiscall!(C_LAYOUTQ, u32, 0x118D110, 0x2A, 1, 0xE835C4) as u8 != 0
        {
            wr8(obj + OBJ_UNIT + 2, 1);
            wr8(obj + OBJ_MARKED, 1);
        }
        // Mix gate and flag-table scan (computed data addresses).
        let mix = (rd32(rt::relocated(G_SCAN_MIX1)) ^ rd32(rt::relocated(G_SCAN_MIX0)))
            & rd32(rt::relocated(G_SCAN_MIX1));
        if (mix as u8) & 7 != 0 {
            wr8(obj + OBJ_SEENB, 1);
            wr8(obj + OBJ_MARKED, 1);
        }
        if rd32(rt::relocated(G_LAYOUT_SEL)) == 1 {
            let base = rd32(rt::relocated(G_SCAN_PAGE))
                .wrapping_shl(8)
                .wrapping_add(G_SCAN_TAB);
            let mut off = 1u32;
            while off < 0xDD {
                if mrd8(rt::relocated(base.wrapping_add(off))) != 0 {
                    wr8(obj + OBJ_SEENB, 1);
                    wr8(obj + OBJ_MARKED, 1);
                    break;
                }
                off += 1;
            }
        }
        if mrd8(obj + OBJ_SEEN9) != 0 || mrd8(obj + OBJ_SEENB) != 0 {
            wr8(obj + OBJ_ARMED, 0);
        }
        // Zero-chain: any nonzero query raises the armed flag.
        let mut flagged = false;
        let q0 = rt::callee_thiscall!(C_QUERY, u32, obj);
        if rt::callee_thiscall!(C_STATEQ, u32, q0) != 0 {
            flagged = true;
        } else {
            let mut stop = false;
            for off in [4u32, 8, 0xC, 0x10] {
                let q = rt::callee_thiscall!(C_QUERY, u32, obj);
                if rd32(q.wrapping_add(off)) != 0 {
                    flagged = true;
                    stop = true;
                    break;
                }
            }
            if !stop {
                let idx = rd32(obj + OBJ_INDEX) as i32;
                if idx >= 0 {
                    let row = G_STATUS_TAB.wrapping_add((idx as u32).wrapping_mul(0xBC));
                    if rt::callee_thiscall!(C_STATEQ, u32, row) != 0 {
                        flagged = true;
                    } else {
                        for off in [4u32, 8, 0xC, 0x10] {
                            if rd32(rt::relocated(row.wrapping_add(off))) != 0 {
                                flagged = true;
                                break;
                            }
                        }
                    }
                }
            }
        }
        if flagged {
            mwr16(obj + OBJ_ARMED, 1);
        }
        // Tail dispatch: alternate ending, direct end, or toggle path.
        let alt_end = rd32(rt::relocated(G_ALT_END)) != 0 || mrd8(obj + OBJ_MARKED) == 0;
        let twins = !alt_end
            && (mrd8(obj + OBJ_CLS0) ^ mrd8(obj + OBJ_CLS2)) > 0x7F
            && (mrd8(obj + OBJ_CLS3) ^ mrd8(obj + OBJ_CLS0)) <= 0x7F;
        if twins {
            let t = (mrd8(obj + OBJ_TOGGLE) == 0) as u8;
            wr8(obj + OBJ_TOGGLE, t);
            if t != 0 {
                mwr16(obj + OBJ_REPFLAG, 0xFFFF);
                let rep = rd32(obj + OBJ_REPORT);
                if rep != 0 {
                    fill_mark(rep);
                }
            } else {
                mwr16(obj + OBJ_REPFLAG, 0);
                let rep = rd32(obj + OBJ_REPORT);
                if rep != 0 {
                    fill_zero(rep);
                }
            }
        } else if alt_end {
            wr8(obj + OBJ_TOGGLE, 0);
            mwr16(obj + OBJ_REPFLAG, 0);
            let rep = rd32(obj + OBJ_REPORT);
            if rep != 0 {
                fill_zero(rep);
            }
        }
        // Synthesize one F8 press when armed and due (unsigned time check).
        if mrd8(obj + OBJ_ARMED) != 0
            && rd32(obj + OBJ_TIME).wrapping_add(0x17700) < rd32(rt::relocated(G_CLOCK))
        {
            let extra = rt::callee_stdcall!(C_MSGEXTRA, u32,);
            let kb = [1u32, 0x77, 0, 0, extra];
            rt::callee_stdcall!(C_SENDIN, u32, 1, kb.as_ptr() as u32, 0x1C);
            wr32(obj + OBJ_TIME, rd32(rt::relocated(G_CLOCK)));
        }
        // Tail bytes follow the marked flag.
        let tail = if mrd8(obj + OBJ_MARKED) != 0 { 0xFF } else { 0 };
        wr8(obj + OBJ_DIRTY1, tail);
        wr8(obj + OBJ_DIRTY0, tail);
        wr8(obj + OBJ_DIRTY3, tail);
        wr8(obj + OBJ_DIRTY2, tail);
        0
    }
});
