// original: 0x0059E530 timing_tables_calibrate (proposed)

/// Calibrate the mainloop timing tables from the machine: read the system
/// configuration, derive the per-CPU timing rows, and publish the finished
/// tables to the timing globals.
///
/// `ecx_in` is a mode flag (only its low byte steers two branches; the full
/// word is also passed through to the configuration helper). The function
/// takes no stack arguments and no object. It queries the system
/// configuration and the power interface, reduces a CPU count with a signed
/// division by 24, samples the cycle counter helper twice (entry block and the
/// table-hit block below), walks one object table through two virtual calls,
/// scans the timing table for the row matching the sampled keys, and on a hit
/// runs the hit block (which may repeat while the filter float stays at or
/// above 1.0). Everything is published to globals; the return value is the
/// notifier answer when the notifier runs, otherwise the last table word.
/// The signed division by 24 is exact (multiplier `0x2AAAAAAB`, high-word
/// shift 2, sign adjust); the three constant divisions in the sample blocks
/// are exact multiply-and-shift sequences, mirrored bit for bit. Quotient
/// zero (a difference strictly between -24 and 24) is excluded: the original
/// raises a divide fault there while a Rust division aborts with a different
/// trap.
/// The stack argument of the post-table helper call is caller-saved register
/// residue from the previous virtual call, not behaviour, and is skipped.
///
/// Original: 0x0059E530 (thiscall, no stack arguments, `EAX` return).
/// Encrypted callees: none. Cookie check: intercepted with preserved
/// registers, the standard shape.
lf_checker_rt::export!(thiscall, rw_0059E530(ecx_in: u32) -> u32 {
    unsafe {
        const G_E80: u32 = 0x1160E80;
        const G_E84: u32 = 0x1160E84;
        const G_E88: u32 = 0x1160E88;
        const G_E8C: u32 = 0x1160E8C;
        const G_E90: u32 = 0x1160E90;
        const G_E94: u32 = 0x1160E94;
        const G_E98: u32 = 0x1160E98;
        const G_E9C: u32 = 0x1160E9C;
        const G_EA0: u32 = 0x1160EA0;
        const G_EA4: u32 = 0x1160EA4;
        const G_EA8: u32 = 0x1160EA8;
        const G_EAC: u32 = 0x1160EAC;
        const G_EB0: u32 = 0x1160EB0;
        const G_EB4: u32 = 0x1160EB4;
        const G_5500: u32 = 0x1BB5500;
        const G_5504: u32 = 0x1BB5504;
        const G_BDBC: u32 = 0x18B6DBC;
        const G_E6BC: u32 = 0x110E6BC;
        const G_BBH: u32 = 0x1168BB4;
        const G_BBH0: u32 = 0x1168BB0;
        const G_F90: u32 = 0x18B6E90;
        const G_OBJ: u32 = 0x17ED8D8;
        const G_ARG: u32 = 0x17ED930;
        const G_MODE: u32 = 0x1045538;
        const DIV6_MAGIC: u32 = 0x2AAAAAAB;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        /// Volatile twin of rd32: same value, same fault, but the compiler
        /// may not delete or reorder the read. Used only for the sample
        /// loop, whose sum the original discards (see _dead below): with a
        /// plain read LLVM proved the sum dead, deleted the reads and then
        /// the whole 4G-iteration loop, so the rewrite returned while the
        /// original faulted on overrun rows.
        #[inline(always)]
        unsafe fn rd32v(a: u32) -> u32 {
            unsafe { (a as *const u32).read_volatile() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn g32(va: u32) -> u32 {
            unsafe { rd32(lf_checker_rt::relocated(va)) }
        }
        #[inline(always)]
        unsafe fn wg32(va: u32, v: u32) {
            unsafe { wr32(lf_checker_rt::relocated(va), v) }
        }
        /// Signed divide-by-24 exactly as the original's imul/sar/adjust
        /// sequence computes it. Only the sign handling differs from an
        /// unsigned division; see the mutant.
        #[inline(always)]
        fn div6(diff: u32) -> i32 {
            let prod = (DIV6_MAGIC as i32 as i64).wrapping_mul(diff as i32 as i64);
            let q = ((prod >> 32) as i32) >> 2;
            q.wrapping_add((((q as u32) >> 31)) as i32)
        }
        /// The three constant divisions, exact multiply-and-shift mirrors.
        #[inline(always)]
        fn div_c1(v: u32) -> u32 {
            let hi = ((0x899C0F61u64 * (v as u64)) >> 32) as u32;
            let mut c = v.wrapping_sub(hi);
            c >>= 1;
            c = c.wrapping_add(hi);
            c >> 9
        }
        #[inline(always)]
        fn div_c2(v: u32) -> u32 {
            (((0x33C42535u64 * (v as u64)) >> 32) as u32) >> 7
        }
        #[inline(always)]
        fn div_c3(v: u32) -> u32 {
            (((0x88888889u64 * (v as u64)) >> 32) as u32) >> 8
        }

        let ecl: u8 = (ecx_in & 0xFF) as u8;
        let mut sysinfo = [0u32; 9];
        let _: u32 = lf_checker_rt::callee_stdcall!(1, u32, sysinfo.as_mut_ptr() as u32);
        let w10: u32 = sysinfo[4];
        let w14: u32 = sysinfo[5];
        let mut out1 = [0u32; 2];
        let mut out2 = [0u32; 3];
        // id2 takes (w14, out2, scratch): arg0 reloads sysinfo[5] from the
        // frame, arg2 pushes whatever id1 left in ecx, which is id1's
        // per-side step index, 0 for the single call of each trial.
        let _: u32 = lf_checker_rt::callee_thiscall!(2, u32, out1.as_mut_ptr() as u32,
            w14, out2.as_mut_ptr() as u32, 0);
        let o0: u32 = out1[0];
        let o4: u32 = out1[1];
        let mut slot20: u32 = 0;
        if o0 != o4 {
            let edi = div6(o4.wrapping_sub(o0));
            let outlen = (edi as u32).wrapping_mul(3).wrapping_shl(3);
            let _: u32 = lf_checker_rt::callee_stdcall!(3, u32, 0xB, 0, 0, o0, outlen);
            let mut edsum = 0u32;
            if edi != 0 {
                let mut esi_p = o0.wrapping_add(4);
                let mut cc = 0u32;
                loop {
                    let bit = 1u32.wrapping_shl(cc);
                    if (w10 & bit) != 0 {
                        edsum = edsum.wrapping_add(rd32v(esi_p));
                    }
                    cc = cc.wrapping_add(1);
                    esi_p = esi_p.wrapping_add(0x18);
                    if cc == edi as u32 {
                        break;
                    }
                }
            }
            if w14 > 1 {
                if w14 > 2 {
                    edsum = edsum.wrapping_shl(3);
                } else {
                    edsum = edsum.wrapping_shl(2);
                }
            }
            let q = edsum / (edi as u32);
            let t = q.wrapping_mul(0x63);
            let r = (((0x57619F1u64 * (t as u64)) >> 32) as u32) >> 9;
            slot20 = if r >= 0x63 { 0x63 } else { r };
        }
        let e80v: u32 = if ecl == 0 { 0 } else { g32(G_E80) };
        let c5500: u32 = g32(G_5500);
        let d5504: u32 = g32(G_5504);
        wg32(G_E80, e80v);
        if (c5500 | d5504) != 0 {
            if d5504 == 0 && c5500 <= 0x59999980 {
                wg32(G_E94, 0);
            } else {
                wg32(G_E94, 0x15);
            }
        } else {
            let mut msex = [0u32; 16];
            msex[0] = 0x40;
            let ans4: u32 = lf_checker_rt::callee_stdcall!(4, u32, msex.as_mut_ptr() as u32);
            if ans4 == 0 {
                wg32(G_E94, 0x15);
            } else {
                let cc0 = msex[2];
                let dd0 = msex[3];
                wg32(G_5500, cc0);
                wg32(G_5504, dd0);
                if dd0 == 0 && cc0 <= 0x59999980 {
                    wg32(G_E94, 0);
                } else {
                    wg32(G_E94, 0x15);
                }
            }
        }
        wg32(G_E8C, 0);
        let z0 = 0u32;
        let z1 = 0u32;
        let z2 = 0u32;
        let mut slot2 = 0u32;
        let _: u32 = lf_checker_rt::callee_cdecl!(5, u32, &slot2 as *const u32 as u32,
            &z0 as *const u32 as u32, &z1 as *const u32 as u32,
            &z2 as *const u32 as u32);
        let esi2: u32 = slot2;
        let c1 = div_c1(esi2);
        wg32(G_EAC, (if c1 <= 2 { c1 } else { 2 }).wrapping_add(1));
        let d2 = div_c2(esi2);
        wg32(G_EB0, (if d2 <= 3 { d2 } else { 3 }).wrapping_add(1));
        let d3 = div_c3(esi2);
        wg32(G_EB4, if d3 <= 3 { d3 } else { 3 });
        let early_e80: u32 = g32(G_E80);
        wg32(G_E98, 0x1E);
        wg32(G_E90, 1);
        wg32(G_E9C, 0x14);
        wg32(G_EA8, 0);
        wg32(G_EA0, 1);
        wg32(G_EA4, 1);
        let gp: u32 = g32(G_OBJ);
        let ec6: u32 = rd32(gp);
        let sl6: u32 = rd32(ec6.wrapping_add(0x18));
        let f6: extern "thiscall" fn(u32, u32, u32) -> u32 =
            unsafe { core::mem::transmute(sl6 as usize) };
        let mut outslot = 0u32;
        let ans6: u32 = f6(ec6, gp, &mut outslot as *mut u32 as u32);
        if (ans6 as i32) >= 0 {
            let obj2: u32 = outslot;
            let ec7: u32 = rd32(obj2);
            let sl7: u32 = rd32(ec7.wrapping_add(0x20));
            let f7: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
                unsafe { core::mem::transmute(sl7 as usize) };
            let ans7: u32 = f7(ec7, obj2, g32(G_ARG), out2.as_ptr() as u32);
            if (ans7 as i32) >= 0 {
                let count0: u32 = g32(G_BBH) & 0xFFFF;
                if count0 != 0 {
                    let mut base: u32 = g32(G_BBH0).wrapping_add(8);
                    let mut edi = 0u32;
                    loop {
                        let ccnt: u32 = g32(G_BBH) & 0xFFFF;
                        let mut hit = false;
                        if out2[0] == rd32(base.wrapping_sub(8)) {
                            if out2[1] == rd32(base.wrapping_sub(4)) {
                                if out2[2] == rd32(base) {
                                    hit = true;
                                }
                            }
                        }
                        if hit {
                            let gate1: u8 = if g32(G_E6BC) == 0 { 1 } else { 0 };
                            loop {
                                wg32(G_E80, edi);
                                let mut slot3 = 0u32;
                                let _: u32 = lf_checker_rt::callee_cdecl!(5, u32,
                                    &slot3 as *const u32 as u32,
                                    &z0 as *const u32 as u32,
                                    &z1 as *const u32 as u32,
                                    &z2 as *const u32 as u32);
                                let esi3: u32 = slot3;
                                let fc1 = div_c1(esi3);
                                wg32(G_EAC, (if fc1 <= 2 { fc1 } else { 2 }).wrapping_add(1));
                                let fd3 = div_c3(esi3);
                                wg32(G_EB4, if fd3 <= 3 { fd3 } else { 3 });
                                let fd2 = div_c2(esi3);
                                wg32(G_EB0, (if fd2 <= 3 { fd2 } else { 3 }).wrapping_add(1));
                                if ecl != 0 {
                                    wg32(G_E80, early_e80);
                                }
                                let _: u32 = lf_checker_rt::callee_thiscall!(13, u32,
                                    lf_checker_rt::relocated(0x18B6E90));
                                if gate1 == 0 {
                                    break;
                                }
                                let f = f32::from_bits(g32(G_F90));
                                if !(f >= 1.0) {
                                    break;
                                }
                                if edi == 0 {
                                    break;
                                }
                                edi = edi.wrapping_sub(1);
                            }
                            break;
                        }
                        edi = edi.wrapping_add(1);
                        base = base.wrapping_add(0x10);
                        if edi == ccnt {
                            break;
                        }
                    }
                }
            }
            let eax3: u32 = outslot;
            let ec8: u32 = rd32(eax3);
            let sl8: u32 = rd32(ec8.wrapping_add(8));
            let f8: extern "thiscall" fn(u32, u32) -> u32 =
                unsafe { core::mem::transmute(sl8 as usize) };
            let _: u32 = f8(ec8, eax3);
        }
        // The original keeps the clamped sample in its frame, loads it once
        // into edx after the id8 call, and never uses it (edx dies in the
        // next call): the value is unobservable, only the loop's fault is.
        let _dead: u32 = slot20;
        let _: u32 = lf_checker_rt::callee_cdecl!(9, u32, 0);
        let vmode: u32 = g32(G_MODE);
        if vmode == 2 || vmode == 3 || vmode == 1 {
            wg32(G_E80, 0);
            wg32(G_E84, 1);
            wg32(G_E94, 0);
            wg32(G_E90, 0);
            wg32(G_E98, 0);
            wg32(G_E9C, 0);
            wg32(G_E8C, 0);
            wg32(G_EA8, 0);
        }
        let _: u32 = lf_checker_rt::callee_cdecl!(10, u32,);
        let mut i = 0u32;
        while i < 16 {
            wg32(G_BDBC.wrapping_add(i.wrapping_mul(4)),
                g32(G_E80.wrapping_add(i.wrapping_mul(4))));
            i = i.wrapping_add(1);
        }
        let fsave: u32 = g32(G_F90);
        let retval: u32;
        if o0 != 0 {
            let p1: u32 = lf_checker_rt::tls_slot(0);
            let ob: u32 = rd32(p1.wrapping_add(8));
            let vt: u32 = rd32(ob);
            let slt: u32 = rd32(vt.wrapping_add(0xC));
            let ft: extern "thiscall" fn(u32, u32) -> u32 =
                unsafe { core::mem::transmute(slt as usize) };
            retval = ft(ob, o0);
            let _fr: u32 = fsave;
        } else {
            retval = g32(G_EB4);
        }
        let _: u32 = lf_checker_rt::callee_cdecl!(12, u32,);
        retval
    }
});
