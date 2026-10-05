// original: 0x00b60430 audio_voice_setup (proposed)

/// Build voice-mix structures on the frame and run the mix chain.
///
/// `this` is the voice controller (`+0x18` is the key passed to the info
/// callee). `arg0` is an entity (null selects the random branch), `arg1` a
/// struct pointer (its `+0x30` field address is passed on), `arg2` an opaque
/// word, `arg3` a pointer to three f32s, `arg4` a flag word (low byte) also
/// passed to the second mix callee, `arg5` an opaque word, `arg6` an f32
/// ceiling. Returns whatever the trailing cookie-check callee returns.
///
/// Behaviour: copy the `arg3` triple to the frame and run the resolver
/// callee on (`arg0`, `arg2`, triple). When `arg0` is live (non-null with
/// (`+0x28` & `0x3C0`) == `0xC0`) stash it; if its byte `+0x219` is also
/// non-zero run the probe callee on two zeroed frame buffers and seed `edi`
/// from its low byte (`0x3101` when zero, `0x1101` otherwise), else run the
/// random callee, scale it (signed int to float, times a constant) and seed
/// `edi` (`0x1101` when the second constant is ordered-above the scaled
/// value, else `0x101`). Build one 24-word voice struct on the frame from
/// three globals (words the original never stores read back as zero under
/// the contract's stack fill; the original's further 39 structs are never
/// read or passed, so they are not built). When the info record's `+0x4`
/// equals 3, walk its `+0xFC` chain (signed `<= 1` bounds; the running
/// value starts at 1, becomes `0x10` once the fourth query runs, then that
/// chain's value, or 8 when the virtual slot `0x128` probe answers zero)
/// and run the float callee for the mix argument (0.0 on the skipped paths).
/// When `arg4`'s low byte is zero run the 13-arg setup callee into `edi`;
/// then, unless the stashed entity is null or its bytes `+0x218`/`+0x219`
/// refuse, copy the triple to the output block (overwriting it with the
/// struct's second quad word when `edi` is signed-positive) and run the
/// first mix callee. Run the info/float16 pair, keeping `arg6` when it is
/// ordered at or above its constant and taking the converted value
/// otherwise; run the two mix callees (the second may post a scripted word
/// to the struct head). When the info record's `+0x4` is 2, 3, 4 or 5
/// (checked in order, one call per check) rebuild the output block (with
/// the struct quad when the head is non-zero) and run the third mix callee.
/// When `arg0` is non-null and its `+0x6C` link is null or its byte `+0xE`
/// is zero, run the final mix callee. Return the cookie-check result.
///
/// Signed comparisons (all scripted with negatives and `0x80000000`, and
/// each proven by a wrong version that reads them unsigned): the two
/// `+0xFC <= 1` bounds (signed `jle`), the `edi > 0` gate (signed `jle`
/// inverted), the int-to-float scaling (signed `cvtdq2ps`), the float16
/// conversion (signed `cwde` of the low 16 bits). All other integer
/// comparisons are equality or bit tests. Float `>=`/`>` are ordered
/// (unordered takes the other side, as the original's `comiss`+`jae` does).
///
/// Original: 0x00B60430 (thiscall, seven stack words; u32 return).
lf_checker_rt::export!(thiscall, rw_00b60430(
    this: u32, arg0: u32, arg1: u32, arg2: u32, arg3: u32, arg4: u32, arg5: u32, arg6: u32
) -> u32 {
    unsafe {
        const KIND_MASK: u32 = 0x3C0;
        const KIND_LIVE: u32 = 0xC0;
        const THIS_KEY: u32 = 0x18;
        const ENT_FLAGS: u32 = 0x28;
        const ENT_LINK6C: u32 = 0x6C;
        const ENT_PROBE: u32 = 0x224;
        const ENT_GATE_B: u32 = 0x218;
        const ENT_GATE_C: u32 = 0x219;
        const INFO_KIND: u32 = 0x04;
        const INFO_FC: u32 = 0xFC;
        const VTABLE_PROBE: u32 = 0x128;
        const ARG1_FIELD: u32 = 0x30;
        const G_F0: u32 = 0x1B4B320;
        const G_F1: u32 = 0x1B4B324;
        const G_F2: u32 = 0x1B4B328;
        const C_SCALE: u32 = 0xFE8684;
        const C_LIMIT: u32 = 0xFE880C;
        const C_ARG6: u32 = 0xFE8628;
        const ID_RESOLVE: u32 = 1;
        const ID_PROBE: u32 = 2;
        const ID_RANDOM: u32 = 3;
        const ID_INFO: u32 = 4;
        const ID_INFO2: u32 = 15;
        const ID_VTABLE: u32 = 5;
        const ID_FLOAT: u32 = 6;
        const ID_SETUP: u32 = 7;
        const ID_MIX1: u32 = 8;
        const ID_F16: u32 = 9;
        const ID_MIX2: u32 = 10;
        const ID_MIX3: u32 = 11;
        const ID_MIX4: u32 = 12;
        const ID_MIX5: u32 = 13;
        const ID_COOKIE: u32 = 14;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        let g = |va: u32| (lf_checker_rt::global::<u32>(va) as *const u32).read_unaligned();

        // Frame: input triple, output block, voice struct head.
        let triple = [rd32(arg3), rd32(arg3 + 4), rd32(arg3 + 8)];
        let mut out = [0u32; 4];
        let (gf0, gf1, gf2) = (g(G_F0), g(G_F1), g(G_F2));
        let mut head = [
            0, 0, 0, 0, gf0, gf1, gf2, 0, gf0, gf1, gf2, 0, gf0, gf1, gf2, 0, 0, 0, 0, 0xFFFF,
            0, 0, 0, 0u32,
        ];
        let buf = [0u8; 5];
        let arg0_home = arg0;

        lf_checker_rt::callee_thiscall!(ID_RESOLVE, u32, this, arg0, arg2, triple.as_ptr() as u32);

        // Entity stash + seed.
        let live = arg0 != 0 && rd32(arg0 + ENT_FLAGS) & KIND_MASK == KIND_LIVE;
        let stashed = if live { arg0 } else { 0 };
        let mut edi: u32;
        if live && rd8(arg0 + ENT_GATE_C) != 0 {
            let al: u32 = lf_checker_rt::callee_thiscall!(
                ID_PROBE, u32, rd32(arg0 + ENT_PROBE),
                buf.as_ptr() as u32, buf.as_ptr().add(1) as u32
            );
            edi = if al as u8 == 0 { 0x2101 } else { 0x101 };
            edi |= 0x1000;
        } else {
            let n: u32 = lf_checker_rt::callee_cdecl!(ID_RANDOM, u32,);
            let scaled = mul((n as i32) as f32, f32::from_bits(g(C_SCALE)));
            edi = 0x101;
            if f32::from_bits(g(C_LIMIT)) > scaled {
                edi = 0x1101;
            }
        }

        // Info chain + float argument. frc is the setup callee's 8th
        // argument: 1, then 0x10 once the fourth query runs, then that
        // query chain's value, or 8 when the virtual probe answers zero.
        let key = rd32(this + THIS_KEY);
        let mut fr14 = 0.0f32;
        let mut frc = 1u32;
        let info1: u32 = lf_checker_rt::callee_cdecl!(ID_INFO, u32, key);
        if rd32(info1 + INFO_KIND) == 3 {
            let info2: u32 = lf_checker_rt::callee_cdecl!(ID_INFO, u32, key);
            let mut at_vtable = false;
            if rd32(info2 + INFO_FC) == 0 {
                let info4: u32 = lf_checker_rt::callee_cdecl!(ID_INFO2, u32, key);
                frc = 0x10;
                if (rd32(info4 + INFO_FC) as i32) <= 1 {
                    at_vtable = true;
                } else {
                    let info5: u32 = lf_checker_rt::callee_cdecl!(ID_INFO2, u32, key);
                    frc = rd32(info5 + INFO_FC);
                    at_vtable = true;
                }
            } else {
                let info3: u32 = lf_checker_rt::callee_cdecl!(ID_INFO, u32, key);
                if (rd32(info3 + INFO_FC) as i32) > 1 {
                    let info4: u32 = lf_checker_rt::callee_cdecl!(ID_INFO2, u32, key);
                    frc = 0x10;
                    if (rd32(info4 + INFO_FC) as i32) <= 1 {
                        at_vtable = true;
                    } else {
                        let info5: u32 = lf_checker_rt::callee_cdecl!(ID_INFO2, u32, key);
                        frc = rd32(info5 + INFO_FC);
                        at_vtable = true;
                    }
                }
            }
            if at_vtable {
                if stashed != 0 {
                    let f: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(
                        rd32(rd32(stashed) + VTABLE_PROBE) as usize,
                    );
                    let al: u32 = f(stashed);
                    if al as u8 == 0 {
                        frc = 8;
                    }
                }
                let info6: u32 = lf_checker_rt::callee_cdecl!(ID_INFO, u32, key);
                fr14 = lf_checker_rt::callee_thiscall!(ID_FLOAT, f32, info6, arg0);
            }
        }

        // Setup block.
        if arg4 as u8 == 0 {
            edi = lf_checker_rt::callee_cdecl!(
                ID_SETUP, u32, arg2, triple.as_ptr() as u32, arg1.wrapping_add(ARG1_FIELD),
                &arg0_home as *const u32 as u32, 1, head.as_ptr() as u32, 0x28, frc,
                fr14.to_bits(), 0x1CE, edi, 8, 0
            );
            if stashed != 0
                && rd8(arg0 + ENT_GATE_B) == 0
                && rd8(arg0 + ENT_GATE_C) != 0
            {
                out[0] = triple[0];
                out[1] = triple[1];
                out[2] = triple[2];
                if (edi as i32) > 0 {
                    out[0] = head[4];
                    out[1] = head[5];
                    out[2] = head[6];
                    out[3] = head[7];
                }
                lf_checker_rt::callee_cdecl!(ID_MIX1, u32, arg2, out.as_ptr() as u32);
            }
        } else {
            edi = 0;
        }

        // Info/float16 pair + ceiling mix.
        let info7: u32 = lf_checker_rt::callee_cdecl!(ID_INFO, u32, key);
        let ax: u32 = lf_checker_rt::callee_thiscall!(ID_F16, u32, info7, arg0);
        let derived = ((ax & 0xFFFF) as u16 as i16) as f32;
        let arg6 = f32::from_bits(arg6_bits);
        let mixed = if arg6 >= f32::from_bits(g(C_ARG6)) { arg6 } else { derived };

        let _: f32 = lf_checker_rt::callee_thiscall!(
            ID_MIX2, f32, this, arg0, arg1, head.as_ptr() as u32, edi,
            mixed.to_bits(), arg5
        );
        lf_checker_rt::callee_thiscall!(
            ID_MIX3, u32, this, arg0, arg1, head.as_mut_ptr() as u32,
            triple.as_ptr() as u32, arg4
        );

        // Kind chain + final mix.
        let mut run_mix4 = false;
        let i8: u32 = lf_checker_rt::callee_cdecl!(ID_INFO, u32, key);
        if rd32(i8 + INFO_KIND) == 2 {
            run_mix4 = true;
        } else {
            let i9: u32 = lf_checker_rt::callee_cdecl!(ID_INFO, u32, key);
            if rd32(i9 + INFO_KIND) == 3 {
                run_mix4 = true;
            } else {
                let i10: u32 = lf_checker_rt::callee_cdecl!(ID_INFO, u32, key);
                if rd32(i10 + INFO_KIND) == 4 {
                    run_mix4 = true;
                } else {
                    let i11: u32 = lf_checker_rt::callee_cdecl!(ID_INFO, u32, key);
                    if rd32(i11 + INFO_KIND) == 5 {
                        run_mix4 = true;
                    }
                }
            }
        }
        if run_mix4 {
            out[0] = triple[0];
            out[1] = triple[1];
            out[2] = triple[2];
            if head[0] != 0 {
                out[0] = head[4];
                out[1] = head[5];
                out[2] = head[6];
                out[3] = head[7];
            }
            lf_checker_rt::callee_cdecl!(
                ID_MIX4, u32, arg0, arg1.wrapping_add(ARG1_FIELD), out.as_ptr() as u32
            );
        }

        if arg0 != 0 {
            let q = rd32(arg0 + ENT_LINK6C);
            if q == 0 || rd8(q + 0x0E) == 0 {
                lf_checker_rt::callee_thiscall!(
                    ID_MIX5, u32, this, arg0, arg2, arg3, head.as_ptr() as u32, edi, 0, 0
                );
            }
        }

        lf_checker_rt::callee_thiscall!(ID_COOKIE, u32, 0)
    }
});
