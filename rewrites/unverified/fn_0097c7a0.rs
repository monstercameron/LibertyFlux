// original: 0x0097c7a0 POCKETS_MONEY
//! Audio event dispatcher (thiscall/2: ECX=this, event id, flag byte; callee pops 8).
//!
//! Behaviour: returns void. Rejects the call when the linked object's status
//! word has bit 23 set. Classifies the event through callee 1, runs a
//! level-smoothing pass over this+0x144 (a data-driven float when the
//! smoothing gate allows, else a decayed copy, clamped into range), then
//! dispatches on the event id through a series of gated blocks, each of
//! which builds small frame structs (initialised by struct-filling callees),
//! issues engine requests, and threads callee answers into later blocks.
//! Notable blocks: flag-bit-selected request variants, an indexed-table
//! path for events 8..13, a two-float query whose answers feed a scaled
//! request, a one-time global initialisation (flag/value pair) guarding a
//! scaled path, a validated-pointer request, two struct-fill blocks for
//! events 6/7 and 0/1/14/15/16/17, a bitmask-selected request, and a
//! float-threshold-gated final request that refreshes the object register.
//! A three-global gate at the end selects a logging call, and a final event
//! dispatch selects a notification call with argument 0 or 1.
//!
//! Floating point is single precision throughout with exact operand order;
//! ordered comparisons treat NaN as greater-or-equal (branch taken), the
//! int conversion yields i32::MIN outside range or for NaN, and the event
//! bit test wraps the shift count to five bits. Exit EAX is the last
//! callee answer or entry residue, so the contract compares no return
//! channel; behaviour is verified through heap, globals and calls.
export!(thiscall, rw_0097c7a0(this: u32, event: u32, flag: u32) -> u32 {
    unsafe {
        // Frame: 59 words of scratch; byte and half-word accesses share
        // words with the surrounding dwords exactly like the original.
        let mut fr = [0u32; 59];
        #[inline(always)]
        fn wrf(fr: &mut [u32; 59], w: usize, v: f32) {
            fr[w] = v.to_bits();
        }
        #[inline(always)]
        fn rdf(fr: &[u32; 59], w: usize) -> f32 {
            f32::from_bits(fr[w])
        }
        #[inline(always)]
        fn wrb(fr: &mut [u32; 59], o: usize, v: u8) {
            let s = (o & 3) * 8;
            fr[o >> 2] = (fr[o >> 2] & !(0xff << s)) | ((v as u32) << s);
        }
        #[inline(always)]
        fn wrh(fr: &mut [u32; 59], o: usize, v: u16) {
            let s = (o & 3) * 8;
            fr[o >> 2] = (fr[o >> 2] & !(0xffff << s)) | ((v as u32) << s);
        }
        #[inline(always)]
        fn fp(fr: &mut [u32; 59], w: usize) -> u32 {
            fr.as_mut_ptr().wrapping_add(w) as u32
        }
        #[inline(always)]
        fn fadd(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
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
        fn cvt(x: f32) -> i32 {
            if x.is_nan() || x >= 2147483648.0 || x < -2147483648.0 {
                i32::MIN
            } else {
                x as i32
            }
        }
        #[inline(always)]
        fn g32(va: u32) -> u32 {
            *global::<u32>(va)
        }
        #[inline(always)]
        fn gf(va: u32) -> f32 {
            *global::<f32>(va)
        }
        const F_G1: u32 = 0x00E78588;
        const F_G2: u32 = 0x00E7858C;
        const F_E8D710: u32 = 0x00E8D710;
        const F_E8D718: u32 = 0x00E8D718;
        const F_8670: u32 = 0x00FE8670;
        const F_870C: u32 = 0x00FE870C;
        const F_8758: u32 = 0x00FE8758;
        const F_87D0: u32 = 0x00FE87D0;
        const F_881C: u32 = 0x00FE881C;
        const F_88E8: u32 = 0x00FE88E8;
        const F_8960: u32 = 0x00FE8960;
        const F_8A24: u32 = 0x00FE8A24;
        const F_8AB8: u32 = 0x00FE8AB8;
        const F_DDE98: u32 = 0x012DDE98;
        const G_037720: u32 = 0x01037720;
        const G_1F7060: u32 = 0x011F7060;
        const G_2088B4: u32 = 0x012088B4;
        const G_2312E0: u32 = 0x012312E0;
        const G_231318: u32 = 0x01231318;
        const G_23175C: u32 = 0x0123175C;
        const G_231760: u32 = 0x01231760;
        const G_F1C040: u32 = 0x00F1C040;
        const T_12E8: u32 = 0x012312E8;

        let mut edi = this;
        let mut esi = event;
        let mut eax: u32 = 0;
        let mut ecx: u32 = 0;
        let mut edx: u32 = 0;
        let mut x0: f32 = 0.0;
        let mut x1: f32 = 0.0;
        let mut x2: f32 = 0.0;
        let mut x3: f32 = 0.0;

        fr[0x2f] = edi;
        eax = *((edi + 0x120) as *const u32);
        if *((eax + 0x28) as *const u32) & 0x800000 != 0 {
            return 0;
        }
        // Classify the event, then run the follow-up query.
        eax = callee_stdcall!(1, u32, esi);
        fr[0x0f] = eax;
        callee_thiscall!(2, u32, edi, eax, fp(&mut fr, 0x10));
        // Level smoothing over this+0x144.
        edx = *((edi + 0x120) as *const u32);
        ecx = *((edx + 0x24) as *const u32).wrapping_shr(0x1b);
        let use_data: bool;
        if (ecx as u8) & 1 != 0 {
            use_data = false;
        } else if *((edx + 0x218) as *const u8) != 0 {
            use_data = true;
        } else if *((edx + 0x219) as *const u8) == 0 {
            use_data = true;
        } else {
            eax = callee_thiscall!(3, u32, relocated(0x01165880));
            use_data = (eax as u8) == 0;
        }
        if use_data {
            x0 = gf(F_DDE98);
            if x0 > gf(F_881C) {
                x0 = fmul(x0, gf(F_87D0));
                x0 = fadd(x0, *((edi + 0x144) as *const f32));
            } else {
                x0 = fsub(*((edi + 0x144) as *const f32), gf(F_8758));
            }
        } else {
            x0 = fsub(*((edi + 0x144) as *const f32), gf(F_8758));
        }
        x1 = x0;
        *((edi + 0x144) as *mut f32) = x0;
        x0 = 0.0;
        if !(0.0 > x1) {
            x0 = gf(F_88E8);
            if !(x1 > x0) {
                x0 = x1;
            }
        }
        *((edi + 0x144) as *mut f32) = x0;
        // Small-event flag gate.
        if esi <= 5 && (flag & 0xff) == 0 {
            eax = *((edi + 0x120) as *const u32);
            if *((eax + 0x26c) as *const u8) & 1 == 0 {
                return 0;
            }
        }
        // First dispatch: two request-variant blocks sharing one frame shape.
        if esi == 0 || esi == 0xa || esi == 0xe || esi == 0x10 {
            callee_thiscall!(4, u32, fp(&mut fr, 0x14));
            ecx = *((edi + 0x120) as *const u32);
            x0 = *((edi + 0xbc) as *const f32);
            fr[0x17] = ecx.wrapping_add(0x780);
            fr[0x1c] = *((edi + 8) as *const u32);
            eax = *((edi + 0x128) as *const u8) as u32;
            wrf(&mut fr, 0x14, x0);
            let mut ck: u32 = 0;
            let mut have = false;
            if eax & 1 != 0 {
                ck = 0x00E8CD9C;
                have = true;
            } else if eax & 4 != 0 && (*((ecx + 0xb88) as *const u32) as i32) >= 10
            {
                ck = 0x00E8CDAC;
                have = true;
            }
            if have {
                callee_thiscall!(
                    5,
                    u32,
                    edi,
                    relocated(ck),
                    fp(&mut fr, 0x1a),
                    0xFFFFFFFF,
                    0,
                    0
                );
            }
            if *((edi + 0x88) as *const u32) == 1 {
                callee_thiscall!(
                    5,
                    u32,
                    edi,
                    relocated(0x00E8CDBC),
                    fp(&mut fr, 0x1a),
                    0xFFFFFFFF,
                    0,
                    0
                );
            }
        } else if esi == 1 || esi == 0xb || esi == 0xf || esi == 0x11 {
            callee_thiscall!(4, u32, fp(&mut fr, 0x14));
            ecx = *((edi + 0x120) as *const u32);
            x0 = *((edi + 0xbc) as *const f32);
            fr[0x17] = ecx.wrapping_add(0x780);
            fr[0x1c] = *((edi + 8) as *const u32);
            eax = *((edi + 0x128) as *const u8) as u32;
            wrf(&mut fr, 0x14, x0);
            let mut ck: u32 = 0;
            let mut have = false;
            if eax & 2 != 0 {
                ck = 0x00E8CDC4;
                have = true;
            } else if eax & 8 != 0 && (*((ecx + 0xb88) as *const u32) as i32) >= 10
            {
                ck = 0x00E8CDD4;
                have = true;
            }
            if have {
                callee_thiscall!(
                    5,
                    u32,
                    edi,
                    relocated(ck),
                    fp(&mut fr, 0x1a),
                    0xFFFFFFFF,
                    0,
                    0
                );
            }
            if *((edi + 0x88) as *const u32) == 1 {
                callee_thiscall!(
                    5,
                    u32,
                    edi,
                    relocated(0x00E8CDE4),
                    fp(&mut fr, 0x1a),
                    0xFFFFFFFF,
                    0,
                    0
                );
            }
        }
        // Indexed-table path for events 8..13.
        if esi.wrapping_sub(8) <= 5 {
            ecx = g32(G_2312E0);
            if ecx != 0 {
                eax = callee_thiscall!(
                    6,
                    u32,
                    edi,
                    ecx,
                    esi.wrapping_sub(8),
                    *((edi + 0x78) as *const u32)
                );
                esi = eax;
                callee_thiscall!(4, u32, fp(&mut fr, 0x14));
                eax = fr[0x0b];
                x0 = *((edi + 0xc0) as *const f32);
                fr[0x21] = eax;
                fr[0x19] = fp(&mut fr, 0x10);
                eax = callee_cdecl!(7, u32, x0.to_bits());
                x0 = f32::from_bits(eax);
                x0 = fmul(x0, gf(F_G1));
                wrb(&mut fr, 0x96, rdb_fr(&fr, 0x96) | 4);
                x0 = fmul(x0, gf(F_G2));
                eax = cvt(x0) as u32;
                x0 = *((edi + 0xbc) as *const f32);
                fr[0x1b] = eax;
                fr[0x22] = *((edi + 8) as *const u32);
                wrf(&mut fr, 0x1e, x0);
                callee_thiscall!(
                    8,
                    u32,
                    edi,
                    esi,
                    fp(&mut fr, 0x1a),
                    0xFFFFFFFF,
                    0,
                    0
                );
                esi = event;
            }
        }
        // Object-presence gate; the rest of the middle may exit to the tail.
        'mid: {
            eax = *((edi + 0xb8) as *const u32);
            if eax == 0 {
                break 'mid;
            }
            eax = callee_thiscall!(
                9,
                u32,
                relocated(0x0115D9A0),
                ((eax + 0x21) as *const u32).read_unaligned()
            );
            edx = eax;
            eax = *((edi + 0x120) as *const u32);
            if *((edi + 0x1a0) as *const u8) != 0 {
                edx = g32(G_231318);
            }
            fr[0x0c] = edx;
            if *((eax + 0x26c) as *const u8) & 4 != 0
                && *((eax + 0xb30) as *const u32) != 0
            {
                if *((eax + 0x26c) as *const u8) & 4 != 0 {
                    eax = *((eax + 0xb30) as *const u32);
                } else {
                    eax = 0;
                }
                eax = *((eax + 0xad4) as *const u32);
                if eax != 0 {
                    eax = callee_thiscall!(
                        9,
                        u32,
                        relocated(0x0115D9A0),
                        ((eax + 0x21) as *const u32).read_unaligned()
                    );
                    edx = eax;
                    fr[0x0c] = eax;
                }
            }
            if edx == 0 {
                break 'mid;
            }
            // Two-float query, or constants for events 16/17.
            if esi == 0x11 || esi == 0x10 {
                eax = *((edi + 0x120) as *const u32);
                x0 = gf(F_8960);
                wrf(&mut fr, 0x09, x0);
                if *((eax + 0x218) as *const u8) == 0
                    && *((eax + 0x219) as *const u8) != 0
                {
                    x0 = gf(F_E8D710);
                    wrf(&mut fr, 0x09, x0);
                }
                x1 = gf(F_88E8);
            } else {
                callee_thiscall!(
                    10,
                    u32,
                    edi,
                    fp(&mut fr, 0x06),
                    fp(&mut fr, 0x08)
                );
                x0 = rdf(&fr, 0x04);
                x1 = rdf(&fr, 0x08);
                edx = fr[0x0c];
                wrf(&mut fr, 0x09, x0);
            }
            wrf(&mut fr, 0x08, x1);
            if esi == 2 || esi == 3 {
                x0 = fsub(x0, gf(F_8A24));
                wrf(&mut fr, 0x09, x0);
            }
            // Bounded-index scaled-request block.
            'pre480: {
                ecx = *((edi + 0x78) as *const u32);
                if (ecx as i32) <= 0 || (ecx as i32) > 6 {
                    break 'pre480;
                }
                eax = *((edi + 0xb8) as *const u32);
                eax = *((eax + 0x25) as *const u8) as u32;
                wrb(&mut fr, 0x2b, eax as u8);
                if eax == 0 {
                    break 'pre480;
                }
                if !((esi as i32) <= 5 || esi.wrapping_sub(0xe) <= 3) {
                    break 'pre480;
                }
                eax = callee_thiscall!(6, u32, edi, edx, esi, ecx);
                fr[0x04] = eax;
                callee_thiscall!(4, u32, fp(&mut fr, 0x14));
                fr[0x1c] = *((edi + 8) as *const u32);
                wrb(&mut fr, 0x96, rdb_fr(&fr, 0x96) | 4);
                ecx = *((edi + 0x78) as *const u32);
                ecx = *((relocated(T_12E8) + ecx.wrapping_mul(4)) as *const u32);
                fr[0x1b] = ecx;
                ecx = rdb_fr(&fr, 0x2b) as u32;
                x0 = ecx as f32;
                x0 = fmul(x0, gf(F_870C));
                eax = callee_cdecl!(11, u32, x0.to_bits());
                x0 = f32::from_bits(eax);
                wrf(&mut fr, 0x0f, x0);
                x1 = *((edi + 0xbc) as *const f32);
                x1 = fadd(x1, rdf(&fr, 0x0b));
                x2 = rdf(&fr, 0x0f);
                x0 = rdf(&fr, 0x0a);
                x2 = fadd(x2, x1);
                fr[0x19] = fp(&mut fr, 0x12);
                wrf(&mut fr, 0x14, x2);
                eax = callee_cdecl!(7, u32, x0.to_bits());
                x0 = f32::from_bits(eax);
                x0 = fmul(x0, gf(F_G1));
                x0 = fmul(x0, gf(F_G2));
                eax = cvt(x0) as u32;
                fr[0x1b] = eax;
                eax = fr[0x11];
                fr[0x27] = eax;
                eax = fr[0x0c];
                callee_thiscall!(
                    8,
                    u32,
                    edi,
                    eax,
                    fp(&mut fr, 0x1a),
                    0xFFFFFFFF,
                    0,
                    0
                );
                if !(esi == 0
                    || esi == 0x10
                    || esi == 0x11
                    || esi == 1
                    || esi == 0xe
                    || esi == 0xf)
                {
                    break 'pre480;
                }
                if *((edi + 0x78) as *const u32) == 4 {
                    ecx = *((edi + 0x120) as *const u32);
                    callee_thiscall!(
                        12,
                        u32,
                        ecx.wrapping_add(0x570),
                        1,
                        fp(&mut fr, 0x11),
                        fp(&mut fr, 0x39),
                        0
                    );
                    x0 = rdf(&fr, 0x0d);
                    if x0 > gf(F_E8D718) {
                        eax = fr[0x37];
                        x0 = fsub(x0, gf(F_8AB8));
                        fr[0x22] = eax;
                        x0 = fadd(x0, rdf(&fr, 0x1a));
                        eax = fr[0x0e];
                        wrf(&mut fr, 0x20, x0);
                        callee_thiscall!(
                            13,
                            u32,
                            edi,
                            eax,
                            fp(&mut fr, 0x10),
                            fp(&mut fr, 0x1a),
                            0xFFFFFFFF,
                            0,
                            0
                        );
                        if fr[0x08] != 0 {
                            eax = callee_thiscall!(
                                14,
                                u32,
                                relocated(0x0115DEF0),
                                0
                            );
                            x1 = *((eax) as *const f32);
                            x0 = rdf(&fr, 0x10);
                            x2 = *((eax + 4) as *const f32);
                            x3 = *((eax + 8) as *const f32);
                            x0 = fsub(x0, x1);
                            wrf(&mut fr, 0x08, x1);
                            wrf(&mut fr, 0x34, x0);
                            x0 = rdf(&fr, 0x15);
                            x0 = fsub(x0, x2);
                            wrf(&mut fr, 0x3a, x2);
                            wrf(&mut fr, 0x12, x3);
                            wrf(&mut fr, 0x35, x0);
                            x0 = rdf(&fr, 0x16);
                            x0 = fsub(x0, x3);
                            wrf(&mut fr, 0x36, x0);
                            callee_thiscall!(
                                15,
                                u32,
                                fp(&mut fr, 0x34),
                                0x3F490FDB,
                                0x7a
                            );
                            x0 = rdf(&fr, 0x30);
                            x0 = fadd(x0, rdf(&fr, 0x04));
                            ecx = fr[0x08];
                            wrf(&mut fr, 0x32, x0);
                            x0 = rdf(&fr, 0x33);
                            x0 = fadd(x0, rdf(&fr, 0x38));
                            wrf(&mut fr, 0x33, x0);
                            x0 = rdf(&fr, 0x34);
                            x0 = fadd(x0, rdf(&fr, 0x10));
                            wrf(&mut fr, 0x34, x0);
                            callee_thiscall!(16, u32, ecx, fp(&mut fr, 0x30));
                            ecx = fr[0x08];
                            callee_thiscall!(17, u32, ecx, 0, 0, 0);
                        }
                    }
                }
                // Threshold-gated scaled path with one-time init.
                x0 = *((edi + 0x144) as *const f32);
                x1 = gf(F_870C);
                if x0 > x1 {
                    eax = g32(G_231760);
                    if eax & 1 == 0 {
                        eax |= 1;
                        *global::<u32>(G_231760) = eax;
                        eax = callee_cdecl!(18, u32, relocated(0x00E8CDF0), 0);
                        esi = eax;
                        x1 = gf(F_870C);
                        *global::<u32>(G_23175C) = esi;
                    } else {
                        esi = g32(G_23175C);
                    }
                    eax = *((edi + 0xb8) as *const u32);
                    wrb(&mut fr, 0x94, 0xff);
                    eax = *((eax + 0x25) as *const u8) as u32;
                    x0 = eax as f32;
                    x0 = fmul(x0, x1);
                    x0 = fmul(x0, *((edi + 0x144) as *const f32));
                    eax = callee_cdecl!(11, u32, x0.to_bits());
                    x0 = f32::from_bits(eax);
                    wrf(&mut fr, 0x10, x0);
                    x0 = *((edi + 0xbc) as *const f32);
                    x0 = fadd(x0, rdf(&fr, 0x0b));
                    x1 = rdf(&fr, 0x10);
                    x1 = fadd(x1, x0);
                    wrf(&mut fr, 0x1e, x1);
                    callee_thiscall!(
                        8,
                        u32,
                        edi,
                        esi,
                        fp(&mut fr, 0x14),
                        0xFFFFFFFF,
                        0,
                        0
                    );
                    esi = event;
                }
                // Validated-pointer request.
                if esi == 0xf
                    || esi == 0xe
                    || *(((*((edi + 0x120) as *const u32)) + 0xb80) as *const u32)
                        == 4
                {
                    ecx = *((edi + 0x120) as *const u32);
                    eax = callee_thiscall!(19, u32, ecx.wrapping_add(0x2b0), 1);
                    if (eax as u8) != 0 {
                        ecx = *((edi + 0x120) as *const u32);
                        eax = callee_thiscall!(20, u32, ecx.wrapping_add(0x2b0));
                        if eax != 0 {
                            eax = *((eax + 0x10) as *const u32);
                            if eax != 0 {
                                x0 = *((edi + 0xbc) as *const f32);
                                x0 = fadd(x0, rdf(&fr, 0x09));
                                wrb(&mut fr, 0xb4, 0xff);
                                wrf(&mut fr, 0x1c, x0);
                                fr[0x1d] = 0;
                                eax = ((eax + 0x2a) as *const u32).read_unaligned();
                                callee_thiscall!(
                                    8,
                                    u32,
                                    edi,
                                    eax,
                                    fp(&mut fr, 0x1a),
                                    0xFFFFFFFF,
                                    0,
                                    0
                                );
                            }
                        }
                    }
                }
            } // 'pre480
            // Struct-fill block for events 6/7.
            if esi == 6 || esi == 7 {
                eax = *((edi + 0xb4) as *const u32);
                if eax != 0 && *((eax + 0x38) as *const u32) != 0 {
                    callee_thiscall!(21, u32, fp(&mut fr, 0x14));
                    eax = *((edi + 0xac) as *const u32);
                    x0 = rdf(&fr, 0x10);
                    fr[0x26] = eax;
                    eax = *((edi + 0xb0) as *const u16) as u32;
                    wrh(&mut fr, 0xa2, eax as u16);
                    eax = *((edi + 0xb4) as *const u32);
                    eax = *((eax + 0x38) as *const u32);
                    wrf(&mut fr, 0x1c, x0);
                    x0 = rdf(&fr, 0x15);
                    fr[0x1a] = eax;
                    wrf(&mut fr, 0x1f, x0);
                    x0 = rdf(&fr, 0x18);
                    wrf(&mut fr, 0x22, x0);
                    callee_thiscall!(
                        22,
                        u32,
                        relocated(0x012202E0),
                        fp(&mut fr, 0x1a),
                        0x3DCCCCCD,
                        0,
                        0
                    );
                }
            }
            eax = *((edi + 0xb8) as *const u32);
            if *((eax + 0x26) as *const u8) != 0 {
                if esi == 0 || esi == 1 || esi == 0xe || esi == 0xf || esi == 0x10 || esi == 0x11
                {
                    eax = *((edi + 0xb4) as *const u32);
                    if eax != 0 && *((eax + 0x38) as *const u32) != 0 {
                        callee_thiscall!(21, u32, fp(&mut fr, 0x14));
                        eax = *((edi + 0xac) as *const u32);
                        x0 = rdf(&fr, 0x10);
                        fr[0x26] = eax;
                        eax = *((edi + 0xb0) as *const u16) as u32;
                        wrh(&mut fr, 0xa2, eax as u16);
                        eax = *((edi + 0xb4) as *const u32);
                        eax = *((eax + 0x38) as *const u32);
                        wrf(&mut fr, 0x1a, x0);
                        x0 = rdf(&fr, 0x13);
                        wrf(&mut fr, 0x1b, x0);
                        x0 = rdf(&fr, 0x14);
                        fr[0x16] = eax;
                        wrf(&mut fr, 0x1c, x0);
                        eax = *((edi + 0xb8) as *const u32);
                        eax = *((eax + 0x26) as *const u8) as u32;
                        x0 = eax as f32;
                        x0 = fmul(x0, gf(F_870C));
                        x0 = fmul(x0, gf(F_8960));
                        callee_thiscall!(
                            22,
                            u32,
                            relocated(0x012202E0),
                            fp(&mut fr, 0x1a),
                            x0.to_bits(),
                            0,
                            0
                        );
                    }
                }
            }
            // Bitmask-selected request.
            eax = 1u32.wrapping_shl(esi);
            if *((edi + 0x74) as *const u32) & eax != 0 {
                callee_thiscall!(4, u32, fp(&mut fr, 0x14));
                eax = *((edi + 8) as *const u32);
                x0 = *((edi + 0xbc) as *const f32);
                x0 = fadd(x0, rdf(&fr, 0x09));
                wrb(&mut fr, 0x96, rdb_fr(&fr, 0x96) | 4);
                fr[0x1c] = eax;
                eax = *((edi + 0x78) as *const u32);
                eax = *((relocated(T_12E8) + eax.wrapping_mul(4)) as *const u32);
                fr[0x1d] = eax;
                fr[0x1b] = fp(&mut fr, 0x12);
                eax = fr[0x0d];
                fr[0x25] = eax;
                wrf(&mut fr, 0x1e, x0);
                callee_thiscall!(
                    5,
                    u32,
                    edi,
                    relocated(0x00E8CE14),
                    fp(&mut fr, 0x1a),
                    0xFFFFFFFF,
                    0,
                    0
                );
            }
            // Float-threshold-gated final request.
            if esi == 0 || esi == 1 {
                callee_thiscall!(23, u32, edi);
                x0 = f32::from_bits(callee_ret_bits!());
                wrf(&mut fr, 0x04, x0);
                x0 = rdf(&fr, 0x04);
                if x0 > gf(F_8670) {
                    eax = callee_cdecl!(24, u32, 0x3F4CCCCD);
                    if (eax as u8) != 0 {
                        esi = *((edi + 0x78) as *const u32);
                        eax = callee_thiscall!(
                            6,
                            u32,
                            edi,
                            fr[0x10],
                            4,
                            esi
                        );
                        edi = eax;
                        callee_thiscall!(4, u32, fp(&mut fr, 0x14));
                        eax = fr[0x2f];
                        x0 = rdf(&fr, 0x04);
                        ecx = *((eax + 8) as *const u32);
                        wrb(&mut fr, 0x96, rdb_fr(&fr, 0x96) | 4);
                        fr[0x1c] = ecx;
                        ecx = *((relocated(T_12E8)
                            + esi.wrapping_mul(4))
                            as *const u32);
                        fr[0x1d] = ecx;
                        fr[0x1b] = fp(&mut fr, 0x12);
                        eax = callee_cdecl!(11, u32, x0.to_bits());
                        x0 = f32::from_bits(eax);
                        wrf(&mut fr, 0x16, x0);
                        eax = fr[0x0d];
                        fr[0x21] = eax;
                        eax = edi;
                        edi = fr[0x39];
                        callee_thiscall!(
                            8,
                            u32,
                            edi,
                            eax,
                            fp(&mut fr, 0x1a),
                            0xFFFFFFFF,
                            0,
                            0
                        );
                    }
                }
            }
        } // 'mid
        // Trailing three-global gate with logging call.
        if g32(G_1F7060) != 1 && g32(G_2088B4) == g32(G_F1C040) {
            esi = event;
            if g32(G_037720) != 0x12 {
                eax = *((edi + 0x120) as *const u32);
                callee_cdecl!(25, u32, eax, esi, flag);
            }
        } else {
            esi = event;
        }
        // Final event dispatch to the notification call.
        if esi == 0 || esi == 4 || esi == 0xe || esi == 0x10 {
            ecx = *((edi + 0x120) as *const u32);
            callee_thiscall!(26, u32, ecx, 1);
        } else if esi == 1 || esi == 5 || esi == 0xf || esi == 0x11 {
            ecx = *((edi + 0x120) as *const u32);
            callee_thiscall!(26, u32, ecx, 0);
        }
        0
    }
});
