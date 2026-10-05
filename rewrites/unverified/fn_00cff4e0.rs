// original: 0x00cff4e0 FIGHT_RUN

/// Decide whether a fighter starts, continues or drops its combat task.
///
/// `this` is the fighter object, `par` a task parameter block. Returns an
/// 8-bit status in `al` (1 = engage or keep the task, 0 = stand down); the
/// upper 24 bits of `eax` keep whatever the last step left there (a callee
/// answer, a kept pointer or a masked flag word) and are reproduced exactly.
///
/// Behaviour in order:
/// 1. Ask helper 0 about the parameter block's sequence slot. A flag bit
///    at `this+0x60` or a missing subject (`this+0x3c`) returns at once.
/// 2. When the parameter mode byte selects it, poll helper 1 and combine
///    its answer with two global words into three flag bytes.
/// 3. Run the weapon checks (helpers 3, 4, 5) into a two-bit level, then
///    the target chain (helper 6 three times, helper 7 twice, planted
///    slot `0x128` once) into one `armed` bit.
/// 4. Unless two subject bytes veto it, poll helper 8 twice and the
///    subject's planted slot `0x128`; several combinations return 1 here.
/// 5. With the rate bit set, compare a level-derived score against the
///    float answered through the parameter block's planted slot `0xfc`.
/// 6. Walk the subject's state word and stance fields, polling helpers 11,
///    12 and 13; each can return 1.
/// 7. Merge the level into `this+0x60`, optionally ask helpers 8, 9 and 10
///    about a ranged attack, then scan a 16-entry ally array (helper 1 to
///    test, helper 14 to confirm); two confirmations return 1.
/// 8. Fall through helpers 16 and 15 into a rating table indexed by the
///    merged flags; scale the rating, compare it against a helper-17
///    reading, and either launch helper 18 (return 1) or tune the stance
///    and return 0.
///
/// Faithfulness notes: the original keeps one scratch byte in its incoming
/// argument slot, so the stack comparison is off for this function (its
/// value is observed through every branch it feeds instead). Two
/// null checks (subject re-test, helper-4 object) cannot fire because an
/// earlier check returns first; they are kept as written. All float
/// arithmetic is single-precision in the original's operand order.
///
/// Original: 0x00cff4e0 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00cff4e0(this: u32, par: u32) -> u32 {
    unsafe {
        const THIS_SUBJ: u32 = 0x3c;
        const THIS_FLAGS: u32 = 0x60;
        const THIS_MODE: u32 = 0x64;
        const THIS_RATE: u32 = 0x94;
        const PAR_SEQ: u32 = 0x2b0;
        const PAR_MODE: u32 = 0xa60;
        const PAR_SUB: u32 = 0x21c;
        const PAR_TRAV: u32 = 0x224;
        const PAR_IDX: u32 = 0x2e;
        const PAR_26C: u32 = 0x26c;
        const PAR_BA4: u32 = 0xba4;
        const PAR_LAUNCH: u32 = 0x570;
        const PAR_X20: u32 = 0x20;
        const SUB_218: u32 = 0x218;
        const SUB_219: u32 = 0x219;
        const SUB_SUB: u32 = 0x21c;
        const SUB_TRAV: u32 = 0x224;
        const SUB_26C: u32 = 0x26c;
        const SUB_A60: u32 = 0xa60;
        const SUB_A70: u32 = 0xa70;
        const SUB_STATE: u32 = 0x28;
        const G_INDEX: u32 = 0x0012_f9f3c;
        const G_MODE: u32 = 0x0011_d6fd4;
        const G_TABLE: u32 = 0x0012_95cd8;
        const G_BYTE: u32 = 0x0017_1de5c;
        const G_RATES: u32 = 0x0010_53c68;
        const C_SLOPE: u32 = 0x00fe_8b38; // 20.0
        const C_BASE: u32 = 0x00fe_8bc0; // 120.0
        const C_ONE: u32 = 0x00fe_88e8; // 1.0
        const C_SCALE: u32 = 0x00fe_8870; // 0.66
        const C_TICK: u32 = 0x00fe_8684; // 3.05e-05
        const VT_SLOT_A: u32 = 0x128;
        const VT_SLOT_C: u32 = 0xfc;
        const RATE_BITS: u32 = 0x41a0_0000; // 20.0f

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
        unsafe fn rd16(a: u32) -> u16 {
            unsafe { (a as *const u16).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        unsafe fn rglob(a: u32) -> u32 {
            unsafe { rd32(lf_checker_rt::relocated(a)) }
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }

        // Step 1: helper 0 and the instant exits.
        let seq = rd32(par.wrapping_add(PAR_SEQ));
        let t = seq.wrapping_add(3).wrapping_mul(3);
        let pushval = rd32(par.wrapping_add(t.wrapping_mul(4)).wrapping_add(PAR_SEQ));
        let r0: u32 = lf_checker_rt::callee_cdecl!(0, u32, pushval);
        let mut eaxv = r0;
        let s2 = r0;
        if rd8(this.wrapping_add(THIS_FLAGS)) & 8 != 0 {
            return (eaxv & 0xffff_ff00) | 1;
        }
        let subj = rd32(this.wrapping_add(THIS_SUBJ));
        if subj == 0 {
            return eaxv & 0xffff_ff00;
        }
        // Step 2: the helper-1 poll and the three flag bytes.
        let mut arg_local: u32;
        let bl0: u8;
        let bh_orig: u8;
        if rd8(par.wrapping_add(PAR_MODE)) == 1
            && rd32(rd32(par.wrapping_add(PAR_SUB)).wrapping_add(0x12c)) != 2
        {
            bl0 = 1;
            arg_local = 1;
            let r1: u32 = lf_checker_rt::callee_thiscall!(1, u32, par);
            eaxv = r1;
            bh_orig = if r1 & 0xff == 0 { 0 } else { 1 };
        } else {
            bl0 = 0;
            arg_local = 0;
            bh_orig = 0;
        }
        let flag0: u8 = if bl0 != 0
            && (rd16(par.wrapping_add(PAR_IDX)) as i16) as i32
                == rglob(G_INDEX) as i32
        {
            1
        } else {
            0
        };
        let flag1: u8 = if rglob(G_MODE) == 3 {
            let r2: u32 = lf_checker_rt::callee_cdecl!(2, u32,);
            eaxv = r2;
            if r2 & 0xff != 0 && bh_orig != 0 {
                1
            } else {
                0
            }
        } else {
            0
        };
        // The 0x26c flag byte, carried to helpers 3 and 5.
        let flag2: u8 = if rd8(par.wrapping_add(PAR_MODE)) == 1
            && rd8(subj.wrapping_add(SUB_26C)) & 4 != 0
            && rd8(par.wrapping_add(PAR_26C)) & 4 != 0
        {
            0
        } else {
            1
        };
        // Step 3: weapon level in ebx.
        let mut ebx: u32 = 0;
        let r3: u32 =
            lf_checker_rt::callee_thiscall!(3, u32, subj.wrapping_add(PAR_SEQ), flag2 as u32);
        eaxv = r3;
        let mut need_id5 = r3 & 0xff == 0;
        if !need_id5 {
            let r4: u32 =
                lf_checker_rt::callee_thiscall!(4, u32, subj.wrapping_add(PAR_SEQ));
            eaxv = r4;
            if r4 != 0 {
                ebx = 1;
            } else {
                need_id5 = true;
            }
        }
        if need_id5 {
            let r5: u32 =
                lf_checker_rt::callee_thiscall!(5, u32, subj.wrapping_add(PAR_SEQ), flag2 as u32);
            eaxv = r5;
            if r5 & 0xff != 0 {
                ebx = 2;
            }
        }
        // The target chain into cl.
        let cl: u8;
        let r6a: u32 = lf_checker_rt::callee_thiscall!(6, u32, subj);
        eaxv = r6a;
        if r6a == 0 {
            cl = 0;
        } else {
            let r6b: u32 = lf_checker_rt::callee_thiscall!(6, u32, subj);
            eaxv = r6b;
            if r6b == 0 {
                cl = 0;
            } else {
                let r7a: u32 =
                    lf_checker_rt::callee_thiscall!(7, u32, r6b.wrapping_add(8));
                eaxv = r7a;
                if r7a == 0 {
                    cl = 0;
                } else {
                    let r6c: u32 = lf_checker_rt::callee_thiscall!(6, u32, subj);
                    eaxv = r6c;
                    if r6c == 0 {
                        cl = 0;
                    } else {
                        let r7b: u32 =
                            lf_checker_rt::callee_thiscall!(7, u32, r6c.wrapping_add(8));
                        eaxv = r7b;
                        if r7b == 0 {
                            cl = 0;
                        } else {
                            let vt = rd32(r7b);
                            let slot = rd32(vt.wrapping_add(VT_SLOT_A));
                            let f: extern "thiscall" fn(u32) -> u32 =
                                core::mem::transmute(slot as usize);
                            let ra: u32 = f(r7b);
                            eaxv = ra;
                            cl = if ra & 0xff == 0 { 0 } else { 1 };
                        }
                    }
                }
            }
        }
        // Step 4: subject-byte veto, helper-8 pair, subject slot.
        let b218 = rd8(subj.wrapping_add(SUB_218));
        let b219 = rd8(subj.wrapping_add(SUB_219));
        let need_id8 = cl != 0 || (b218 == 0 && b219 != 0);
        let mut alv: u8;
        if need_id8 {
            let r8a: u32 = lf_checker_rt::callee_cdecl!(8, u32,);
            eaxv = r8a;
            if r8a != 0 {
                let r8b: u32 = lf_checker_rt::callee_cdecl!(8, u32,);
                eaxv = r8b;
                if rd8(r8b.wrapping_add(0x5a)) & 8 != 0 && arg_local != 0 {
                    wr32(
                        this.wrapping_add(THIS_FLAGS),
                        rd32(this.wrapping_add(THIS_FLAGS)) | 8,
                    );
                    return (eaxv & 0xffff_ff00) | 1;
                }
            }
        }
        let vt_b = rd32(subj);
        let slot_b = rd32(vt_b.wrapping_add(VT_SLOT_A));
        let fb: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(slot_b as usize);
        let rb: u32 = fb(subj);
        eaxv = rb;
        if rb & 0xff != 0 {
            alv = (arg_local & 0xff) as u8;
            eaxv = (eaxv & 0xffff_ff00) | alv as u32;
        } else {
            eaxv = subj;
            alv = (arg_local & 0xff) as u8;
            eaxv = (eaxv & 0xffff_ff00) | alv as u32;
            if !(rd8(subj.wrapping_add(SUB_A60)) == 2 && cl == 0 && alv != 0) {
                // fall through with alv
            } else {
                wr32(
                    this.wrapping_add(THIS_FLAGS),
                    rd32(this.wrapping_add(THIS_FLAGS)) | 8,
                );
                return (eaxv & 0xffff_ff00) | 1;
            }
        }
        // Step 5: the level-score float check.
        if rd32(this.wrapping_add(THIS_FLAGS)) & 0x800 != 0 && alv != 0 {
            let idx = (rd16(par.wrapping_add(PAR_IDX)) as i16) as i32;
            if idx != rglob(G_INDEX) as i32 {
                let tp = rd32(
                    lf_checker_rt::relocated(G_TABLE)
                        .wrapping_add((idx as u32).wrapping_mul(4)),
                );
                let b = rd8(tp.wrapping_add(0xee)) as i32;
                let esi4 = (4 - b).max(0);
                let vt_c = rd32(par);
                let slot_c = rd32(vt_c.wrapping_add(VT_SLOT_C));
                let fc: extern "thiscall" fn(u32) -> f32 =
                    core::mem::transmute(slot_c as usize);
                let st0: f32 = fc(par);
                eaxv = st0.to_bits();
                let mut x = mul(esi4 as f32, rdf(lf_checker_rt::relocated(C_SLOPE)));
                x = add(x, rdf(lf_checker_rt::relocated(C_BASE)));
                if x > st0 {
                    wr32(
                        this.wrapping_add(THIS_FLAGS),
                        rd32(this.wrapping_add(THIS_FLAGS)) | 8,
                    );
                    return (eaxv & 0xffff_ff00) | 1;
                }
                alv = (arg_local & 0xff) as u8;
                eaxv = (eaxv & 0xffff_ff00) | alv as u32;
            }
        }
        // Step 6: state word and stance fields.
        if alv != 0 {
            eaxv = s2;
            if s2 != 0 {
                let k = rd32(s2.wrapping_add(0xc));
                eaxv = k;
                if k != 2 && k != 3 && k != 4 {
                    let ecx1 = subj;
                    if ecx1 != 0 {
                        let shifted = rd32(ecx1.wrapping_add(SUB_STATE)) >> 0x15;
                        eaxv = shifted;
                        if shifted & 1 != 0 {
                            wr32(
                                this.wrapping_add(THIS_FLAGS),
                                rd32(this.wrapping_add(THIS_FLAGS)) | 8,
                            );
                            return (eaxv & 0xffff_ff00) | 1;
                        }
                        if rd32(ecx1.wrapping_add(SUB_A70)) == 1 {
                            wr32(
                                this.wrapping_add(THIS_FLAGS),
                                rd32(this.wrapping_add(THIS_FLAGS)) | 8,
                            );
                            return (eaxv & 0xffff_ff00) | 1;
                        }
                    }
                }
            }
            if rd32(subj.wrapping_add(SUB_A70)) == 1 {
                wr32(
                    this.wrapping_add(THIS_FLAGS),
                    rd32(this.wrapping_add(THIS_FLAGS)) | 8,
                );
                return (eaxv & 0xffff_ff00) | 1;
            }
            let trav = rd32(subj.wrapping_add(SUB_TRAV));
            let r11a: u32 = lf_checker_rt::callee_thiscall!(
                11,
                u32,
                trav.wrapping_add(0x2e0),
                0x2e7,
                0
            );
            eaxv = r11a;
            if r11a & 0xff != 0 {
                wr32(
                    this.wrapping_add(THIS_FLAGS),
                    rd32(this.wrapping_add(THIS_FLAGS)) | 8,
                );
                return (eaxv & 0xffff_ff00) | 1;
            }
        }
        // Flag1 gate into helpers 12 and 13 (runs when flag1 is clear).
        if flag1 == 0 {
            let r12: u32 = lf_checker_rt::callee_cdecl!(12, u32,);
            eaxv = r12;
            if r12 & 0xff != 0
                && subj != 0
                && rd8(subj.wrapping_add(SUB_26C)) & 4 != 0
            {
                let r13a: u32 = lf_checker_rt::callee_thiscall!(13, u32, subj);
                eaxv = r13a;
                if r13a & 0xff != 0 {
                    let parsub = rd32(par.wrapping_add(PAR_SUB));
                    eaxv = parsub;
                    if rd32(parsub.wrapping_add(0x12c)) != 2
                        && rd8(par.wrapping_add(PAR_MODE)) == 1
                    {
                        wr32(
                            this.wrapping_add(THIS_FLAGS),
                            rd32(this.wrapping_add(THIS_FLAGS)) | 8,
                        );
                        return (eaxv & 0xffff_ff00) | 1;
                    }
                }
            }
        }
        // Step 7a: the helper-5/helper-11b pair.
        let mut cl2 = (arg_local & 0xff) as u8;
        eaxv = (eaxv & 0xffff_ff00) | cl2 as u32;
        if cl2 != 0 && rd32(this.wrapping_add(THIS_FLAGS)) & 0xc0 <= 0x40 {
            let r5b: u32 =
                lf_checker_rt::callee_thiscall!(5, u32, par.wrapping_add(PAR_SEQ), 1);
            eaxv = r5b;
            if r5b & 0xff == 0 {
                let trav = rd32(subj.wrapping_add(SUB_TRAV));
                let r11b: u32 = lf_checker_rt::callee_thiscall!(
                    11,
                    u32,
                    trav.wrapping_add(0x2e0),
                    0x640,
                    0
                );
                eaxv = r11b;
                if r11b & 0xff != 0 {
                    wr32(
                        this.wrapping_add(THIS_FLAGS),
                        rd32(this.wrapping_add(THIS_FLAGS)) | 8,
                    );
                    return (eaxv & 0xffff_ff00) | 1;
                }
            }
            cl2 = (arg_local & 0xff) as u8;
        }
        // Step 7b: merge the level into the flags.
        let x = rd32(this.wrapping_add(THIS_FLAGS));
        if x & 0x20 != 0 {
            let masked = (x >> 6) & 3;
            eaxv = masked;
            if masked >= ebx {
                return eaxv & 0xffff_ff00;
            }
        }
        let newx = (((ebx << 6) ^ x) & 0xc0) ^ x;
        wr32(this.wrapping_add(THIS_FLAGS), newx);
        ebx = newx;
        // Step 7c: the ranged-attack helpers.
        if newx & 0x20 == 0 && cl2 != 0 {
            let r8c: u32 = lf_checker_rt::callee_cdecl!(8, u32,);
            eaxv = r8c;
            let h = r8c;
            let r9: u32 = lf_checker_rt::callee_thiscall!(9, u32, h);
            eaxv = r9;
            if (r9 as i32) > 0 {
                let aim = rd32(par.wrapping_add(PAR_X20)).wrapping_add(0x30);
                let r10: u32 = lf_checker_rt::callee_cdecl!(
                    10, u32, par, aim, RATE_BITS, 0xffff_ffff, 0xffff_ffff, 0, 0
                );
                eaxv = r10;
                if (r10 as i32) > 0 {
                    wr32(
                        this.wrapping_add(THIS_FLAGS),
                        rd32(this.wrapping_add(THIS_FLAGS)) | 0x28,
                    );
                    return (eaxv & 0xffff_ff00) | 1;
                }
            }
        }
        if rd8(lf_checker_rt::relocated(G_BYTE)) != 0 {
            wr32(
                this.wrapping_add(THIS_FLAGS),
                rd32(this.wrapping_add(THIS_FLAGS)) | 0x28,
            );
            return (eaxv & 0xffff_ff00) | 1;
        }
        let xc = rd32(this.wrapping_add(THIS_FLAGS));
        if xc & 0x20 == 0 {
            let sub2 = rd32(subj.wrapping_add(SUB_SUB));
            eaxv = subj;
            eaxv = sub2;
            if rd32(sub2.wrapping_add(0x12c)) == 2 && arg_local != 0 {
                wr32(
                    this.wrapping_add(THIS_FLAGS),
                    rd32(this.wrapping_add(THIS_FLAGS)) | 0x28,
                );
                return (eaxv & 0xffff_ff00) | 1;
            }
        }
        // Step 7d: the rating index.
        let esi_idx = (rd16(par.wrapping_add(PAR_IDX)) as i16) as i32;
        let tp2 = rd32(
            lf_checker_rt::relocated(G_TABLE)
                .wrapping_add((esi_idx as u32).wrapping_mul(4)),
        );
        let e = rd8(tp2.wrapping_add(0xee)) as i32 - 1;
        eaxv = e as u32;
        let mut edx: i32 = if e > 0 { 3 } else { 0 };
        if e < edx {
            edx = e;
        }
        let b64 = rd8(this.wrapping_add(THIS_MODE));
        eaxv = (eaxv & 0xffff_ff00) | b64 as u32;
        if b64 != 0xff {
            edx = b64 as i8 as i32;
        }
        if rd8(par.wrapping_add(PAR_MODE)) == 2 {
            edx = 4;
        } else {
            let ps = rd32(par.wrapping_add(PAR_SUB));
            eaxv = ps;
            if rd32(ps.wrapping_add(0x12c)) == 2 || esi_idx == rglob(G_INDEX) as i32 {
                edx = 4;
            }
        }
        // Step 7e: the ally scan.
        let bl_late: u8;
        let skip_rating: bool;
        if bh_orig == 0 {
            bl_late = (ebx & 0xff) as u8;
            skip_rating = flag0 == 0;
        } else {
            skip_rating = false;
            if (xc & 0xc0) > 0x40 {
                bl_late = bh_orig;
            } else {
                let base =
                    rd32(par.wrapping_add(PAR_TRAV)).wrapping_add(0x168);
                arg_local = 0;
                eaxv = 0;
                for i in 0..16u32 {
                    let elem = rd32(base.wrapping_add(i.wrapping_mul(4)));
                    if elem != 0 && rd8(elem.wrapping_add(0x211)) == 0 {
                        let rl: u32 = lf_checker_rt::callee_thiscall!(1, u32, elem);
                        eaxv = rl;
                        if rl & 0xff != 0 {
                            let re: u32 = lf_checker_rt::callee_thiscall!(
                                14,
                                u32,
                                rd32(elem.wrapping_add(SUB_TRAV)).wrapping_add(0x2e0)
                            );
                            eaxv = re;
                            eaxv = arg_local;
                            if re != 0 {
                                arg_local = arg_local.wrapping_add(1);
                            }
                        }
                    }
                }
                if (eaxv as i32) >= 2 {
                    wr32(
                        this.wrapping_add(THIS_FLAGS),
                        rd32(this.wrapping_add(THIS_FLAGS)) | 0x28,
                    );
                    return (eaxv & 0xffff_ff00) | 1;
                }
                bl_late = bh_orig;
            }
        }
        let bh_late: u8 = flag0;
        // Step 8: helpers 16 and 15 select the rating.
        if skip_rating {
            // edx keeps its scanned value
        } else if flag1 != 0 {
            let r16: u32 =
                lf_checker_rt::callee_thiscall!(16, u32, par.wrapping_add(PAR_SEQ), 0);
            eaxv = r16;
            if r16 & 0xff != 0 {
                edx = 4;
            } else if bh_late == 0 {
                let m60 = rd32(this.wrapping_add(THIS_FLAGS));
                if m60 & 0x4000 == 0 && m60 & 0x20 == 0 {
                    let r15: u32 = lf_checker_rt::callee_thiscall!(
                        15,
                        u32,
                        rd32(par.wrapping_add(PAR_TRAV)),
                        subj,
                        1
                    );
                    eaxv = r15;
                    edx = 3;
                    if r15 & 0xff != 0 {
                        edx = 4;
                    }
                } else {
                    edx = 4;
                }
            } else {
                edx = 4;
            }
        } else if bh_late == 0 {
            let m60 = rd32(this.wrapping_add(THIS_FLAGS));
            if m60 & 0x4000 == 0 && m60 & 0x20 == 0 {
                let r15: u32 = lf_checker_rt::callee_thiscall!(
                    15,
                    u32,
                    rd32(par.wrapping_add(PAR_TRAV)),
                    subj,
                    1
                );
                eaxv = r15;
                edx = 3;
                if r15 & 0xff != 0 {
                    edx = 4;
                }
            } else {
                edx = 4;
            }
        } else {
            edx = 4;
        }
        // Step 8b: the rating table and helper 17.
        wr32(
            this.wrapping_add(THIS_FLAGS),
            rd32(this.wrapping_add(THIS_FLAGS)) | 0x20,
        );
        let m60b = rd32(this.wrapping_add(THIS_FLAGS));
        let tidx = ((m60b >> 6) & 3).wrapping_add((edx as u32).wrapping_mul(3));
        let mut rating = rdf(
            lf_checker_rt::relocated(G_RATES).wrapping_add(tidx.wrapping_mul(4)),
        );
        arg_local = rating.to_bits();
        if rglob(G_MODE) == 1 && 1.0 > rating {
            rating = mul(rating, rdf(lf_checker_rt::relocated(C_SCALE)));
            arg_local = rating.to_bits();
        }
        wr32(this.wrapping_add(THIS_RATE), 0);
        let r17: u32 = lf_checker_rt::callee_cdecl!(17, u32,);
        eaxv = r17;
        let x1 = f32::from_bits(arg_local);
        let mut x0 = (r17 as i32) as f32;
        x0 = mul(x0, rdf(lf_checker_rt::relocated(C_TICK)));
        if x1 < x0 {
            let m = rd32(this.wrapping_add(THIS_FLAGS)) & 0xc0;
            eaxv = m;
            if m > 0x40 {
                wr32(
                    this.wrapping_add(THIS_FLAGS),
                    rd32(this.wrapping_add(THIS_FLAGS)) | 8,
                );
                return (eaxv & 0xffff_ff00) | 1;
            }
            let r18: u32 = lf_checker_rt::callee_thiscall!(
                18,
                u32,
                par.wrapping_add(PAR_LAUNCH),
                lf_checker_rt::relocated(0x00ed_f7e8),
                0,
                0,
                0,
                0xffff_ffff,
                0,
                0,
                0x3f80_0000,
                0,
                0
            );
            eaxv = r18;
            wr32(
                this.wrapping_add(THIS_FLAGS),
                rd32(this.wrapping_add(THIS_FLAGS)) | 8,
            );
            return (eaxv & 0xffff_ff00) | 1;
        }
        let r2b: u32 = lf_checker_rt::callee_cdecl!(2, u32,);
        eaxv = r2b;
        if r2b & 0xff != 0 {
            return eaxv & 0xffff_ff00;
        }
        if rglob(G_MODE) != 1 {
            return eaxv & 0xffff_ff00;
        }
        let masked = rd32(this.wrapping_add(THIS_FLAGS)) & 0xc0;
        eaxv = masked;
        if masked > 0x40 {
            return eaxv & 0xffff_ff00;
        }
        if bl_late == 0 {
            return eaxv & 0xffff_ff00;
        }
        if rd32(par.wrapping_add(PAR_BA4)) != 6 {
            return eaxv & 0xffff_ff00;
        }
        if subj == 0 {
            return eaxv & 0xffff_ff00;
        }
        let r13b: u32 = lf_checker_rt::callee_thiscall!(13, u32, subj);
        eaxv = r13b;
        if r13b & 0xff == 0 {
            return eaxv & 0xffff_ff00;
        }
        wr32(this.wrapping_add(THIS_RATE), RATE_BITS);
        eaxv & 0xffff_ff00
    }
});
