// original: 0x00b61a20 audio_voice_dispatch (proposed)
/// Dispatch an audio voice update by info kind, then mix and return a handle.
///
/// Nine cdecl stack words (`arg0`..`arg8`; the entry registers are unread).
/// `arg0` is an entity, `arg1` the voice object, `arg2` the info key,
/// `arg3`/`arg8` f32 scales, `arg4` a flag byte, `arg5` an optional float
/// quad, `arg6`/`arg7` struct pointers. Returns a handle word, or 0.
///
/// Behaviour: look up the info record for the key (null returns 0). Unless
/// the voice's `+0x38` word is non-null and equals its `+0x7B4` word, run
/// the seed pair and scale the random value (signed int to float, times one
/// or two constants depending on an ordered compare of `arg3`), then
/// optionally run the setup callee. Scan the five chain heads at
/// `[voice+0x224]+0x44`, walk the first non-null one's `+8` chain, and call
/// its virtual slot `0xC`, merging the result into the flag by kind
/// (`0x83F`/`0x83E`/`0x83B`). By kind: 4 runs the port check and either
/// returns the close callee's answer or posts to `[esi+0x4C]`; 3 builds the
/// float quad and runs the mix chain including the 13-arg setup (whose
/// eighth stack word is always zero fill) and the struct query; 1 runs the
/// two probe callees and the scale gate; any other kind either runs the 7-arg
/// voice mix or runs the count loop
/// (at most two bodies) of random-scale, xor-mix, struct fills and two
/// 7-arg mixes, then the 9-arg mix. Every path joins a tail that either
/// returns the handle or runs the exit pair. Float compares are ordered;
/// the int-to-float scalings and the `0x3FFF` gate are signed.
///
/// Original: 0x00B61A20 (cdecl, nine stack words; u32 return).

lf_checker_rt::export!(cdecl, rw_00b61a20(
    arg0: u32, arg1: u32, arg2: u32, arg3: u32, arg4: u32,
    arg5: u32, arg6: u32, arg7: u32, arg8: u32
) -> u32 {
    unsafe {
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u16 {
            unsafe { (a as *const u16).read_unaligned() }
        }

        const INFO_KIND: u32 = 0x0C;
        const INFO_F20: u32 = 0x20;
        const INFO_A0: u32 = 0xA0;
        const INFO_K4: u32 = 0x04;
        const VTABLE_SLOT: u32 = 0x0C;
        const VOX_20: u32 = 0x20;
        const VOX_38: u32 = 0x38;
        const VOX_224: u32 = 0x224;
        const VOX_7B4: u32 = 0x7B4;
        const G_PORT: u32 = 0x167E2A0;
        const G_BOOL: u32 = 0x10521A0;
        const G_CNT: u32 = 0x1046A88;
        const G_CNT2: u32 = 0x1046A8C;
        const G_SQ: u32 = 0x1046A90;
        const G_CNT3: u32 = 0x1046A94;
        const G_F0: u32 = 0x110DB70;
        const G_F1: u32 = 0x110DB74;
        const G_F2: u32 = 0x110DB78;
        const G_TABLE: u32 = 0x1295CD8;
        const G_VOX2: u32 = 0x18B8968;
        const C_SCALE: u32 = 0xFE8684;
        const C_SCALE2: u32 = 0xFE892C;
        const C_A3: u32 = 0xFE8B48;
        const C_ABA: u32 = 0xFE88BC;
        const C_LSCALE: u32 = 0xFE8AEC;
        const C_LSUB: u32 = 0xFE8AA0;
        const C_XOR: u32 = 0xFE8FA0;
        const C_MUL: u32 = 0xFE87D0;
        const C_MUL2: u32 = 0xFE881C;
        const C_MIX: u32 = 0xFE8A24;
        const C_F32: u32 = 0xFE8AB8;
        const C_CMP: u32 = 0xFE88E8;
        const C_CMP2: u32 = 0xFE8830;
        const C_GATE: u32 = 0xFE8874;
        const SINGLETON: u32 = 0x12E2420;
        const ID_INFO: u32 = 1;
        const ID_SEED: u32 = 2;
        const ID_RND: u32 = 3;
        const ID_GATE: u32 = 4;
        const ID_SETUP: u32 = 5;
        const ID_VT: u32 = 6;
        const ID_CE: u32 = 7;
        const ID_BQ: u32 = 8;
        const ID_PORT: u32 = 9;
        const ID_MIXA: u32 = 10;
        const ID_CLOSE: u32 = 11;
        const ID_MIXB: u32 = 12;
        const ID_EXIT: u32 = 13;
        const ID_BIG: u32 = 14;
        const ID_MIXC: u32 = 15;
        const ID_PROBE: u32 = 16;
        const ID_PROBE2B: u32 = 31;
        const ID_PROBE2: u32 = 17;
        const ID_MIXD: u32 = 18;
        const ID_MIXE: u32 = 19;
        const ID_FR1: u32 = 20;
        const ID_FR2: u32 = 21;
        const ID_FILL1: u32 = 22;
        const ID_FILL2: u32 = 23;
        const ID_FILL3: u32 = 24;
        const ID_MIX7: u32 = 25;
        const ID_MIX6: u32 = 26;
        const ID_MIXF: u32 = 27;
        const ID_MIX9: u32 = 28;
        const ID_W1: u32 = 29;
        const ID_W2: u32 = 30;

        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        let g = |va: u32| (lf_checker_rt::global::<u32>(va) as *const u32).read_unaligned();
        let g8 =
            |va: u32| (lf_checker_rt::global::<u8>(va) as *const u8).read_unaligned();
        let vslot = |o: u32| unsafe {
            let f: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(rd32(rd32(o) + VTABLE_SLOT) as usize);
            f(o)
        };
        // Exit region (0xB61D62): shared tail call.
        let exit_region = |esi: u32| unsafe {
            if esi == 0 {
                return 0;
            }
            let t: u32 = lf_checker_rt::callee_thiscall!(ID_PORT, u32, g(G_PORT));
            if t == 0 {
                return 0;
            }
            lf_checker_rt::callee_thiscall!(ID_EXIT, u32, t, 0x3E8, 0x2710, esi, 0)
        };
        let exit_nocheck = |esi: u32| unsafe {
            let t: u32 = lf_checker_rt::callee_thiscall!(ID_PORT, u32, g(G_PORT));
            if t == 0 {
                return 0;
            }
            lf_checker_rt::callee_thiscall!(ID_EXIT, u32, t, 0x3E8, 0x2710, esi, 0)
        };
        let table_byte = |idx: i32| unsafe {
            let p = g(G_TABLE.wrapping_add((idx as u32).wrapping_mul(4)));
            rd8(p.wrapping_add(0xEF))
        };

        // ---- pre-loop ----
        let edi = arg1;
        let info: u32 = lf_checker_rt::callee_cdecl!(ID_INFO, u32, arg2);
        if info == 0 {
            return 0;
        }
        let e38 = rd32(edi + VOX_38);
        let mut flag: u8;
        if e38 != 0 && e38 == rd32(edi + VOX_7B4) {
            flag = 0;
        } else {
            lf_checker_rt::callee_thiscall!(ID_SEED, u32, edi, 1, 1);
            flag = 1;
            let n: u32 = lf_checker_rt::callee_cdecl!(ID_RND, u32,);
            let arg3 = f32::from_bits(arg3_bits);
            let mut scaled = mul((n as i32) as f32, f32::from_bits(g(C_SCALE)));
            if arg3 > f32::from_bits(g(C_A3)) {
                scaled = mul(scaled, f32::from_bits(g(C_SCALE2)));
            }
            let ga: u32 = lf_checker_rt::callee_cdecl!(ID_GATE, u32,);
            if (ga as u8 == 0) && (scaled > f32::from_bits(g(C_ABA))) {
                lf_checker_rt::callee_cdecl!(ID_SETUP, u32, edi, 1, 1, 0);
            }
        }
        if (arg4 as u8) != 0 {
            flag = 1;
        }
        let mut esi = 0u32;
        if flag == 0 {
            // five-head scan + chain walk
            let base = rd32(edi + VOX_224);
            let mut node = 0u32;
            let mut k = 0u32;
            while k < 5 {
                let w = rd32(base + 0x44 + k * 4);
                if w != 0 {
                    node = w;
                    break;
                }
                k += 1;
            }
            if node != 0 {
                let mut cur = node;
                loop {
                    esi = cur;
                    cur = rd32(cur + 8);
                    if cur == 0 {
                        break;
                    }
                }
            }
            let kind = rd32(info + INFO_KIND);
            if kind == 4 {
                if vslot(esi) != 0x83F {
                    flag = 1;
                }
            } else if kind == 3 {
                if vslot(esi) != 0x83E {
                    flag = 1;
                }
            } else if vslot(esi) != 0x83B || arg6 == 0 {
                if vslot(esi) != 0x83F {
                    flag = 1;
                }
            } else {
                // scaled triple into the voice block
                let a8 = f32::from_bits(arg8_bits);
                let s0 = mul(f32::from_bits(rd32(arg7)), a8);
                let s1 = mul(f32::from_bits(rd32(arg7 + 4)), a8);
                let s2 = mul(f32::from_bits(rd32(arg7 + 8)), a8);
                let triple = [s0.to_bits(), s1.to_bits(), s2.to_bits()];
                let w52 = rd16(arg6 + 0x52) as u32;
                lf_checker_rt::callee_thiscall!(
                    ID_CE, u32, esi, edi, arg0, arg2, w52, arg6.wrapping_add(0x10),
                    triple.as_ptr() as u32, arg6.wrapping_add(0x20)
                );
            }
        }
        // ---- head ----
        if flag == 0 {
            return 0;
        }
        let bref: u32 = lf_checker_rt::callee_thiscall!(ID_BQ, u32, edi.wrapping_add(0x2B0));
        esi = 0;
        let kind = rd32(info + INFO_KIND);
        if kind == 4 {
            let t: u32 = lf_checker_rt::callee_thiscall!(ID_PORT, u32, g(G_PORT));
            if (arg4 as u8) == 0 {
                if t == 0 {
                    return 0;
                }
                return lf_checker_rt::callee_thiscall!(ID_CLOSE, u32, t, arg2);
            }
            esi = if t == 0 {
                0
            } else {
                lf_checker_rt::callee_thiscall!(
                    ID_MIXA, u32, t, 0x3E8, 0x7530, 0x3E99999A, esi, esi
                )
            };
            if g8(G_BOOL) != 0 {
                let idx = rd16(edi + 0x2E) as u16 as i16 as i32;
                if (table_byte(idx) as u32) < g(G_CNT) {
                    (esi.wrapping_add(0x4C) as *mut u32).write_unaligned(1);
                }
            }
        } else if kind == 3 {
            // float quad from the voice or the override
            let ev = rd32(edi + VOX_20);
            let (r30, r34, r38, r3c);
            if arg5 == 0 {
                r30 = rd32(ev + 0x30);
                r34 = rd32(ev + 0x34);
                r38 = rd32(ev + 0x38);
                r3c = 0u32;
            } else {
                r30 = rd32(arg5);
                r34 = rd32(arg5 + 4);
                r38 = rd32(arg5 + 8);
                r3c = rd32(arg5 + 0xC);
            }
            let mut quad = [r30, r34, r38, r3c];
            // arg4==0 adds the bias into the quad's third word
            if (arg4 as u8) == 0 {
                quad[2] = add(f32::from_bits(quad[2]), f32::from_bits(g(0xFE88E8))).to_bits();
                let t: u32 = lf_checker_rt::callee_thiscall!(ID_PORT, u32, g(G_PORT));
                if t != 0 {
                    esi = lf_checker_rt::callee_thiscall!(
                        ID_MIXB, u32, t, 0x3E8, 0x2710, quad.as_ptr() as u32, 0, 0
                    );
                }
                if g8(G_BOOL) == 0 {
                    (esi.wrapping_add(0x28) as *mut u32).write_unaligned(0);
                } else {
                    let idx = rd16(edi + 0x2E) as u16 as i16 as i32;
                    let w = if (table_byte(idx) as u32) < g(G_CNT2) { 3 } else { 0 };
                    (esi.wrapping_add(0x28) as *mut u32).write_unaligned(w);
                }
                return exit_region(esi);
            }
            // arg4!=0: vector block. xmm copies of the quad words.
            let (x1, x3, x2) = if arg5 == 0 {
                (
                    f32::from_bits(rd32(ev + 0x30)),
                    f32::from_bits(rd32(ev + 0x34)),
                    f32::from_bits(rd32(ev + 0x38)),
                )
            } else {
                (
                    f32::from_bits(r30),
                    f32::from_bits(r34),
                    f32::from_bits(r38),
                )
            };
            if rd32(info + INFO_A0) != 3 {
                // skip to the struct query tail
            } else {
                let d0 = sub(f32::from_bits(rd32(ev + 0x30)), x1);
                let d1 = sub(f32::from_bits(rd32(ev + 0x34)), x3);
                let d2 = sub(f32::from_bits(rd32(ev + 0x38)), x2);
                let sq = add(add(mul(d1, d1), mul(d0, d0)), mul(d2, d2));
                if f32::from_bits(g(G_SQ)) > sq {
                    let n: u32 = lf_checker_rt::callee_cdecl!(ID_RND, u32,);
                    let small = (n as i32) < 0x3FFF;
                    if small {
                        let edi2 = g(G_VOX2);
                        let e2 = rd32(edi2);
                        let w1 = {
                            let f: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(
                                rd32(e2 + 0x10) as usize,
                            );
                            f(edi2)
                        };
                        let w2 = {
                            let f: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(
                                rd32(e2 + 0x20) as usize,
                            );
                            f(edi2)
                        };
                        let blk = [
                            bref,
                            info,
                            esi,
                            esi,
                            1.0f32.to_bits(),
                            0u32,
                            r30,
                            r34,
                            r38,
                            r3c,
                        ];
                        lf_checker_rt::callee_thiscall!(
                            ID_BIG, u32, lf_checker_rt::relocated(SINGLETON),
                            arg1, 0, blk.as_ptr() as u32, w2, w1,
                            lf_checker_rt::relocated(0xEB0F4C), arg0, 0, 0, 5,
                            0, 0, 0
                        );
                    }
                }
            }
            let t: u32 = lf_checker_rt::callee_thiscall!(ID_PORT, u32, g(G_PORT));
            if t == 0 {
                esi = 0;
            } else {
                esi = lf_checker_rt::callee_thiscall!(
                    ID_MIXC, u32, t, 0x3E8, 0x7530, quad.as_ptr() as u32
                );
            }
            // tail through 0xB6244C shared below
        } else {
            // kind==1 with arg4==0 runs the probe gate; every other kind
            // (1 with arg4!=0, or not 1/3/4) shares the fc1 region below.
            if kind == 1 && (arg4 as u8) == 0 {
                let t1 = rd32(edi + VOX_224).wrapping_add(0x2E0);
                let al1: u32 = lf_checker_rt::callee_thiscall!(ID_PROBE, u32, t1, 0x76C, 0);
                let mut at_scale = al1 as u8 == 0;
                if !at_scale {
                    let al2: u32 =
                        lf_checker_rt::callee_thiscall!(ID_PROBE2B, u32, t1, 0x770, 0);
                    if al2 as u8 != 0 {
                        at_scale = true;
                    } else if bref == 0 {
                        at_scale = true;
                    } else {
                        let al3: u32 = lf_checker_rt::callee_thiscall!(ID_PROBE2, u32, bref);
                        if al3 as u8 != 0 {
                            // fall into the fc1 region below
                        } else {
                            at_scale = true;
                        }
                    }
                }
                if at_scale {
                    let n: u32 = lf_checker_rt::callee_cdecl!(ID_RND, u32,);
                    let scaled = mul((n as i32) as f32, f32::from_bits(g(C_SCALE)));
                    if scaled > f32::from_bits(g(C_GATE)) {
                        let t: u32 = lf_checker_rt::callee_thiscall!(ID_PORT, u32, g(G_PORT));
                        if t == 0 {
                            return 0;
                        }
                        esi = lf_checker_rt::callee_thiscall!(
                            ID_MIXD, u32, t, 0x3E8, 0x2710, arg0, 0, 0, esi, 0, 0
                        );
                        return exit_region(esi);
                    }
                    let t: u32 = lf_checker_rt::callee_thiscall!(ID_PORT, u32, g(G_PORT));
                    if t == 0 {
                        esi = 0;
                    } else {
                        let c = rd32(arg0 + 0x20);
                        let ec = if c == 0 {
                            arg0.wrapping_add(0x10)
                        } else {
                            c.wrapping_add(0x30)
                        };
                        esi = lf_checker_rt::callee_thiscall!(
                            ID_MIXE, u32, t, 0x3E8, 0x2710, ec, arg0, 0
                        );
                    }
                    if g8(G_BOOL) == 0 {
                        (esi.wrapping_add(0x28) as *mut u32).write_unaligned(1);
                    } else {
                        let idx = rd16(edi + 0x2E) as u16 as i16 as i32;
                        let w = if (table_byte(idx) as u32) < g(G_CNT3) { 4 } else { 1 };
                        (esi.wrapping_add(0x28) as *mut u32).write_unaligned(w);
                    }
                    return exit_region(esi);
                }
            }
            // ---- fc1 region ----
            if arg6 == esi {
                // tail
            } else {
                let dl = if arg0 != 0
                    && rd32(arg0 + 0x28) & 0x3C0 == 0xC0
                    && rd8(arg0 + 0x219) != 0
                {
                    1u8
                } else {
                    0u8
                };
                let run_loop = (arg4 as u8) == 0
                    && dl != 0
                    && rd8(edi + 0x219) == 0
                    && rd32(edi + 0x2A0) & 4 == 0
                    && rd32(edi + 0x268) & 0x10 == 0;
                if run_loop {
                    let gf0 = f32::from_bits(g(G_F0));
                    let gf1 = f32::from_bits(g(G_F1));
                    let gf2 = f32::from_bits(g(G_F2));
                    let ev = rd32(edi + VOX_20);
                    let mut counter = 0u32;
                    // fill structs persist across iterations (one frame)
                    let mut s80 = [0u32; 20];
                    let mut s140 = [0u32; 4];
                    let mut se0 = [0u32; 4];
                    while counter < 2 {
                        counter += 1;
                        // random scale
                        let n: u32 = lf_checker_rt::callee_cdecl!(ID_RND, u32,);
                        let _lsc = sub(
                            mul(
                                mul((n as i32) as f32, f32::from_bits(g(C_SCALE))),
                                f32::from_bits(g(C_LSCALE)),
                            ),
                            f32::from_bits(g(C_LSUB)),
                        );
                        // xor mix + struct pair
                        // f32xmm0 stubs put the answer bits in EAX too; a Rust
                        // f32 return would read x87 st0 instead.
                        let x94: u32 = lf_checker_rt::callee_cdecl!(ID_FR1, u32,);
                        let m40 = x94 ^ g(C_XOR);
                        let x93: u32 = lf_checker_rt::callee_cdecl!(ID_FR2, u32,);
                        let (m44, m48) = (x93, 0u32);
                        let c = f32::from_bits(g(C_MUL));
                        let t0 = mul(gf1, c);
                        let t2 = mul(gf2, c);
                        let t3 = mul(gf0, c);
                        let e30 = f32::from_bits(rd32(ev + 0x30));
                        let e34 = f32::from_bits(rd32(ev + 0x34));
                        let e38 = f32::from_bits(rd32(ev + 0x38));
                        let m54 = add(e34, t0);
                        let m50 = add(t3, e30);
                        let m58 = add(e38, t2);
                        let c2 = f32::from_bits(g(C_MUL2));
                        let u5 = mul(gf1, c2);
                        let u4 = mul(gf2, c2);
                        let u6 = mul(gf0, c2);
                        let m24 = sub(e34, u5);
                        let m28 = sub(e38, u4);
                        let m20 = sub(e30, u6);
                        lf_checker_rt::callee_thiscall!(
                            ID_FILL1, u32, s80.as_mut_ptr() as u32
                        );
                        lf_checker_rt::callee_thiscall!(
                            ID_FILL2, u32, s140.as_mut_ptr() as u32
                        );
                        lf_checker_rt::callee_thiscall!(
                            ID_FILL3, u32, se0.as_mut_ptr() as u32
                        );
                        // second-phase triple (R+0x30): mixed BEFORE A536B0#1
                        // and passed as its second pointer; R+0x70 recomputes
                        // the identical triple for the 6-arg mix below.
                        let cx = f32::from_bits(g(C_MIX));
                        let f30 = add(mul(f32::from_bits(m40), cx), m50);
                        let f34 = add(mul(f32::from_bits(m44), cx), m54);
                        let f38 = add(mul(f32::from_bits(m48), cx), m58);
                        let b50 = [m50.to_bits(), m54.to_bits(), m58.to_bits(), 0u32];
                        let b30 = [f30.to_bits(), f34.to_bits(), f38.to_bits(), 0u32];
                        let a1: u32 = lf_checker_rt::callee_cdecl!(
                            ID_MIX7, u32, b50.as_ptr() as u32, b30.as_ptr() as u32, 0,
                            s140.as_ptr() as u32, 6, 1, 4
                        );
                        if a1 != 0 {
                            // the answer only skips the rest of this iteration
                            continue;
                        }
                        // 6-arg mix over the R+0x70 triple (same values)
                        let m1arg = sub(m58, f32::from_bits(g(C_F32)));
                        let _: u32 = lf_checker_rt::callee_cdecl!(
                            ID_MIX6, u32, b30.as_ptr() as u32, m1arg.to_bits(), 0,
                            s80.as_ptr() as u32, 6, 4
                        );
                        // third-phase floats + second 7-arg mix
                        let g60 = add(mul(f32::from_bits(m40), cx), m20);
                        let g64 = add(mul(f32::from_bits(m44), cx), m24);
                        let g68 = add(mul(f32::from_bits(m48), cx), m28);
                        let b60 = [g60.to_bits(), g64.to_bits(), g68.to_bits(), 0u32];
                        let b20 = [m20.to_bits(), m24.to_bits(), m28.to_bits(), 0u32];
                        let _: u32 = lf_checker_rt::callee_cdecl!(
                            ID_MIX7, u32, b20.as_ptr() as u32, b60.as_ptr() as u32,
                            0, se0.as_ptr() as u32, 6, 1, 4
                        );
                        // float ladder into esi. All three compares are ordered
                        // (`ja`/falling-off-`jbe`), so NaN takes the low branch.
                        let x2 = f32::from_bits(s80[6]);
                        let ecxv = s80[0];
                        let e38b = f32::from_bits(rd32(ev + 0x38));
                        let c3 = f32::from_bits(g(C_CMP));
                        let ladder_tail = |esiv: u32| -> u32 {
                            let x1 = sub(e38b, c3);
                            let x0b = sub(x1, f32::from_bits(g(C_MIX)));
                            if x0b > x2 {
                                return 6;
                            }
                            if (s80[18] >> 24) & 1 != 0 {
                                let x1b = sub(x1, c3);
                                if x1b > x2 {
                                    return 6;
                                }
                            }
                            esiv
                        };
                        if se0[0] == 0 {
                            if ecxv == 0 {
                                esi = 4;
                            } else {
                                esi = ladder_tail(esi);
                            }
                        } else if ecxv == 0 {
                            esi = 5;
                        } else {
                            let x0 =
                                sub(sub(e38b, c3), f32::from_bits(g(C_CMP2)));
                            if x0 > x2 {
                                esi = 5;
                            } else {
                                esi = ladder_tail(esi);
                            }
                        }
                        let f32v = sub(f32::from_bits(b50[2]), f32::from_bits(g(C_F32)));
                        let f32v = if esi == 6 || ecxv == 0 { f32v } else { x2 };
                        let t: u32 =
                            lf_checker_rt::callee_thiscall!(ID_PORT, u32, g(G_PORT));
                        if t == 0 {
                            esi = 0;
                            continue;
                        }
                        let mut b40 = [m40, m44, m48, 0u32];
                        esi = lf_checker_rt::callee_thiscall!(
                            ID_MIXF, u32, t, 0x7D0, 0x2710, esi, b40.as_mut_ptr() as u32,
                            f32v.to_bits(), arg0, 0
                        );
                        if esi != 0 {
                            return exit_nocheck(esi);
                        }
                    }
                    if esi != 0 {
                        // 0xB623C9: dead in practice (exhausted implies esi==0)
                        return exit_nocheck(esi);
                    }
                    // counter exhausted falls into the 9-arg mix below
                }
                // ---- 9-arg mix ----
                let t: u32 = lf_checker_rt::callee_thiscall!(ID_PORT, u32, g(G_PORT));
                if t == 0 {
                    esi = 0;
                } else {
                    let a8 = f32::from_bits(arg8_bits);
                    let s0 = mul(f32::from_bits(rd32(arg7)), a8);
                    let s1 = mul(f32::from_bits(rd32(arg7 + 4)), a8);
                    let s2 = mul(f32::from_bits(rd32(arg7 + 8)), a8);
                    let triple = [s0.to_bits(), s1.to_bits(), s2.to_bits()];
                    let w52 = rd16(arg6 + 0x52) as u32;
                    esi = lf_checker_rt::callee_thiscall!(
                        ID_MIX9, u32, t, edi, 0x3E8, 0x2710, arg0, arg2, w52,
                        arg6.wrapping_add(0x10), triple.as_ptr() as u32,
                        arg6.wrapping_add(0x20)
                    );
                }
            }
        }
        // ---- tail (0xB6244C) ----
        if (arg4 as u8) == 0 {
            return exit_region(esi);
        }
        esi
    }
});
