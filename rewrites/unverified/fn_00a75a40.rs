// original: 0x00A75A40 ped_task_update_dispatch (proposed)

/// Pick and run the next update for a ped task, dispatching on task state.
///
/// `this` is the task owner (vtable at `+0x0` with a query slot at `+0x3c`,
/// sub-object at `+0x4`); `arg1` is the task record (flag word at `+0x0`,
/// group pointer at `+0x224`, state block at `+0x2b0`, rate float at
/// `+0xaa0`, heading flag at `+0x20`). Five `.data` globals steer the
/// dispatch: a generation pair, an enable flag, a mode flag and an
/// auxiliary pointer. Return value is the selected callee's answer.
///
/// Behaviour: a prober callee reads the state block while two gate
/// callees plus two predicate callees build an inhibit flag. When the
/// probe hits, its kind is 6 and the flag is clear, a generation-compare
/// section runs a validator pair and stores the fresh generation,
/// returning early on several paths (each with its own defined EAX).
/// Otherwise, in mode 2 two query calls check for states 5/6 and may run
/// a float-parameter callee and return. Otherwise a three-probe block
/// (with a helper pair and a combiner) selects a successor object, a
/// second predicate pair plus four more query calls build four one-bit
/// flags, the heading flag becomes 0x100 or 0x101, and control tail-jumps
/// through the successor into a final callee whose answer is returned.
///
/// The tail jump is a computed jump through a planted vtable. The
/// original writes its argument over its incoming arg slot and jumps, so
/// its stack adjustment on that path is +4, not +8; the rewrite cannot
/// reproduce a computed tail jump in safe Rust, so it makes a balanced
/// call (the stub returns plain on the original side and pops the word
/// on the rewrite side) and returns normally. The call itself, its
/// argument and ECX, and the returned answer are all compared; only the
/// stack-pointer adjustment differs on tail-path trials, so the ESP
/// check is off (a post-hoc audit compares it on the other paths). One
/// predicate argument is a single byte pushed as a full word; only its
/// low byte is compared. Early exits return whatever EAX holds there,
/// including two spots where a byte test has replaced AL.
/// Calling convention: thiscall, one stack word, EAX return.
lf_checker_rt::export!(thiscall, rw_00a75a40(this: u32, arg1: u32) -> u32 {
    unsafe {
        const ST_BLK: u32 = 0x2b0;
        const GRP: u32 = 0x224;
        const GRP_OFF: u32 = 0x44;
        const RATE: u32 = 0xaa0;
        const HEAD: u32 = 0x20;
        const G_GEN: u32 = 0x012fa6f4;
        const G_GEN_SRC: u32 = 0x01173604;
        const G_ENABLE: u32 = 0x01160ec0;
        const G_MODE: u32 = 0x011d6fd4;
        const G_AUX: u32 = 0x0167e2a0;
        const Q_SLOT: u32 = 0x3c;
        const KIND_SLOT: u32 = 0xc;
        const NEXT_SLOT: u32 = 0x30;
        const D_PROBE: u32 = 1;
        const D_GATE_A: u32 = 2;
        const D_GATE_B: u32 = 3;
        const D_PRED_A: u32 = 4;
        const D_PRED_B: u32 = 5;
        const D_KIND: u32 = 6;
        const D_CTX: u32 = 7;
        const D_VAL_A: u32 = 8;
        const D_VAL_B: u32 = 9;
        const D_VAL_C: u32 = 10;
        const D_RUN: u32 = 11;
        const D_ALT: u32 = 12;
        const D_FLOAT: u32 = 13;
        const D_SEL_A: u32 = 14;
        const D_SEL_B: u32 = 15;
        const D_SEL_C: u32 = 16;
        const D_AUX: u32 = 17;
        const D_WRAP: u32 = 18;
        const D_COMB: u32 = 19;
        const D_FLAG_A: u32 = 20;
        const D_FLAG_B: u32 = 21;
        // Planted-vtable stub ids (from the contract, via the objects):
        // query slot +0x3c -> 22, kind slot +0xc -> 23/24, next +0x30 -> 25.
        const V_TAIL: u32 = 26;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        unsafe fn wr16(a: u32, v: u16) {
            unsafe { (a as *mut u16).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn g32(file_va: u32) -> u32 {
            unsafe { rd32(lf_checker_rt::relocated(file_va)) }
        }
        #[inline(always)]
        unsafe fn wg32(file_va: u32, v: u32) {
            unsafe { (lf_checker_rt::relocated(file_va) as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn vcall1(obj: u32, slot: u32, ecx_val: u32, arg: u32) -> u32 {
            unsafe {
                let vt = rd32(obj);
                let f: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(rd32(vt + slot) as usize);
                f(ecx_val, arg)
            }
        }
        #[inline(always)]
        unsafe fn vcall0(obj: u32, slot: u32, ecx_val: u32) -> u32 {
            unsafe {
                let vt = rd32(obj);
                let f: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(rd32(vt + slot) as usize);
                f(ecx_val)
            }
        }

        /// Heading-flag value: second predicate pair plus four query calls.
        /// All calls here are stack-balanced; only the tail call leaks.
        unsafe fn flag_value(this: u32, arg1: u32) -> u16 {
            unsafe {
                const Q_SLOT: u32 = 0x3c;
                const KIND_SLOT: u32 = 0xc;
                const D_FLAG_A: u32 = 20;
                const D_FLAG_B: u32 = 21;

                let ocx = rd32(this + 4);
                if ocx == 0 {
                    return 0x101;
                }
                if vcall0(ocx, KIND_SLOT, ocx) != 4 {
                    return 0x101;
                }
                let is8 = rd32(arg1) == 8;
                let w: u32 = lf_checker_rt::callee_cdecl!(D_FLAG_A, u32, is8 as u32);
                let bl2: u8 = if (w as u8) != 0 {
                    1
                } else if is8 {
                    (lf_checker_rt::callee_cdecl!(D_FLAG_B, u32,) as u8 != 0) as u8
                } else {
                    0
                };
                let m1: u32 = vcall1(this, Q_SLOT, this, 0x413);
                let bh: u8 = if m1 != 0 {
                    1
                } else {
                    (vcall1(this, Q_SLOT, this, 0x13f) != 0) as u8
                };
                let ebp2: u32 = vcall1(this, Q_SLOT, this, 0x40f);
                let eax2: u32 = vcall1(this, Q_SLOT, this, 0x422);
                let st_e = if ebp2 != 0 { rd32(ebp2 + 0x34) } else { 0 };
                let st_a = if eax2 != 0 { rd32(eax2 + 0x4c) } else { 0 };
                let dl: u8 = ((ebp2 != 0 && st_e == 4) || (eax2 != 0 && st_a == 4)) as u8;
                let al: u8 = ((ebp2 != 0 && st_e != 4 && st_e != 0xffff_ffff)
                    || (eax2 != 0 && st_a != 4 && st_a != 0xffff_ffff))
                    as u8;
                // The original compares the arg slot's low byte, which by
                // now holds is8 (a byte store overwrote it); al is 0 there.
                if bl2 != 0 || bh != 0 || dl != 0 {
                    0x100
                } else if al != 0 || !is8 {
                    0x101
                } else {
                    0x100
                }
            }
        }

        // Phase 1: probe plus inhibit flag.
        let ebp: u32 = lf_checker_rt::callee_thiscall!(D_PROBE, u32, arg1 + ST_BLK);
        let grp = rd32(arg1 + GRP) + GRP_OFF;
        let mut bl: u8 = 0;
        if lf_checker_rt::callee_thiscall!(D_GATE_A, u32, grp, 4u32) != 0
            && lf_checker_rt::callee_thiscall!(D_GATE_B, u32, grp, 0x413u32) != 0
            && lf_checker_rt::callee_cdecl!(D_PRED_A, u32, 1u32) as u8 == 0
            && lf_checker_rt::callee_cdecl!(D_PRED_B, u32,) as u8 == 0
        {
            bl = 1;
        }
        // Phase 2: generation section.
        if ebp != 0 {
            let kind: u32 = lf_checker_rt::callee_cdecl!(D_KIND, u32, rd32(ebp + 0x18));
            if rd32(kind + 4) == 6 && bl == 0 {
                let ctx: u32 = lf_checker_rt::callee_thiscall!(D_CTX, u32, arg1);
                let gen_old = g32(G_GEN);
                let gen_new = g32(G_GEN_SRC);
                if rd8(ctx + 0x328d) != bl && g32(G_ENABLE) != 0 && gen_old != gen_new {
                    let s1: u32 = lf_checker_rt::callee_thiscall!(D_VAL_A, u32, arg1);
                    if (s1 as u8) == 0 {
                        let x = rd8(ctx + 0x27de) ^ rd8(ctx + 0x27dc);
                        if x > 0x7f {
                            let _: u32 =
                                lf_checker_rt::callee_thiscall!(D_RUN, u32, arg1, 1u32, 0xffff_ffffu32);
                            let g = g32(G_GEN_SRC);
                            wg32(G_GEN, g);
                            return g;
                        }
                    }
                    let s2: u32 = lf_checker_rt::callee_thiscall!(D_VAL_B, u32, arg1);
                    if (s2 as u8) != 0 {
                        let x = rd8(ctx + 0x27de) ^ rd8(ctx + 0x27dc);
                        if x <= 0x7f {
                            let _: u32 =
                                lf_checker_rt::callee_thiscall!(D_RUN, u32, arg1, 0u32, 0xffff_ffffu32);
                            let g = g32(G_GEN_SRC);
                            wg32(G_GEN, g);
                            return g;
                        }
                        // The xor leaves its result in AL: low byte is x.
                        return (s2 & 0xffff_ff00) | x as u32;
                    }
                    return s2;
                }
                // Compare block: stale EAX is the context pointer here, but
                // the xor tests replace AL before the early exits below.
                let blo = rd8(ctx + 0x27dc);
                let x1 = rd8(ctx + 0x27de) ^ blo;
                if x1 <= 0x7f {
                    return (ctx & 0xffff_ff00) | x1 as u32;
                }
                let x2 = rd8(ctx + 0x27df) ^ blo;
                if x2 > 0x7f || gen_old == gen_new {
                    return (ctx & 0xffff_ff00) | x2 as u32;
                }
                wg32(G_GEN, gen_new);
                let s3: u32 = lf_checker_rt::callee_thiscall!(D_VAL_C, u32, arg1);
                if (s3 as u8) != 0 {
                    let r: u32 =
                        lf_checker_rt::callee_thiscall!(D_RUN, u32, arg1, 0u32, 0xffff_ffffu32);
                    return r;
                }
                let n: u32 = lf_checker_rt::callee_thiscall!(D_ALT, u32, arg1);
                if (n as u8) != 0 {
                    let r: u32 = lf_checker_rt::callee_thiscall!(
                        D_RUN, u32, arg1, 1u32, 0xffff_ffffu32
                    );
                    return r;
                }
                return n;
            }
        }
        // Mode-2 query block.
        if g32(G_MODE) == 2 {
            let o1: u32 = vcall1(this, Q_SLOT, this, 0x40f);
            let o2: u32 = vcall1(this, Q_SLOT, this, 0x422);
            let hit = (o1 != 0 && (rd32(o1 + 0x34) == 5 || rd32(o1 + 0x34) == 6))
                || (o2 != 0 && (rd32(o2 + 0x4c) == 5 || rd32(o2 + 0x4c) == 6));
            if hit {
                let f = rdf(arg1 + RATE);
                let r: u32 = lf_checker_rt::callee_thiscall!(D_FLOAT, u32, arg1, f.to_bits());
                return r;
            }
        }
        // Selector block.
        let c1: u32 = lf_checker_rt::callee_thiscall!(D_SEL_A, u32, grp, 1u32);
        let mut skip_helpers = false;
        if c1 != 0 {
            let c2: u32 = lf_checker_rt::callee_thiscall!(D_SEL_B, u32, grp, 1u32);
            if vcall0(c2, KIND_SLOT, c2) == 2 {
                skip_helpers = true;
            }
        }
        if !skip_helpers {
            let p: u32 = lf_checker_rt::callee_thiscall!(D_AUX, u32, g32(G_AUX));
            let q: u32 = if p != 0 {
                lf_checker_rt::callee_thiscall!(D_WRAP, u32, p)
            } else {
                0
            };
            let _: u32 = lf_checker_rt::callee_thiscall!(D_COMB, u32, grp, q, 1u32);
        }
        let esi2: u32 = lf_checker_rt::callee_thiscall!(D_SEL_C, u32, grp, 1u32);
        wr16(esi2 + HEAD, flag_value(this, arg1));
        // Tail: successor query, then the balanced tail call (see doc).
        let t1: u32 = vcall0(esi2, NEXT_SLOT, esi2);
        lf_checker_rt::callee_thiscall!(V_TAIL, u32, t1, 1u32)
    }
});
