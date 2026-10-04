// original: 0x00cc0620 CPedMoveBlendOnFoot::vf1 (staged; currently stage 3)

/// First virtual-slot update of the on-foot move blend object (STAGE 3).
///
/// `this` (ECX) points to the blend object; its word at `+0x24` points to a
/// state object. Stage 1 covers the entry block through the early-out path
/// only, i.e. inputs where the state word at `+0x26c` has bit 2 set; any
/// other input fails loudly (see below) so a stage-1 contract can never
/// pass on unimplemented paths.
///
/// Entry block: `flag_obj` is loaded from `[state+0x6c]`. When it is null
/// or its byte at `+0x0e` is zero, the flag update is skipped. Otherwise,
/// when the state byte at `+0x219` is zero, bit 0x2000 of `[this+0x50]` is
/// cleared. When it is nonzero, a helper (thiscall, one stack word,
/// byte result) is asked up to three times in order (arguments 0xd3,
/// 0xfe, 0x11f) with `flag_obj+0x808` as its object; the first nonzero
/// answer clears bit 0x2000, while three zero answers set it. Bits
/// 0x200000/0x100000 of `[this+0x50]` are then always cleared and a global
/// frame flag is reset to 0.
///
/// Early-out path (state bit 2 set): a second helper (thiscall, argument
/// 1) runs against the state object, then `[this+0x48]` is set to -1,
/// `[this+0x30]`, `[this+0x5c]`, `[this+0x4c]` and `[this+0x20]` to 0,
/// `[this+0x1c]` to 3.0f bits, and control tail-jumps to the shared
/// finish routine with `this` still in ECX (forwarded through the checker
/// as a call whose answer is the return value).
///
/// The continue path (`+0x26c` bit 2 clear) is NOT implemented in stage 1:
/// reaching it aborts, which the checker reports as a loud rewrite-side
/// fault rather than a silent pass.
///
/// Stage 2 adds the flag shortcut: when the continue path is taken but bit
/// 0x100 of the state word at `+0x29c` is set, the function returns the
/// state pointer at once with no further stores or calls.
///
/// Stage 3 adds the main cascade down to the exit at the `0xcc00b0` probe:
/// the scale-factor store, the gated zeroing, the virtual slot call, the
/// two-helper query pair, the node-list walk, the mode selection, the
/// object resolutions with their fallbacks, the conditional parameter
/// call, and the limit check. Reaching either frontier (the walk's
/// `0x41e` exit or the probe's fallthrough) aborts loudly; the stage-3
/// contract never feeds such inputs.
///
/// Original: 0x00cc0620 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_00cc0620(this: u32) -> u32 {
    unsafe {
        const STATE_OFF: u32 = 0x24;
        const FLAGS_OFF: u32 = 0x50;
        const FLAG_OBJ_OFF: u32 = 0x6c;
        const FLAG_BYTE_OFF: u32 = 0x0e;
        const GATE_BYTE_OFF: u32 = 0x219;
        const DISPATCH_OFF: u32 = 0x26c;
        const HELPER_OBJ_BIAS: u32 = 0x808;
        const SET_BIT: u32 = 0x2000;
        const CLEAR_BITS: u32 = 0x300000;
        const ASK1: u32 = 0xd3;
        const ASK2: u32 = 0xfe;
        const ASK3: u32 = 0x11f;
        const FRAME_FLAG: u32 = 0x171bf9c;
        const ASK_CALLEE_1: u32 = 1;
        const ASK_CALLEE_2: u32 = 2;
        const ASK_CALLEE_3: u32 = 3;
        const EARLY_CALLEE: u32 = 4;
        const TAIL_CALLEE: u32 = 5;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        unsafe fn wrf(a: u32, v: f32) {
            unsafe { wr32(a, v.to_bits()) }
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        unsafe fn wr16(a: u32, v: u16) {
            unsafe { (a as *mut u16).write_unaligned(v) }
        }

        let state = rd32(this.wrapping_add(STATE_OFF));
        let flag_obj = rd32(state.wrapping_add(FLAG_OBJ_OFF));
        if flag_obj != 0 && rd8(flag_obj.wrapping_add(FLAG_BYTE_OFF)) != 0 {
            if rd8(state.wrapping_add(GATE_BYTE_OFF)) == 0 {
                wr32(
                    this.wrapping_add(FLAGS_OFF),
                    rd32(this.wrapping_add(FLAGS_OFF)) & !SET_BIT,
                );
            } else {
                let helper = flag_obj.wrapping_add(HELPER_OBJ_BIAS);
                let a1: u32 = lf_checker_rt::callee_thiscall!(ASK_CALLEE_1, u32, helper, ASK1);
                let mut all_zero = (a1 as u8) == 0;
                if all_zero {
                    let a2: u32 =
                        lf_checker_rt::callee_thiscall!(ASK_CALLEE_2, u32, helper, ASK2);
                    all_zero = (a2 as u8) == 0;
                    if all_zero {
                        let a3: u32 =
                            lf_checker_rt::callee_thiscall!(ASK_CALLEE_3, u32, helper, ASK3);
                        all_zero = (a3 as u8) == 0;
                    }
                }
                let flags = rd32(this.wrapping_add(FLAGS_OFF));
                if all_zero {
                    wr32(this.wrapping_add(FLAGS_OFF), flags | SET_BIT);
                } else {
                    wr32(this.wrapping_add(FLAGS_OFF), flags & !SET_BIT);
                }
            }
        }
        wr32(
            this.wrapping_add(FLAGS_OFF),
            rd32(this.wrapping_add(FLAGS_OFF)) & !CLEAR_BITS,
        );
        wr32(lf_checker_rt::relocated(FRAME_FLAG), 0);
        let state2 = rd32(this.wrapping_add(STATE_OFF));
        if rd32(state2.wrapping_add(DISPATCH_OFF)) & 4 == 0 {
            // Stage 2: flag shortcut returns the state pointer.
            if rd32(state2.wrapping_add(0x29c)) & 0x100 != 0 {
                return state2;
            }
            // Stage 3: main cascade down to the cc0a16 exit.
            let flagword = rd32(state2.wrapping_add(0x29c));
            if flagword & 0x200 != 0 || rd32(state2.wrapping_add(0x268)) & 0x1000 != 0 {
                wr32(this.wrapping_add(0x0c), 0);
                wr32(this.wrapping_add(0x10), 0);
            }
            let scale = rdf(state2.wrapping_add(0x28c));
            let factor = rdf(lf_checker_rt::relocated(0x105141c));
            wrf(lf_checker_rt::relocated(0x171bf8c), mul(scale, factor));
            let qobj = rd32(state2.wrapping_add(0x224)).wrapping_add(0x44);
            let qans: u32 = lf_checker_rt::callee_thiscall!(6, u32, qobj, 0x410);
            let bl_flag = qans != 0 && rd8(qans.wrapping_add(0x72)) == 3;
            let g1: u32 = lf_checker_rt::callee_thiscall!(7, u32, state2);
            if (g1 as u8) != 0 && bl_flag {
                wr32(this.wrapping_add(0x0c), 0);
                wr32(this.wrapping_add(0x10), 0);
            }
            let mut flags = rd32(this.wrapping_add(FLAGS_OFF));
            if flags & 0x4000 != 0 {
                wr32(this.wrapping_add(0x0c), rd32(this.wrapping_add(0x14)));
                wr32(this.wrapping_add(0x10), rd32(this.wrapping_add(0x18)));
            }
            flags &= !0x4000;
            wr32(this.wrapping_add(FLAGS_OFF), flags);
            // Virtual slot +0x4c on `this` (planted stub, both sides alike).
            let slot = rd32(rd32(this).wrapping_add(0x4c));
            let vcall: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(slot as usize);
            let _ = vcall(this);
            wr8(lf_checker_rt::relocated(0x171bf89), 1);
            wr8(lf_checker_rt::relocated(0x1051526), 1);
            let st3 = rd32(this.wrapping_add(STATE_OFF));
            let saved_ecx = rd32(st3.wrapping_add(0x78));
            let mut ebx = rd32(rd32(st3.wrapping_add(0xa80)).wrapping_add(0x3c));
            let c6: u32 = lf_checker_rt::callee_cdecl!(9, u32, ebx);
            let g104 = rd32(lf_checker_rt::relocated(0x10496e8));
            let c8: u32 = lf_checker_rt::callee_cdecl!(10, u32, c6, g104, ebx);
            if (c8 as u8) == 0 {
                ebx = 0x31;
            }
            // Scratch byte the walk-exit detour sets; 0 on stages 1-4 paths.
            let flag_0e: u8 = 0;
            let st4 = rd32(this.wrapping_add(STATE_OFF));
            let mut saved_ebx = ebx;
            // Scratch slot holding ebx across the object-resolution block.
            let mut slot_18: u32 = 0;
            if rd32(st4.wrapping_add(0x2a0)) & 0x400 != 0 {
                let mut node = rd32(rd32(st4.wrapping_add(0x224)).wrapping_add(0x2e0));
                if node != 0 {
                    let mut edx = (rd32(node.wrapping_add(8)) >> 1) & 7;
                    loop {
                        let bits = (rd32(node.wrapping_add(8)) >> 1) & 7;
                        if edx < bits {
                            break;
                        }
                        if rd32(node.wrapping_add(4)) == 0x41e {
                            panic!("stage 3: walk exit to cc0aa8 not implemented");
                        }
                        edx = bits;
                        node = rd32(node.wrapping_add(0x0c));
                        if node == 0 {
                            break;
                        }
                    }
                }
            }
            // cc0832 merge.
            let set40 = ebx == 0x3b
                || rd32(this.wrapping_add(0x40)) == 0x3b
                || ebx == 0x34
                || rd32(rd32(this.wrapping_add(STATE_OFF)).wrapping_add(0x2a0)) & 0x400 != 0;
            if set40 {
                wr32(this.wrapping_add(0x40), 0xffff_ffff);
            }
            let ecx40 = rd32(this.wrapping_add(0x40));
            if ecx40 != 0xffff_ffff {
                let f50 = rd32(this.wrapping_add(FLAGS_OFF));
                let gate = (f50 >> 1) & 1 != 0 || (ecx40 != 0x34 && ecx40 != 0x35);
                if f50 & 0x40000 != 0 && gate {
                    let mut fallthrough = true;
                    let h1: u32 =
                        lf_checker_rt::callee_thiscall!(11, u32, rd32(this.wrapping_add(STATE_OFF)));
                    if (h1 as u8) == 0 {
                        let c1: u32 =
                            lf_checker_rt::callee_cdecl!(13, u32, rd32(this.wrapping_add(0x40)));
                        if (c1 as u8) == 0 {
                            fallthrough = false;
                        }
                    }
                    if fallthrough {
                        let h2: u32 = lf_checker_rt::callee_thiscall!(
                            12,
                            u32,
                            rd32(this.wrapping_add(STATE_OFF))
                        );
                        if (h2 as u8) != 0 {
                            let c2: u32 = lf_checker_rt::callee_cdecl!(
                                14,
                                u32,
                                rd32(this.wrapping_add(0x40))
                            );
                            if (c2 as u8) != 0 {
                                fallthrough = false;
                            }
                        }
                    }
                    if fallthrough {
                        ebx = rd32(this.wrapping_add(0x40));
                        saved_ebx = ebx;
                    }
                }
            }
            // cc08b7 merge.
            let s1: u32 = lf_checker_rt::callee_cdecl!(15, u32, ebx);
            let mut edi = s1;
            if edi == 0 {
                edi = lf_checker_rt::callee_cdecl!(16, u32, ebx);
            }
            if (rd32(edi.wrapping_add(0x378)) >> 0x0d) & 1 == 0 {
                let st = rd32(this.wrapping_add(STATE_OFF));
                let _: u32 = lf_checker_rt::callee_thiscall!(18, u32, edi, st);
            }
            ebx = rd32(edi.wrapping_add(4));
            slot_18 = ebx;
            let mut ebp: u32 = 0;
            if ebx != 0xffff_ffff {
                ebp = lf_checker_rt::callee_cdecl!(17, u32, ebx);
                if (rd32(ebp.wrapping_add(0x378)) >> 0x0d) & 1 == 0 {
                    let st = rd32(this.wrapping_add(STATE_OFF));
                    let _: u32 = lf_checker_rt::callee_thiscall!(19, u32, ebp, st);
                }
            }
            wr32(this.wrapping_add(0x30), 0);
            wr8(lf_checker_rt::relocated(0x171bf96), 0);
            wr32(lf_checker_rt::relocated(0x10514a8), 0x3f80_0000);
            wr8(lf_checker_rt::relocated(0x171bf97), 0);
            let f50 = rd32(this.wrapping_add(FLAGS_OFF));
            if (f50 >> 1) & 1 != 0 {
                if f50 & 4 == 0 {
                    let a40 = rd32(this.wrapping_add(0x40));
                    if a40 != saved_ebx && a40 != 0xffff_ffff {
                        let _: u32 = lf_checker_rt::callee_thiscall!(
                            20, u32, this, saved_ecx, a40, 0xc080_0000
                        );
                    }
                }
                if (rd32(edi.wrapping_add(0x378)) >> 5) & 1 == 0 {
                    let mut proceed = true;
                    if ebx != 0xffff_ffff {
                        let s3: u32 = lf_checker_rt::callee_cdecl!(21, u32, ebx);
                        if (rd32(s3.wrapping_add(0x378)) >> 5) & 1 != 0 {
                            proceed = false;
                        }
                    }
                    if proceed {
                        edi = lf_checker_rt::callee_cdecl!(22, u32, 0x2f);
                        if edi == 0 {
                            edi = lf_checker_rt::callee_cdecl!(23, u32, 0x2f);
                        }
                        slot_18 = 0xffff_ffff;
                    }
                }
            }
            // cc09d3.
            let h3: u32 =
                lf_checker_rt::callee_thiscall!(24, u32, rd32(this.wrapping_add(STATE_OFF)));
            if (h3 as u8) != 0 {
                let c38 = rd32(this.wrapping_add(0x38)) as i32;
                if c38 >= 0 {
                    let lim = rd32(lf_checker_rt::relocated(0x11735b4))
                        .wrapping_sub(rd32(this.wrapping_add(0x34)))
                        as i32;
                    if lim > c38 {
                        let st = rd32(this.wrapping_add(STATE_OFF));
                        let _: u32 = lf_checker_rt::callee_thiscall!(25, u32, st, 0, 0xffff_ffff);
                        wr32(this.wrapping_add(0x34), 0);
                        wr32(this.wrapping_add(0x38), 0xffff_ffff);
                    }
                }
            }
            // cc0a0c: exit through the epilogue when nonzero.
            let fin: u32 = lf_checker_rt::callee_thiscall!(26, u32, this, edi);
            if (fin as u8) != 0 {
                return fin;
            }
            panic!("stage 3: cc0a1c fallthrough not implemented");
        }
        let _: u32 = lf_checker_rt::callee_thiscall!(EARLY_CALLEE, u32, state2, 1);
        wr32(this.wrapping_add(0x48), 0xffff_ffff);
        wr32(this.wrapping_add(0x30), 0);
        wr32(this.wrapping_add(0x5c), 0);
        wr32(this.wrapping_add(0x4c), 0);
        wr32(this.wrapping_add(0x1c), 0x4040_0000);
        wr32(this.wrapping_add(0x20), 0);
        lf_checker_rt::callee_thiscall!(TAIL_CALLEE, u32, this)
    }
});
