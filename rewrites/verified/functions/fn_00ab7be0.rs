// original: 0x00AB7BE0 refresh_player_diff_variant
// Walk the 11 slots of a player texture-difference record like the slot
// refresher, but resolve each hit through the variant pipeline: scan the
// record's variant list, notify the two observers, then publish the slot
// either directly or through the record's virtual hook depending on the
// mode bits. Empty key sets return early; records that select no variant
// on a slot leave that slot alone.
export!(cdecl, rw_00AB7BE0(a0: u32, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32, a6: u32, a7: u32, a8: u32, a9: u32) -> u32 {
    unsafe {
        let _ = a8;
        // Mode bits from the flag word (the original keeps the bit in its
        // own incoming stack slot; a local holds the same value here).
        let direct = (a6 & 3) != 0;
        let flags_bit = ((a6 >> 1) & 1) as u8;
        let inner = *((a0.wrapping_add(0x128)) as *const u32);
        if *(inner as *const u8) == 0 && *((inner.wrapping_add(1)) as *const u8) == 0 {
            return inner;
        }
        for ebp in 0..11u32 {
            let key = *((a2.wrapping_add(ebp).wrapping_add(0x5c)) as *const u8) as u32;
            // First-level hash: slot index -> key record.
            let mut hit1 = 0u32;
            let count1 = *((inner.wrapping_add(0xc)) as *const u16) as u32;
            if count1 != 0 {
                let t1 = *((inner.wrapping_add(8)) as *const u32);
                let mut e = *((t1.wrapping_add((ebp % count1).wrapping_mul(4))) as *const u32);
                while e != 0 {
                    if *(e as *const u32) == ebp {
                        hit1 = e;
                        break;
                    }
                    e = *((e.wrapping_add(0x14)) as *const u32);
                }
            }
            if hit1 == 0 {
                continue;
            }
            let sub = hit1.wrapping_add(4);
            if sub == 0 {
                continue;
            }
            // Second-level hash: name byte -> variant record.
            let mut hit2 = 0u32;
            let count2 = *((sub.wrapping_add(8)) as *const u16) as u32;
            if count2 != 0 {
                let t2 = *((sub.wrapping_add(4)) as *const u32);
                let mut e = *((t2.wrapping_add((key % count2).wrapping_mul(4))) as *const u32);
                while e != 0 {
                    if *(e as *const u32) == key {
                        hit2 = e;
                        break;
                    }
                    e = *((e.wrapping_add(0x8c)) as *const u32);
                }
            }
            if hit2 == 0 {
                continue;
            }
            let e2b = hit2.wrapping_add(4);
            if e2b == 0 || *((e2b) as *const u8) & 1 == 0 {
                continue;
            }
            let vt = *(a0 as *const u32);
            let vslot = ((vt.wrapping_add(0x34)) as *const u32).read();
            let hook: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(vslot as usize);
            let scret = callee_cdecl!(2, u32, hook(a0));
            let edi = callee_cdecl!(
                3, u32, scret, ebp, key,
                a2.wrapping_add(ebp.wrapping_mul(4)).wrapping_add(4)
            );
            if edi == 0 {
                continue;
            }
            let mut done_flag = 0u8;
            let stored = callee_cdecl!(4, u32,);
            if ebp == 0
                && !direct
                && (a7 as u8) != 0
                && *global::<u32>(0x0103EDA8) != 0xFFFFFFFF
            {
                let p1 = *((edi.wrapping_add(0x40)) as *const u32);
                let p2 = *(p1 as *const u32);
                let bx = *(p2 as *const u32);
                let count = *((bx.wrapping_add(0x1a)) as *const u16) as u32;
                let mut idx = 0u32;
                while idx < count {
                    let r1 = callee_thiscall!(5, u32, bx, idx);
                    if *((r1.wrapping_add(8)) as *const u32) == 0 {
                        let r2 = callee_thiscall!(5, u32, bx, idx);
                        if *((r2.wrapping_add(0x44)) as *const u32) != 0 {
                            let al = callee_cdecl!(6, u32,) as u8;
                            let gv = if al != 0 {
                                *global::<u32>(0x0103EDA4)
                            } else {
                                *global::<u32>(0x0103EDA8)
                            };
                            callee_cdecl!(7, u32, gv);
                            done_flag = 1;
                        }
                    }
                    idx += 1;
                }
            }
            if a3 != 0 {
                callee_thiscall!(8, u32, a3, edi, ebp);
                if ebp == 0 && a9 != 0 {
                    callee_thiscall!(9, u32, a3, edi, a9);
                }
            }
            let mut esi_flag = 0u32;
            if *((edi.wrapping_add(0x44)) as *const u32) != 0 && (a5 as u8) == 0 {
                esi_flag = 1;
            }
            if *((edi.wrapping_add(esi_flag.wrapping_mul(4)).wrapping_add(0x40)) as *const u32)
                == 0
            {
                if done_flag != 0 {
                    callee_cdecl!(7, u32, stored);
                }
                continue;
            }
            if !direct {
                if (a8 as u8) != 0 {
                    callee_cdecl!(10, u32,);
                }
                let mut bl = 0u8;
                if ebp == 7 {
                    let ec = *((edi.wrapping_add(8)) as *const u32);
                    if callee_thiscall!(11, u32, ec, relocated(0x00EA5298)) != 0 {
                        bl = 1;
                    }
                }
                if a3 != 0 && callee_thiscall!(12, u32, a3) as u8 != 0 {
                    esi_flag = 0;
                }
                if bl != 0 {
                    callee_cdecl!(13, u32, edi, 0, a1, a4, esi_flag);
                } else {
                    let vt2 = *(edi as *const u32);
                    let s2 = ((vt2.wrapping_add(0x28)) as *const u32).read();
                    let f2: extern "thiscall" fn(u32, u32, u32, u32, u32) -> u32 =
                        core::mem::transmute(s2 as usize);
                    f2(edi, 0, a1, a4, esi_flag);
                }
                if (a8 as u8) != 0 {
                    callee_cdecl!(15, u32,);
                }
            } else {
                let ab = if flags_bit == 0 { 2 } else { 1 };
                let vt3 = *(edi as *const u32);
                let s3 = ((vt3.wrapping_add(0x30)) as *const u32).read();
                let f3: extern "thiscall" fn(u32, u32, u32, u32, u32, u32) -> u32 =
                    core::mem::transmute(s3 as usize);
                f3(edi, 0, a1, esi_flag, ab, a4);
            }
            if done_flag != 0 {
                callee_cdecl!(7, u32, stored);
            }
        }
        // The trailing slots never resolve, so every long run ends with the
        // hash-miss residue in the return register.
        0
    }
});
