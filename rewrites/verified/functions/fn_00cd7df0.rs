// original: 0x00CD7DF0 CTaskComplexFollowLeaderInFormation::vf20

/// Original: 0x00CD7DF0 (thiscall, one stack word, callee pops 4).
export!(thiscall, rw_00cd7df0(task: u32, ped: u32) -> u32 {
    unsafe {
        const CHILD: u32 = 0x08;
        const SUB: u32 = 0x18;
        const SPACING: u32 = 0x1c;
        const DONE_BYTE: u32 = 0x26c;
        const DONE_BIT: u8 = 4;
        const REGS: u32 = 0xb30;
        const POS: u32 = 0x20;
        const NAV: u32 = 0x224;
        const STATE_SLOT: u32 = 0x0c;
        const STATE_KEEP: u32 = 0x2d4;
        const STATE_ADOPT: u32 = 0x2de;
        const STATE_JOIN: u32 = 0x11d;
        const KIND_WANT: u32 = 0x3b6;
        const LIMIT_BYTE: u32 = 0x1070;
        const ADMIT_SLOT: u32 = 0x14;
        const ADOPT_SLOT: u32 = 0x4c;
        const LEG_SLOT: u32 = 0x58;
        const CHAIN_OFF: u32 = 0x2e0;
        const NODE_HEAD: u32 = 0x04;
        const NODE_NEXT: u32 = 0x0c;
        const HEAD_WANT: u32 = 0x76c;
        const KIND2_WANT: u32 = 0x41a;
        const HOLDER: u32 = 0x08;
        const HOLDER_WORD: u32 = 0x18;

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
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn fsub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        unsafe fn hook0(obj: u32, slot: u32) -> u32 {
            unsafe {
                let tgt = rd32(rd32(obj).wrapping_add(slot));
                let f: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(tgt as usize);
                f(obj)
            }
        }

        let mut slot = rd32(task.wrapping_add(CHILD));
        let sub = rd32(task.wrapping_add(SUB));
        if sub == 0 {
            let _: u32 = callee_thiscall!(10, u32, task, ped);
            slot = callee_thiscall!(11, u32, task, 0x516, ped);
        } else if hook0(rd32(task.wrapping_add(CHILD)), STATE_SLOT) != STATE_KEEP {
            let done_set = rd8(sub.wrapping_add(DONE_BYTE)) & DONE_BIT != 0;
            let regs = rd32(sub.wrapping_add(REGS));
            if !done_set || regs == 0 {
                let ch = rd32(task.wrapping_add(CHILD));
                if hook0(ch, STATE_SLOT) == STATE_ADOPT {
                    if rd8(ch.wrapping_add(0x0c)) & 1 == 0 {
                        let tgt = rd32(rd32(ch).wrapping_add(ADMIT_SLOT));
                        let admit: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
                            core::mem::transmute(tgt as usize);
                        if admit(ch, ped, 0, 0) & 0xff == 0 {
                            // fall through to join with slot unchanged
                        } else {
                            let f = rd32(ch.wrapping_add(0x0c)) | 2;
                            (ch.wrapping_add(0x0c) as *mut u32).write_unaligned(f);
                            let tgt = rd32(rd32(task).wrapping_add(ADOPT_SLOT));
                            let adopt: extern "thiscall" fn(u32, u32) -> u32 =
                                core::mem::transmute(tgt as usize);
                            slot = adopt(task, ped);
                        }
                    } else {
                        let tgt = rd32(rd32(task).wrapping_add(ADOPT_SLOT));
                        let adopt: extern "thiscall" fn(u32, u32) -> u32 =
                            core::mem::transmute(tgt as usize);
                        slot = adopt(task, ped);
                    }
                }
            } else {
                let ea = rd32(sub.wrapping_add(POS));
                let pa = rd32(ped.wrapping_add(POS));
                let dx = fsub(rdf(ea.wrapping_add(0x30)), rdf(pa.wrapping_add(0x30)));
                let dy = fsub(rdf(ea.wrapping_add(0x34)), rdf(pa.wrapping_add(0x34)));
                let dz = fsub(rdf(ea.wrapping_add(0x38)), rdf(pa.wrapping_add(0x38)));
                let nav = rd32(ped.wrapping_add(NAV)).wrapping_add(0x44);
                let loc: u32 = callee_thiscall!(4, u32, nav, 1);
                if loc != 0 && hook0(loc, STATE_SLOT) == KIND_WANT {
                    let n2 = add(add(mul(dx, dx), mul(dy, dy)), mul(dz, dz));
                    let cap = f32::from_bits(*global::<u32>(0xFE8B40));
                    if cap > n2 {
                        let ident: u32 = callee_thiscall!(5, u32, ped);
                        if ident == 0 {
                            let b30 = regs;
                            let tgt = rd32(rd32(b30).wrapping_add(LEG_SLOT));
                            let meas: extern "thiscall" fn(u32) -> f32 =
                                core::mem::transmute(tgt as usize);
                            let leg_a = meas(b30);
                            let leg_b = meas(b30);
                            let xa = rd32(ped.wrapping_add(POS));
                            let xc = rd32(sub.wrapping_add(POS));
                            let dx1 = fsub(rdf(xa.wrapping_add(0x30)), rdf(xc.wrapping_add(0x30)));
                            let dy1 = fsub(rdf(xa.wrapping_add(0x34)), rdf(xc.wrapping_add(0x34)));
                            let dz1 = fsub(rdf(xa.wrapping_add(0x38)), rdf(xc.wrapping_add(0x38)));
                            let prod = mul(leg_a, leg_b);
                            let n2p =
                                add(add(mul(dy1, dy1), mul(dx1, dx1)), mul(dz1, dz1));
                            if prod > n2p {
                                let bc = rd32(sub.wrapping_add(REGS));
                                let ev: u32 = callee_thiscall!(14, u32, bc);
                                let lim = rd8(rd32(sub.wrapping_add(REGS)).wrapping_add(LIMIT_BYTE));
                                if (ev as i32) < (lim as i32) {
                                    let ch = rd32(task.wrapping_add(CHILD));
                                    let ok: u32 =
                                        callee_thiscall!(15, u32, ch, ped, 0, 0);
                                    if ok & 0xff != 0 {
                                        let _: u32 = callee_thiscall!(10, u32, task, ped);
                                        slot = callee_thiscall!(11, u32, task, 0x2de, ped);
                                    }
                                }
                            } else {
                                let sp = rdf(task.wrapping_add(SPACING));
                                if mul(sp, sp) > n2p {
                                    let ch = rd32(task.wrapping_add(CHILD));
                                    let ok: u32 =
                                        callee_thiscall!(16, u32, ch, ped, 0, 0);
                                    if ok & 0xff != 0 {
                                        let r2: u32 = callee_thiscall!(13, u32, ped);
                                        if r2 == 0 {
                                            return 0;
                                        }
                                        let g: u32 = *global::<u32>(0x167E2A0);
                                        let gs: u32 = callee_thiscall!(17, u32, g);
                                        let esi2 = if gs == 0 {
                                            0
                                        } else {
                                            callee_thiscall!(22, u32, gs, r2)
                                        };
                                        let gt: u32 = callee_thiscall!(18, u32, g);
                                        if gt == 0 {
                                            return 0;
                                        }
                                        let gu: u32 = callee_thiscall!(19, u32, g);
                                        let iax = if gu == 0 {
                                            0
                                        } else {
                                            callee_thiscall!(23, u32, gu, 0, 0)
                                        };
                                        return callee_thiscall!(24, u32, gt, esi2, iax, 0, 0);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        // join
        let ch = rd32(task.wrapping_add(CHILD));
        if hook0(ch, STATE_SLOT) == STATE_JOIN {
            let sub2 = rd32(task.wrapping_add(SUB));
            if sub2 != 0 {
                let chain = rd32(sub2.wrapping_add(NAV));
                let mut node = rd32(chain.wrapping_add(CHAIN_OFF));
                let mut found = false;
                while node != 0 {
                    // The key computation feeds an always-taken branch
                    // (both halves read the same word), so only the head
                    // test and the link step are live.
                    if rd32(node.wrapping_add(NODE_HEAD)) == HEAD_WANT {
                        found = true;
                        break;
                    }
                    node = rd32(node.wrapping_add(NODE_NEXT));
                }
                let mut eax = 0u32;
                if found {
                    let base = chain.wrapping_add(CHAIN_OFF);
                    let mut slot18 = 0u32;
                    let sel: u32 = callee_thiscall!(26, u32, base, 0x779, 0);
                    if sel & 0xff == 0 {
                        slot18 = callee_thiscall!(27, u32, base, 0x76c, 5);
                    }
                    let holder = rd32(rd32(task.wrapping_add(CHILD)).wrapping_add(HOLDER));
                    if holder != 0 {
                        let kind = hook0(holder, STATE_SLOT);
                        if kind == KIND2_WANT
                            && rd32(holder.wrapping_add(HOLDER_WORD)) == slot18
                        {
                            let _: u32 = callee_thiscall!(31, u32, task, ped);
                            return slot;
                        }
                    }
                    if slot18 == 0 {
                        let g: u32 = *global::<u32>(0x167E2A0);
                        let gv: u32 = callee_thiscall!(20, u32, g);
                        if gv == 0 {
                            eax = 0;
                        } else {
                            let pv = rd32(ped.wrapping_add(POS)).wrapping_add(0x30);
                            eax = callee_thiscall!(28, u32, gv, 0x0c, 0, pv, 0, 0xffff_ffff);
                        }
                    } else {
                        let g: u32 = *global::<u32>(0x167E2A0);
                        let gw: u32 = callee_thiscall!(21, u32, g);
                        if gw == 0 {
                            eax = 0;
                        } else {
                            let pv = rd32(slot18.wrapping_add(POS)).wrapping_add(0x30);
                            eax = callee_thiscall!(29, u32, gw, 0x0f, slot18, pv, 0, 0xffff_ffff);
                        }
                    }
                } else {
                    let base = chain.wrapping_add(CHAIN_OFF);
                    let sel: u32 = callee_thiscall!(25, u32, base, 0x41a, 0);
                    if sel & 0xff != 0 {
                        let hsub = rd32(rd32(task.wrapping_add(CHILD)).wrapping_add(HOLDER_WORD));
                        let mut proceed = false;
                        if hsub == 0 {
                            proceed = true;
                        } else if hook0(hsub, STATE_SLOT) == KIND2_WANT {
                            let hw = rd32(rd32(rd32(task.wrapping_add(CHILD)).wrapping_add(HOLDER)).wrapping_add(HOLDER_WORD));
                            if hw != 0 {
                                proceed = true;
                            }
                        } else {
                            proceed = true;
                        }
                        if proceed {
                            let g: u32 = *global::<u32>(0x167E2A0);
                            let gv: u32 = callee_thiscall!(20, u32, g);
                            if gv == 0 {
                                eax = 0;
                            } else {
                                let pv = rd32(ped.wrapping_add(POS)).wrapping_add(0x30);
                                eax = callee_thiscall!(28, u32, gv, 0x0c, 0, pv, 0, 0xffff_ffff);
                            }
                            let c10 = rd32(task.wrapping_add(CHILD));
                            let _: u32 = callee_thiscall!(30, u32, c10, eax);
                            let _: u32 = callee_thiscall!(31, u32, task, ped);
                            return slot;
                        }
                        let _: u32 = callee_thiscall!(31, u32, task, ped);
                        return slot;
                    }
                    let _: u32 = callee_thiscall!(31, u32, task, ped);
                    return slot;
                }
                if found {
                    let c10 = rd32(task.wrapping_add(CHILD));
                    let _: u32 = callee_thiscall!(30, u32, c10, eax);
                }
            }
        }
        let _: u32 = callee_thiscall!(31, u32, task, ped);
        slot
    }
});
