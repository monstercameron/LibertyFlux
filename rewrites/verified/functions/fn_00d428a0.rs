// original: 0x00d428a0 task_op_dispatch (proposed)

/// Run one numbered task query against a ped or task object.
///
/// `obj` points to the object (kind bits at `+0x28`, condition word at
/// `+0x24`, sub-object at `+0x224`, vtable at `+0x0`). `op` (0..0x31)
/// selects the query through a jump table; anything above 0x31 returns the
/// entry call's value unchanged. `out` receives a status word at `+0x4`
/// (preset to 2, set to 1 when the query completes, 0 for one query) plus
/// query-specific fields. `arg3` is only read by two queries (bit tests).
///
/// Entry: the kind field selects one of two aliases (`edi` when kind is 3,
/// `ebp` when kind is 6, else both null; most queries need `edi`), then a
/// virtual slot (`+0xd4`, thiscall, no arguments) runs and its return value
/// `v` becomes the default result. Each query is a short chain of object,
/// task-list and weapon-manager calls whose scripted answers steer the
/// path; the shared tail re-queries the task list and the shared epilogue
/// sets status 1 only when bit 27 of the condition word is set. One query
/// compares a 16-bit field against three game-state words (the first two
/// exit directly, the third feeds the stale-flag jump), three compare a
/// game float against a constant (ordered comparisons: NaN takes the exit),
/// one stores a squared distance, and two pass the address of the incoming
/// `obj` slot to a callee that fills one word through it (the word itself
/// is never read back). All `mode`-style comparisons are equality or
/// unsigned; the two signed ones are the task-count bounds in queries 21
/// (continue while signed greater than 20) and 22 (while signed not
/// greater). Returns a small status or the last callee's value in `eax`.
///
/// Original: 0x00d428a0 (cdecl, four stack words).
lf_checker_rt::export!(cdecl, rw_00d428a0(obj: u32, op: u32, out: u32, arg3: u32) -> u32 {
    unsafe {
        const OBJ_X20: u32 = 0x20;
        const OBJ_X24: u32 = 0x24;
        const OBJ_KIND: u32 = 0x28;
        const OBJ_X2E: u32 = 0x2e;
        const OBJ_B1E2: u32 = 0x1e2;
        const OBJ_W21C: u32 = 0x21c;
        const OBJ_SUB: u32 = 0x224;
        const OBJ_Q228: u32 = 0x228;
        const OBJ_B26C: u32 = 0x26c;
        const OBJ_BA60: u32 = 0xa60;
        const OBJ_B219: u32 = 0x219;
        const OBJ_WB30: u32 = 0xb30;
        const OBJ_FE50: u32 = 0xe50;
        const SUB_TMGR: u32 = 0x2e0;
        const OBJ_WMGR: u32 = 0x2b0;
        const SUB_X44: u32 = 0x44;
        const OUT_ST: u32 = 0x04;
        const OUT_X8: u32 = 0x08;
        const OUT_XC: u32 = 0x0c;
        const OUT_X10: u32 = 0x10;
        const OUT_B14: u32 = 0x14;
        const OUT_X18: u32 = 0x18;
        const OUT_X20: u32 = 0x20;
        const G_A: u32 = 0x0172_0904;
        const G_B: u32 = 0x0172_0910;
        const G_C: u32 = 0x0172_091c;
        const ROW_TAB: u32 = 0x0129_5CD8;
        const G_84: u32 = 0x012D_DE84;
        const G_AC: u32 = 0x012D_DEAC;
        const G_B0: u32 = 0x012D_DEB0;
        const G_B4: u32 = 0x012D_DEB4;
        const C_120: u32 = 0x00FE_8BC0;
        const C_400: u32 = 0x00FE_8C20;
        const C_10: u32 = 0x00FE_8B08;
        const C_02: u32 = 0x00FE_87D0;
        const C_05: u32 = 0x00FE_8830;
        const COND_BIT: u32 = 0x0800_0000;
        const FILL_SLOT: u32 = 0xd4;
        const VFLOAT_SLOT: u32 = 0xfc;
        const VBYTE_SLOT: u32 = 0x128;
        const VWORD_SLOT: u32 = 0x1b8;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u16 {
            unsafe { (a as *const u16).read_unaligned() }
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
        unsafe fn g32(a: u32) -> u32 {
            unsafe { lf_checker_rt::global::<u32>(a).read() }
        }
        #[inline(always)]
        fn fsub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        fn fmul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn fadd(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        unsafe fn call_vt0(o: u32, slot: u32) -> u32 {
            unsafe {
                let f: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(rd32(rd32(o) + slot) as usize);
                f(o)
            }
        }

        // Epilogues. epi2 keeps the preset status 2, epi1 sets 1. epi_stale
        // is the shared `je`: every query except 49 jumps to it past the
        // bit test, so it branches on the stale zero flag of the query's
        // own last test (`flag_nz` = that test found nonzero). epi_test is
        // the one genuine execution of the bit test (query 49 only).
        #[inline(always)]
        unsafe fn epi2(eax: u32) -> u32 {
            eax
        }
        #[inline(always)]
        unsafe fn epi1(out: u32, eax: u32) -> u32 {
            unsafe {
                wr32(out + OUT_ST, 1);
                eax
            }
        }
        #[inline(always)]
        unsafe fn epi_stale(out: u32, eax: u32, flag_nz: bool) -> u32 {
            unsafe {
                if flag_nz {
                    wr32(out + OUT_ST, 1);
                }
                eax
            }
        }
        #[inline(always)]
        unsafe fn epi_test(out: u32, obj: u32, eax: u32) -> u32 {
            unsafe {
                if (rd32(obj + OBJ_X24) & COND_BIT) != 0 {
                    wr32(out + OUT_ST, 1);
                }
                eax
            }
        }
        // Shared tail: re-query the task list for `id`; the `(an instruction of the original)`
        // before the jump feeds the stale-flag `je`, so status 1 means the
        // call's low byte was nonzero.
        #[inline(always)]
        unsafe fn shared_tail(out: u32, sub: u32, id: u32) -> u32 {
            unsafe {
                let r = lf_checker_rt::callee_thiscall!(
                    5, u32, sub.wrapping_add(SUB_TMGR), id, 0);
                epi_stale(out, r, (r & 0xFF) != 0)
            }
        }
        // Shared tail of queries 9-11: status 1, run slot 0x1b8 on the
        // `+0xb30` object, store its answer at `+0x8`.
        #[inline(always)]
        unsafe fn tail9(out: u32, edi: u32) -> u32 {
            unsafe {
                wr32(out + OUT_ST, 1);
                let r = call_vt0(rd32(edi + OBJ_WB30), VWORD_SLOT);
                wr32(out + OUT_X8, r);
                r
            }
        }

        wr32(out + OUT_ST, 2);
        let kind = (rd32(obj + OBJ_KIND) >> 6) & 0xF;
        let edi = if kind == 3 { obj } else { 0 };
        let ebp = if kind == 6 { obj } else { 0 };
        let v = call_vt0(obj, FILL_SLOT);
        if op > 0x31 {
            return epi2(v);
        }
        match op {
            0 => {
                if edi == 0 {
                    return epi2(v);
                }
                // Stale-flag je fed by `cmp byte [edi+0x219], 0`.
                epi_stale(out, v, rd8(edi + OBJ_B219) != 0)
            }
            1 => {
                let c = lf_checker_rt::callee_thiscall!(2, u32, v, obj, 0);
                if c != 0xFFFF_FFFF {
                    return epi2(c);
                }
                if ebp != 0 {
                    let b = lf_checker_rt::callee_thiscall!(3, u32, ebp);
                    if b == 0 {
                        return epi2(0);
                    }
                    let b2 = lf_checker_rt::callee_thiscall!(3, u32, ebp);
                    let d = call_vt0(b2, 4);
                    if d != 1 {
                        return epi2(d);
                    }
                    wr32(out + OUT_ST, d);
                    return d;
                }
                if edi == 0 {
                    return epi2(c);
                }
                let sub = rd32(edi + OBJ_SUB);
                let f = lf_checker_rt::callee_thiscall!(
                    5, u32, sub.wrapping_add(SUB_TMGR), 0x390, 0);
                if (f & 0xFF) == 0 {
                    return shared_tail(out, sub, 8);
                }
                let f2 = lf_checker_rt::callee_thiscall!(
                    5, u32, sub.wrapping_add(SUB_TMGR), 0x3b3, 0);
                if (f2 & 0xFF) != 0 {
                    return shared_tail(out, sub, 8);
                }
                let f8 = lf_checker_rt::callee_thiscall!(6, u32, edi);
                if (f8 & 0xFF) != 0 {
                    return epi1(out, f8);
                }
                let a = lf_checker_rt::callee_thiscall!(7, u32, sub, 0x3b8);
                if a == 0 {
                    return epi1(out, 0);
                }
                shared_tail(out, sub, 8)
            }
            2 => {
                if edi == 0 {
                    return epi2(v);
                }
                let sub = rd32(edi + OBJ_SUB);
                let f = lf_checker_rt::callee_thiscall!(
                    5, u32, sub.wrapping_add(SUB_TMGR), 0x770, 0);
                if (f & 0xFF) != 0 {
                    return epi1(out, f);
                }
                let c21 = rd32(edi + OBJ_W21C);
                if rd32(c21.wrapping_add(0x12c)) == 2 {
                    return epi2(c21);
                }
                if rd8(edi + OBJ_BA60) != 1 {
                    return epi2(c21);
                }
                let f2 = lf_checker_rt::callee_thiscall!(
                    5, u32, sub.wrapping_add(SUB_TMGR), 0x2e2, 0);
                if (f2 & 0xFF) == 0 {
                    let r = lf_checker_rt::callee_thiscall!(
                        8, u32, sub.wrapping_add(SUB_X44), 0x2e2);
                    if r == 0 {
                        return epi2(0);
                    }
                }
                let g = lf_checker_rt::callee_thiscall!(9, u32, edi);
                if g == 0 {
                    return epi2(0);
                }
                let g2 = lf_checker_rt::callee_thiscall!(9, u32, edi);
                let q = rd32(g2.wrapping_add(0xf50));
                if q != 0 {
                    let g3 = lf_checker_rt::callee_thiscall!(9, u32, edi);
                    let q3 = rd32(g3.wrapping_add(0xf50));
                    if rd8(q3.wrapping_add(0x219)) != 0 {
                        return epi1(out, q3);
                    }
                }
                let p = lf_checker_rt::callee_cdecl!(10, u32,);
                if p == 0 {
                    return epi2(0);
                }
                let p2 = lf_checker_rt::callee_cdecl!(10, u32,);
                let s2 = rd32(p2.wrapping_add(OBJ_SUB));
                let f3 = lf_checker_rt::callee_thiscall!(
                    5, u32, s2.wrapping_add(SUB_TMGR), 0x2de, 0);
                if (f3 & 0xFF) == 0 {
                    return epi2(f3);
                }
                let a8 = lf_checker_rt::callee_thiscall!(
                    11, u32, s2.wrapping_add(SUB_TMGR), 0x2de, 5);
                let g4 = lf_checker_rt::callee_thiscall!(9, u32, edi);
                if a8 != g4 {
                    return epi2(g4);
                }
                wr32(out + OUT_ST, 1);
                g4
            }
            3 => {
                if edi == 0 {
                    return epi2(v);
                }
                shared_tail(out, rd32(edi + OBJ_SUB), 0x3b3)
            }
            4 => {
                if edi == 0 {
                    return epi2(v);
                }
                let sub = rd32(edi + OBJ_SUB);
                let f = lf_checker_rt::callee_thiscall!(
                    5, u32, sub.wrapping_add(SUB_TMGR), 0x76c, 0);
                if (f & 0xFF) != 0 {
                    let f2 = lf_checker_rt::callee_thiscall!(
                        5, u32, sub.wrapping_add(SUB_TMGR), 0x770, 0);
                    if (f2 & 0xFF) == 0 {
                        return epi1(out, f2);
                    }
                }
                let f3 = lf_checker_rt::callee_thiscall!(
                    5, u32, sub.wrapping_add(SUB_TMGR), 6, 0);
                epi_stale(out, f3, (f3 & 0xFF) != 0)
            }
            5 => {
                if edi == 0 {
                    return epi2(v);
                }
                shared_tail(out, rd32(edi + OBJ_SUB), 0x78d)
            }
            6 => {
                if edi == 0 {
                    return epi2(v);
                }
                shared_tail(out, rd32(edi + OBJ_SUB), 0x258)
            }
            7 => {
                if edi == 0 {
                    return epi2(v);
                }
                shared_tail(out, rd32(edi + OBJ_SUB), 0x25e)
            }
            8 => {
                if edi == 0 {
                    return epi2(v);
                }
                let f = lf_checker_rt::callee_thiscall!(6, u32, edi);
                if (f & 0xFF) != 0 {
                    return epi2(f);
                }
                let a = lf_checker_rt::callee_thiscall!(
                    7, u32, rd32(edi + OBJ_SUB), 0x3b8);
                // Stale-flag je fed by `(an instruction of the original)` (full word).
                epi_stale(out, a, a != 0)
            }
            9 => {
                if edi == 0 {
                    return epi2(v);
                }
                if (rd8(edi + OBJ_B26C) & 4) == 0 {
                    return epi2(v);
                }
                let b30 = rd32(edi + OBJ_WB30);
                if b30 == 0 {
                    return epi2(0);
                }
                if rd32(b30.wrapping_add(0xf50)) != edi {
                    return epi2(b30);
                }
                tail9(out, edi)
            }
            10 => {
                if edi == 0 {
                    return epi2(v);
                }
                if (rd8(edi + OBJ_B26C) & 4) == 0 {
                    return epi2(v);
                }
                if rd32(edi + OBJ_WB30) == 0 {
                    return epi2(v);
                }
                let a = lf_checker_rt::callee_stdcall!(14, u32, edi);
                if a != 2 {
                    return epi2(a);
                }
                tail9(out, edi)
            }
            11 => {
                if edi == 0 {
                    return epi2(v);
                }
                if (rd8(edi + OBJ_B26C) & 4) == 0 {
                    return epi2(v);
                }
                if rd32(edi + OBJ_WB30) == 0 {
                    return epi2(v);
                }
                let a = lf_checker_rt::callee_stdcall!(14, u32, edi);
                if a == 1 || a == 3 {
                    return tail9(out, edi);
                }
                epi2(a)
            }
            12 | 15 => {
                let c = lf_checker_rt::callee_thiscall!(2, u32, v, obj, 0);
                if c == 0xFFFF_FFFF {
                    return epi2(c);
                }
                wr32(out + OUT_X10, c);
                wr8(out + OUT_B14, 0);
                wr32(out + OUT_ST, 1);
                c
            }
            13 => {
                if edi == 0 {
                    return epi2(v);
                }
                let sub = rd32(edi + OBJ_SUB);
                let f = lf_checker_rt::callee_thiscall!(
                    5, u32, sub.wrapping_add(SUB_TMGR), 0xdd, 0);
                if (f & 0xFF) == 0 {
                    return epi2(f);
                }
                let r = lf_checker_rt::callee_thiscall!(
                    8, u32, sub.wrapping_add(SUB_X44), 0xdf);
                if r == 0 {
                    return epi2(0);
                }
                let d = rd32(r.wrapping_add(0x34));
                wr32(out + OUT_ST, 1);
                match d {
                    0 => {
                        wr32(out + OUT_XC, 2);
                        0
                    }
                    2 => {
                        wr32(out + OUT_XC, 1);
                        2
                    }
                    1 => {
                        wr32(out + OUT_XC, 3);
                        1
                    }
                    4 => {
                        wr32(out + OUT_XC, 4);
                        4
                    }
                    3 => {
                        wr32(out + OUT_XC, 5);
                        3
                    }
                    _ => {
                        wr32(out + OUT_ST, 2);
                        d
                    }
                }
            }
            14 => {
                if (arg3 & 8) != 0 {
                    wr32(out + OUT_X8, 0);
                } else if (arg3 & 0x10) != 0 {
                    wr32(out + OUT_X8, 1);
                } else if (arg3 & 0x20) != 0 {
                    wr32(out + OUT_X8, 2);
                } else if (arg3 & 0x40) != 0 {
                    wr32(out + OUT_X8, 3);
                } else {
                    return epi2(arg3);
                }
                wr32(out + OUT_ST, 1);
                arg3
            }
            16 => {
                // Stale-flag je fed by `test byte [arg3], 1`.
                epi_stale(out, v, (arg3 & 1) != 0)
            }
            17 => {
                let p = lf_checker_rt::callee_cdecl!(10, u32,);
                if p == 0 {
                    return epi2(0);
                }
                wr32(out + OUT_ST, 1);
                let p2 = lf_checker_rt::callee_cdecl!(10, u32,);
                let c = rd32(obj + OBJ_X20);
                let base = if c != 0 { c.wrapping_add(0x30) } else { obj.wrapping_add(0x10) };
                let q = rd32(p2.wrapping_add(OBJ_X20));
                let dx = fsub(
                    f32::from_bits(rd32(base)),
                    f32::from_bits(rd32(q.wrapping_add(0x30))),
                );
                let dy = fsub(
                    f32::from_bits(rd32(base.wrapping_add(4))),
                    f32::from_bits(rd32(q.wrapping_add(0x34))),
                );
                let dz = fsub(
                    f32::from_bits(rd32(base.wrapping_add(8))),
                    f32::from_bits(rd32(q.wrapping_add(0x38))),
                );
                let d2 = fadd(fadd(fmul(dy, dy), fmul(dx, dx)), fmul(dz, dz));
                wr32(out + OUT_X18, d2.to_bits());
                q
            }
            18 => {
                if edi == 0 {
                    return epi2(v);
                }
                shared_tail(out, rd32(edi + OBJ_SUB), 0x116)
            }
            19 => {
                if edi == 0 {
                    return epi2(v);
                }
                shared_tail(out, rd32(edi + OBJ_SUB), 0x144)
            }
            20 => {
                if edi == 0 {
                    return epi2(v);
                }
                shared_tail(out, rd32(edi + OBJ_SUB), 0x143)
            }
            21 => {
                if edi == 0 {
                    return epi2(v);
                }
                let sub = rd32(edi + OBJ_SUB);
                let f = lf_checker_rt::callee_thiscall!(
                    5, u32, sub.wrapping_add(SUB_TMGR), 0x141, 0);
                if (f & 0xFF) == 0 {
                    return epi2(f);
                }
                let mut scratch = [obj; 1];
                let a = lf_checker_rt::callee_thiscall!(
                    12, u32, sub, scratch.as_mut_ptr() as u32);
                if a == 0 {
                    return epi2(0);
                }
                // Signed bound (jle): continue while [a] > 20.
                if (rd32(a) as i32) <= 0x14 {
                    return epi2(a);
                }
                wr32(out + OUT_ST, 1);
                a
            }
            22 => {
                if edi == 0 {
                    return epi2(v);
                }
                let sub = rd32(edi + OBJ_SUB);
                let f = lf_checker_rt::callee_thiscall!(
                    5, u32, sub.wrapping_add(SUB_TMGR), 0x141, 0);
                if (f & 0xFF) == 0 {
                    return epi2(f);
                }
                let mut scratch = [obj; 1];
                let a = lf_checker_rt::callee_thiscall!(
                    12, u32, sub, scratch.as_mut_ptr() as u32);
                if a == 0 {
                    return epi2(0);
                }
                // Signed bound (jg): continue while [a] <= 20.
                if (rd32(a) as i32) > 0x14 {
                    return epi2(a);
                }
                wr32(out + OUT_ST, 1);
                a
            }
            23 => {
                if edi == 0 {
                    return epi2(v);
                }
                let slot: extern "thiscall" fn(u32) -> f32 =
                    core::mem::transmute(rd32(rd32(edi) + VFLOAT_SLOT) as usize);
                let st0 = slot(edi);
                // The stub answers eax with the same bits it loads into ST0
                // (fld + (an instruction of the original)), so the returned eax is st0's bits; the
                // proof never feeds signalling NaNs, the one input where an
                // x87 load would not round-trip.
                let eax = st0.to_bits();
                let lim = f32::from_bits(g32(C_120));
                if !(lim > st0) {
                    return epi2(eax);
                }
                wr32(out + OUT_ST, 1);
                eax
            }
            24 => {
                let c = lf_checker_rt::callee_thiscall!(2, u32, v, obj, 0);
                // Join point: eax holds the b0e0 answer or the [b+0x1c]
                // word; c == 0 reaches the game-state compares, and the
                // third compare feeds the stale-flag jump below.
                let join: u32;
                if edi != 0 && (rd8(edi + OBJ_B1E2) & 0x0F) >= 2 {
                    let b = lf_checker_rt::callee_thiscall!(13, u32, edi);
                    if b == 0 {
                        return epi2(0);
                    }
                    join = lf_checker_rt::callee_thiscall!(13, u32, edi);
                } else if ebp == 0 {
                    return epi2(c);
                } else {
                    let b = lf_checker_rt::callee_thiscall!(3, u32, ebp);
                    if b == 0 {
                        return epi2(0);
                    }
                    let b2 = lf_checker_rt::callee_thiscall!(3, u32, ebp);
                    let d = call_vt0(b2, 4);
                    if d != 0 {
                        return epi2(d);
                    }
                    let b3 = lf_checker_rt::callee_thiscall!(3, u32, ebp);
                    join = rd32(b3.wrapping_add(0x1c));
                }
                if join == 0 {
                    return epi2(0);
                }
                if c != 0 {
                    return epi1(out, join);
                }
                let m = rd16(join.wrapping_add(OBJ_X2E)) as i16 as i32;
                if m == g32(G_A) as i32 || m == g32(G_B) as i32 {
                    return epi2(m as u32);
                }
                // Stale-flag je fed by `(an instruction of the original)`: status 1 unless the
                // field equals the third game word. Nothing is read through
                // the null `c` (the bit test is skipped), so this never faults.
                epi_stale(out, m as u32, (m as u32) != g32(G_C))
            }
            25 => {
                if edi == 0 {
                    return epi2(v);
                }
                let s = call_vt0(edi, VBYTE_SLOT);
                if (s & 0xFF) == 0 {
                    return epi2(s);
                }
                let q = rd32(edi + OBJ_Q228);
                if q == 0 {
                    return epi2(0);
                }
                let lim = f32::from_bits(g32(C_400));
                let m = f32::from_bits(rd32(q.wrapping_add(0x424)));
                if !(lim > m) {
                    return epi2(q);
                }
                wr32(out + OUT_ST, 1);
                q
            }
            26 => {
                if edi == 0 {
                    return epi2(v);
                }
                let lim = f32::from_bits(g32(C_10));
                let m = f32::from_bits(rd32(edi + OBJ_FE50));
                if !(lim >= m) {
                    return epi2(v);
                }
                wr32(out + OUT_ST, 1);
                v
            }
            27 => {
                if edi == 0 {
                    return epi2(v);
                }
                let g = lf_checker_rt::callee_thiscall!(
                    18, u32, edi.wrapping_add(OBJ_WMGR));
                if g == 0 {
                    return epi1(out, 0);
                }
                let g2 = lf_checker_rt::callee_thiscall!(
                    18, u32, edi.wrapping_add(OBJ_WMGR));
                if rd32(g2.wrapping_add(0x18)) != 0 {
                    return epi2(g2);
                }
                wr32(out + OUT_ST, 1);
                g2
            }
            28 => {
                if edi == 0 {
                    return epi2(v);
                }
                let e = edi.wrapping_add(OBJ_WMGR);
                let g = lf_checker_rt::callee_thiscall!(18, u32, e);
                if g == 0 {
                    return epi2(0);
                }
                let g2 = lf_checker_rt::callee_thiscall!(18, u32, e);
                let m = lf_checker_rt::callee_thiscall!(19, u32, g2);
                if (m & 0xFF) == 0 {
                    return epi2(m);
                }
                let g3 = lf_checker_rt::callee_thiscall!(18, u32, e);
                // Stale-flag je fed by `(an instruction of the original)`.
                epi_stale(out, g3, rd32(g3.wrapping_add(0x18)) != 0)
            }
            29 => {
                if edi == 0 {
                    return epi2(v);
                }
                let g = lf_checker_rt::callee_thiscall!(
                    18, u32, edi.wrapping_add(OBJ_WMGR));
                if g == 0 {
                    return epi2(0);
                }
                let g2 = lf_checker_rt::callee_thiscall!(
                    18, u32, edi.wrapping_add(OBJ_WMGR));
                let b = lf_checker_rt::callee_thiscall!(20, u32, g2);
                // Stale-flag je fed by `(an instruction of the original)`.
                epi_stale(out, b, (b & 0xFF) != 0)
            }
            30 => {
                if edi == 0 {
                    return epi2(v);
                }
                let e = edi.wrapping_add(OBJ_WMGR);
                let g = lf_checker_rt::callee_thiscall!(18, u32, e);
                if g == 0 {
                    return epi2(0);
                }
                let g2 = lf_checker_rt::callee_thiscall!(18, u32, e);
                let n = lf_checker_rt::callee_thiscall!(21, u32, g2);
                if (n & 0xFF) == 0 {
                    return epi2(n);
                }
                let g3 = lf_checker_rt::callee_thiscall!(18, u32, e);
                let i = lf_checker_rt::callee_cdecl!(22, u32, rd32(g3.wrapping_add(0x18)));
                // Stale-flag je fed by `(an instruction of the original)`.
                epi_stale(out, i, rd32(i.wrapping_add(4)) != 7)
            }
            31 => {
                if edi == 0 {
                    return epi2(v);
                }
                let g = lf_checker_rt::callee_thiscall!(
                    18, u32, edi.wrapping_add(OBJ_WMGR));
                if g == 0 {
                    return epi2(0);
                }
                let g2 = lf_checker_rt::callee_thiscall!(
                    18, u32, edi.wrapping_add(OBJ_WMGR));
                let i = lf_checker_rt::callee_cdecl!(22, u32, rd32(g2.wrapping_add(0x18)));
                if rd32(i.wrapping_add(4)) != 7 {
                    return epi2(i);
                }
                wr32(out + OUT_ST, 1);
                i
            }
            32 | 33 | 34 | 35 | 36 | 37 => epi2(v),
            38 => {
                if edi == 0 {
                    return epi2(v);
                }
                shared_tail(out, rd32(edi + OBJ_SUB), 0x41e)
            }
            39 => {
                if edi == 0 {
                    return epi2(v);
                }
                let d = lf_checker_rt::callee_cdecl!(23, u32, edi, 0);
                if d == 2 {
                    return epi1(out, 2);
                }
                let d2 = lf_checker_rt::callee_cdecl!(23, u32, edi, 0);
                if d2 != 3 {
                    return epi2(d2);
                }
                wr32(out + OUT_ST, 1);
                3
            }
            40 => {
                if edi == 0 {
                    return epi2(v);
                }
                let d = lf_checker_rt::callee_cdecl!(23, u32, edi, 0);
                if d != 0 {
                    return epi2(d);
                }
                wr32(out + OUT_ST, 1);
                0
            }
            41 => {
                wr32(out + OUT_ST, 0);
                v
            }
            42 => {
                wr32(out + OUT_ST, 1);
                let i = rd16(obj + OBJ_X2E) as i16 as i32;
                let t = g32(ROW_TAB.wrapping_add((i.wrapping_mul(4)) as u32));
                let x = rd32(t.wrapping_add(0x3c));
                wr32(out + OUT_X20, x);
                x
            }
            43 => {
                let mem = f32::from_bits(g32(G_AC));
                if !(mem >= f32::from_bits(g32(C_02))) {
                    return epi2(v);
                }
                if (rd32(obj + OBJ_X24) & COND_BIT) != 0 {
                    return epi2(v);
                }
                if edi == 0 {
                    return epi1(out, v);
                }
                let f = lf_checker_rt::callee_thiscall!(24, u32, edi);
                if (f & 0xFF) == 0 {
                    return epi1(out, f);
                }
                let q = rd32(edi + OBJ_Q228);
                if rd8(q.wrapping_add(0x5b8)) != 0 {
                    return epi2(q);
                }
                wr32(out + OUT_ST, 1);
                q
            }
            44 => {
                let g = g32(G_84);
                if g != 1 && g != 0 {
                    return epi2(g);
                }
                let n = lf_checker_rt::callee_cdecl!(25, u32, 0x14, 6);
                if (n & 0xFF) != 0 {
                    return epi2(n);
                }
                if (rd32(obj + OBJ_X24) & COND_BIT) != 0 {
                    return epi2(n);
                }
                wr32(out + OUT_ST, 1);
                n
            }
            45 => {
                let n = lf_checker_rt::callee_cdecl!(25, u32, 0x14, 6);
                // Stale-flag je fed by `(an instruction of the original)`.
                epi_stale(out, n, (n & 0xFF) != 0)
            }
            46 => {
                let mem = f32::from_bits(g32(G_B0));
                if !(mem > f32::from_bits(g32(C_02))) {
                    return epi2(v);
                }
                if (rd32(obj + OBJ_X24) & COND_BIT) != 0 {
                    return epi2(v);
                }
                wr32(out + OUT_ST, 1);
                v
            }
            47 => {
                let mem = f32::from_bits(g32(G_B4));
                if !(mem >= f32::from_bits(g32(C_05))) {
                    return epi2(v);
                }
                if (rd32(obj + OBJ_X24) & COND_BIT) != 0 {
                    return epi2(v);
                }
                wr32(out + OUT_ST, 1);
                v
            }
            48 => epi1(out, v),
            49 => epi_test(out, obj, v),
            _ => epi2(v),
        }
    }
});
