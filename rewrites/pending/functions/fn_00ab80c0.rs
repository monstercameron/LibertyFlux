// original: 0x00AB80C0 refresh_player_diff_slots
// Walk the 11 player texture-difference slots of a record: for each slot
// whose name key resolves through the two-level hash, rebuild that slot's
// resource name and resolve it through the file helpers. Marks the record
// active first; records with an empty key set are marked and left alone.
export!(cdecl, rw_00AB80C0(a0: u32, a1: u32, a2: u32) -> u32 {
    unsafe {
        let tab0 = global::<u32>(0x01295CD8);
        let entry = *tab0.add(a0 as usize);
        let edi = a1;
        let flag = (edi.wrapping_add(0x72)) as *mut u8;
        *flag |= 4;
        let inner = *((entry.wrapping_add(0x128)) as *const u32);
        if *(inner as *const u8) == 0 {
            return inner;
        }
        // Object hook, first virtual slot group.
        let vt = *(entry as *const u32);
        let vslot = ((vt.wrapping_add(0x34)) as *const u32).read();
        let hook: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(vslot as usize);
        let probe = hook(entry);
        let carried = callee_cdecl!(2, u32, probe);
        let dflag = global::<u32>(0x015F8BA4);
        let dobj = *dflag;
        let off = *((dobj.wrapping_add(4)) as *const u32);
        let mark = *((probe.wrapping_add(off)) as *const u8);
        let base = if mark & 0x80 != 0 {
            0
        } else {
            let mult = *((dobj.wrapping_add(0xc)) as *const u32);
            mult.wrapping_mul(probe).wrapping_add(*(dobj as *const u32))
        };
        // Null-based read faults exactly like the original when mark is set.
        let arg1val = ((base.wrapping_add(0xc)) as *const u32).read();
        let tmpl0 = *global::<u64>(0x00EA5248);
        let tmpl1 = *global::<u16>(0x00EA5250);
        let strtab = global::<u32>(0x0103EDB0);
        let name_src = edi.wrapping_add(0x5c);
        let mut ebp = edi.wrapping_add(0x30);
        let mut si = 0u32;
        while si < 11 {
            let key = *((name_src.wrapping_add(si)) as *const u8) as u32;
            // First-level hash: slot index -> key record.
            let mut hit1 = 0u32;
            let count1 = *((inner.wrapping_add(0xc)) as *const u16) as u32;
            if count1 != 0 {
                let t1 = *((inner.wrapping_add(8)) as *const u32);
                let mut e = *((t1.wrapping_add((si % count1).wrapping_mul(4))) as *const u32);
                while e != 0 {
                    if *(e as *const u32) == si {
                        hit1 = e;
                        break;
                    }
                    e = *((e.wrapping_add(0x14)) as *const u32);
                }
            }
            if hit1 != 0 {
                let sub = hit1.wrapping_add(4);
                if sub != 0 {
                    // Second-level hash: name byte -> variant record.
                    let mut hit2 = 0u32;
                    let count2 = *((sub.wrapping_add(8)) as *const u16) as u32;
                    if count2 != 0 {
                        let t2 = *((sub.wrapping_add(4)) as *const u32);
                        let mut e =
                            *((t2.wrapping_add((key % count2).wrapping_mul(4))) as *const u32);
                        while e != 0 {
                            if *(e as *const u32) == key {
                                hit2 = e;
                                break;
                            }
                            e = *((e.wrapping_add(0x8c)) as *const u32);
                        }
                    }
                    if hit2 != 0 {
                        let e2b = hit2.wrapping_add(4);
                        if e2b != 0 && *((e2b) as *const u8) & 1 != 0 {
                            callee_cdecl!(3, u32, carried, si, key, ebp.wrapping_sub(0x2c));
                            let cbyte =
                                *((edi.wrapping_add(si).wrapping_add(0x67)) as *const u8);
                            let r4 = callee_thiscall!(4, u32, inner, si, key, cbyte as u32);
                            let s = *strtab.add(si as usize);
                            let mut buf = [0u8; 12];
                            buf[0] = *(s as *const u8);
                            buf[1] = *((s.wrapping_add(1)) as *const u8);
                            buf[2] = *((s.wrapping_add(2)) as *const u8);
                            buf[3] = *((s.wrapping_add(3)) as *const u8);
                            buf[4] = (tmpl0 >> 32) as u8;
                            buf[5] = (tmpl0 >> 40) as u8;
                            buf[6] = (tmpl0 >> 48) as u8;
                            buf[7] = (tmpl0 >> 56) as u8;
                            buf[8] = tmpl1 as u8;
                            buf[9] = (tmpl1 >> 8) as u8;
                            let alword = (cbyte.wrapping_add(0x61)) as u32;
                            callee_cdecl!(
                                5, u32, a2, arg1val, si, buf.as_ptr() as u32, key, alword,
                                r4, ebp
                            );
                        }
                    }
                }
            }
            si += 1;
            ebp = ebp.wrapping_add(4);
        }
        edi.wrapping_add(0x5c)
    }
});
