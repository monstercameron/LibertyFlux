// original: 0x00a330d0 scan_report_and_arm
/// Scan the entity's model table, reporting direct hits and arming the rest.
///
/// `this` is the entity; `p0` selects the record, `p1`/`p2` are report
/// payloads. Rows whose gate answers 1 are reported through the shared
/// dispatcher when their record matches; rows whose gate answers 2 are
/// resolved, basis-checked and projected, then run the shared setup call.
/// Returns the value the original leaves in `eax` on the taken exit path.
export!(thiscall, rw_00a330d0(this: u32, p0: u32, p1: u32, p2: u32) -> u32 {
    unsafe {
        const PARAM_TABLE: u32 = 0x0129_5cd8;
        const FLAG_TABLE: u32 = 0x012F_8498;
        const DISPATCHER: u32 = 0x0171_DEE8;

        /// Project one item corner through a resolved basis into three components.
        ///
        /// `item` carries the corner triple at +4/+8/+0xC, `basis` the matrix.
        /// Operation order matches the original exactly so NaN payloads agree.
        #[inline(always)]
        fn project3(item: u32, basis: u32) -> [f32; 3] {
            unsafe {
                let it = item as *const f32;
                let a = it.add(1).read();
                let b = it.add(2).read();
                let c = it.add(3).read();
                let m = basis as *const f32;
                let m0 = m.read();
                let m4 = m.add(1).read();
                let m8 = m.add(2).read();
                let m10 = m.add(4).read();
                let m14 = m.add(5).read();
                let m18 = m.add(6).read();
                let m20 = m.add(8).read();
                let m24 = m.add(9).read();
                let m28 = m.add(10).read();
                let m30 = m.add(12).read();
                let m34 = m.add(13).read();
                let m38 = m.add(14).read();
                let mut v0 = m10 * b;
                v0 += m0 * a;
                v0 += m20 * c;
                v0 += m30;
                let mut v1 = m14 * b;
                v1 += m4 * a;
                v1 += m24 * c;
                v1 += m34;
                let mut v2 = m18 * b;
                v2 += m8 * a;
                v2 += m28 * c;
                v2 += m38;
                [v0, v1, v2]
            }
        }
        let slot = ((this as *const u8).add(0x2e) as *const u16).read();
        if slot == 0xffff {
            return 0xffff;
        }
        let table = global::<u32>(PARAM_TABLE).offset(slot as isize).read();
        if ((table as *const u8).add(0x5b).read()) & 8 == 0 {
            // eax still holds the table here.
            return table;
        }
        let mut n: u32 = callee_thiscall!(1, u32, table);
        if (n as i32) <= 0 {
            return n;
        }
        let mut i: u32 = 0;
        loop {
            let item: u32 = callee_thiscall!(2, u32, table, i);
            let vt = (item as *const u32).read() as *const u32;
            let gate: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(vt.add(1).read() as usize);
            let g = gate(item) as u8;
            if g == 1 {
                let geta: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(vt.add(7).read() as usize);
                let res = geta(item);
                let mut fire = true;
                if ((res as *const u8).add(0x39).read()) == (p1 as u8) {
                    fire = false;
                }
                if fire && ((res as *const u32).add(9).read()) != 3 {
                    fire = false;
                }
                if fire {
                    let c = ((res as *const u32).add(10).read());
                    if c != p0 && c != 0xffff_ffff {
                        fire = false;
                    }
                }
                if fire {
                    let disp = lf_checker_rt::relocated(DISPATCHER);
                    let _: u32 = callee_thiscall!(12, u32, disp, this, res, p0, p1, p2);
                }
            } else {
                // NOTE: the original re-checks the same gate slot here; both
                // answers come from the same per-trial script value.
                let gate2: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(vt.add(1).read() as usize);
                if (gate2(item) as u8) == 2 {
                    let getb: extern "thiscall" fn(u32) -> u32 =
                        core::mem::transmute(vt.add(9).read() as usize);
                    let res = getb(item);
                    let dl = ((res as *const u8).add(0x2d).read());
                    let mut proceed = true;
                    if dl == (p1 as u8) {
                        proceed = false;
                    }
                    if proceed && ((res as *const u32).add(9).read()) != 1 {
                        proceed = false;
                    }
                    if proceed {
                        let c = ((res as *const u32).add(10).read());
                        if c != p0 && c != 0xffff_ffff {
                            proceed = false;
                        }
                    }
                    if proceed {
                        let flags = ((this as *const u32).add(10)).read();
                        let mut gated = true;
                        if (flags & 0x0010_0000) != 0 {
                            let bi = ((res as *const u32).add(8)).read();
                            let b = global::<u8>(FLAG_TABLE)
                                .offset((bi as i32).wrapping_mul(0x70) as isize)
                                .read();
                            if b == 0 {
                                gated = false;
                            }
                        }
                        if gated {
                            // The frame flag byte at X+7 neighbours the loop
                            // counter's low bytes; mirror the snapshot word.
                            let mut flag: u32 = (i & 0x00ff_ffff) << 8;
                            let arg1 = if dl != 0 { p2 } else { p0 };
                            let r: u32 = callee_cdecl!(
                                6, u32, this, arg1, 1, &mut flag as *mut u32 as u32
                            );
                            if r != 0 {
                                let a45: u32 = callee_thiscall!(7, u32, r);
                                if (a45 as u8) == 0 {
                                    let mut vec = project3(item, r);
                                    let f2 = ((this as *const u32).add(10)).read();
                                    let mut edx = 0u32;
                                    if (f2 & 0x3c0) == 0x100 {
                                        let d = ((this as *const u32).add(0x9f)).read();
                                        if d != 0 {
                                            edx = d;
                                        } else {
                                            let e = ((this as *const u32).add(0xa0)).read();
                                            if e != 0 {
                                                let ef = ((e as *const u32).add(10)).read();
                                                if (ef & 0x3c0) == 0x100 {
                                                    edx = ((e as *const u32).add(0x9f)).read();
                                                }
                                            }
                                        }
                                    }
                                    let isnull = if edx == 0 { 1u32 } else { 0u32 };
                                    // Slot 1 re-pushes the stashed report
                                    // argument (arg1 above), not a zero.
                                    let p: u32 = callee_cdecl!(
                                        8, u32, this, arg1, 0, isnull, edx, 0,
                                        0xffff_ffffu32
                                    );
                                    let pe = p.wrapping_add(0x20);
                                    let idx2 = ((res as *const u32).add(8)).read();
                                    // Slot 8 is the push whose value the next
                                    // instruction overwrites with -1.0f; the
                                    // pushed ecx itself is never observed.
                                    // Slot 13 re-pushes the stashed arg1.
                                    let _: u32 = callee_cdecl!(
                                        9, u32, 0, 0, idx2, 0x3f80_0000u32,
                                        vec.as_mut_ptr() as u32, 0, 0, 1, 0xbf80_0000u32,
                                        0, 0, pe, this, arg1, 0, isnull, edx, 0,
                                        0xffff_ffffu32
                                    );
                                    let a7: u32 = callee_cdecl!(10, u32,);
                                    // a8==0 loops back at once, skipping the
                                    // arming stores below.
                                    let mut skip_tail = false;
                                    if (a7 as u8) != 0 {
                                        let a8: u32 = callee_cdecl!(11, u32,);
                                        if (a8 as u8) == 0 {
                                            skip_tail = true;
                                        }
                                    }
                                    if !skip_tail {
                                        let f = (this as *mut u32).add(10);
                                        f.write(f.read() | 0x0010_0000);
                                        let masked =
                                            ((this as *const u32).add(10)).read() & 0x3c0;
                                        if masked == 0x100 {
                                            let e = ((this as *const u32).add(0xa0)).read();
                                            if e != 0 {
                                                let ef = (e as *mut u32).add(10);
                                                ef.write(ef.read() | 0x0010_0000);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            i = i.wrapping_add(1);
            n = callee_thiscall!(1, u32, table);
            if !((i as i32) < (n as i32)) {
                return n;
            }
        }
    }
});
