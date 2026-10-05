// original: 0x00be2c30 CTaskComplexReact::vf19 (merged name)

/// Run a complex-react task tick: validate the target, pick a reaction.
///
/// `task` points to the task (`+0x48` detached flag, `+0x34` stage,
/// `+0x45` validated flag, `+0x3c` target, `+0x38` fallback); `ped` is the
/// ped the task runs on (flag word at `+0x26c`, manager link at `+0x224`,
/// matrix at `+0x20`, model id word at `+0x2e`, vehicle link at `+0xb30`).
///
/// The tick marks the ped reacting (flag bit 31) and bails out to the
/// fallback path when detached. It then resolves the ped's manager twice
/// (callees 1 and 2, thiscall with a 1/2 selector and 0x76c): a null answer
/// both times skips validation, otherwise the target is validated (callee 3)
/// and the stage is normalised (a failed validation clears it; a passed one
/// keeps stages 2 and 3, resets any other stage to 1, and returns 0 unless
/// flag bit 2 is set or the stage is already 2). A validated tick rolls a
/// chance die (callee 4): the roll scaled by a constant must not exceed 1.0.
///
/// A stage-2 tick with a live target of kind 0xc0 may dispatch directly
/// (callee 10, whose answer is returned) after a proximity gate: past the
/// flag-bit-2 check it needs a linked vehicle whose check passes (callee 5),
/// a known model id, the ped's state byte equal to 1, a link word not equal
/// to 2, and a squared distance below 64.0, computed as
/// (dy*dy + dx*dx) + dz*dz in that order. Any gate failure, and any other
/// stage, falls into the middle path: the ped must be idle (callee 11), the
/// target live, a wake flag derived from the target's state bytes (overridden
/// by callee 12 for kind 0x80) set, and a range check (callee 13, cdecl)
/// passed. Stage 3 builds an aimed reaction (callee 14, thiscall with eight
/// arguments including a pointer to the (dx, dy, dz) deltas, compared via a
/// call-time snapshot while the pointer itself is skipped) and aims it
/// (callee 15); any other stage builds a plain reaction (callee 16) and tags
/// it 5 when a global switch is set and the model's rank byte is below a
/// global limit (signed), else 2. Both converge on callee 17, whose answer is
/// returned, or 0 when the shared dispatcher (callee 6/7/8/9, one stub id per
/// call site) answers null — except the plain path, which faults on the null
/// target write identically on both sides. The fallback path runs a cleanup
/// (callee 18) and either finishes through callee 19 (whose answer is
/// returned) or resets a ped word to a constant (callee 20, emulated by a
/// scripted write) and returns 0.
///
/// Original: 0x00be2c30 (thiscall, ecx = task, one stack word = ped).
lf_checker_rt::export!(thiscall, rw_00be2c30(task: u32, ped: u32) -> u32 {
    unsafe {
        const DETACHED: u32 = 0x48;
        const STAGE: u32 = 0x34;
        const VALIDATED: u32 = 0x45;
        const TARGET: u32 = 0x3c;
        const FALLBACK: u32 = 0x38;
        const PED_FLAGS: u32 = 0x26c;
        const PED_MGR: u32 = 0x224;
        const MGR_INNER: u32 = 0x44;
        const PED_MATRIX: u32 = 0x20;
        const PED_MODEL: u32 = 0x2e;
        const PED_VEHICLE: u32 = 0xb30;
        const PED_STATEB: u32 = 0xa60;
        const PED_LINK: u32 = 0x21c;
        const LINK_WORD: u32 = 0x12c;
        const TGT_KIND: u32 = 0x28;
        const TGT_UP: u32 = 0x20;
        const TGT_B0: u32 = 0x218;
        const TGT_B1: u32 = 0x219;
        const KIND_MASK: u32 = 0x3c0;
        const KIND_DISPATCH: u32 = 0xc0;
        const KIND_WAKE: u32 = 0x80;
        const MODEL_TABLE: u32 = 0x1295cd8;
        const MODEL_FLAGS: u32 = 0xee;
        const MODEL_RANK: u32 = 0xef;
        const THRESH_A: u32 = 0x10475d4;
        const THRESH_B: u32 = 0x10475d8;
        const ROLL_SCALE: u32 = 0xfe8684;
        const DIST_LIMIT: u32 = 0xfe8b88;
        const AIM_BLEND: u32 = 0x10475e4;
        const RANK_LIMIT: u32 = 0x10475e8;
        const SWITCH: u32 = 0x10521a0;
        const DISPATCHER: u32 = 0x167e2a0;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u32 {
            unsafe { (a as *const u16).read_unaligned() as u32 }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
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
        /// Target position: through the up vector, or the inline fallback.
        #[inline(always)]
        unsafe fn pos_of(t: u32) -> u32 {
            unsafe {
                let up = rd32(t.wrapping_add(TGT_UP));
                if up != 0 {
                    up.wrapping_add(0x30)
                } else {
                    t.wrapping_add(0x10)
                }
            }
        }

        wr32(
            ped.wrapping_add(PED_FLAGS),
            rd32(ped.wrapping_add(PED_FLAGS)) | 0x80000000,
        );
        if rd8(task.wrapping_add(DETACHED)) != 0 {
            return be2c30_fallback(task, ped);
        }
        let mgr = rd32(ped.wrapping_add(PED_MGR)).wrapping_add(MGR_INNER);
        let mut a: u32 = lf_checker_rt::callee_thiscall!(1, u32, mgr, 1u32, 0x76cu32);
        if a == 0 {
            a = lf_checker_rt::callee_thiscall!(2, u32, mgr, 2u32, 0x76cu32);
            if a == 0 {
                return be2c30_validated(task, ped);
            }
        }
        let ok: u32 = lf_checker_rt::callee_thiscall!(3, u32, a, ped);
        if ok & 0xff == 0 {
            wr32(task.wrapping_add(STAGE), 0);
        } else {
            if rd32(ped.wrapping_add(PED_FLAGS)) & 4 == 0 && rd32(task.wrapping_add(STAGE)) != 2 {
                return 0;
            }
            let e = rd32(task.wrapping_add(STAGE));
            if e != 2 && e != 3 {
                wr32(task.wrapping_add(STAGE), 1);
            }
        }
        wr8(task.wrapping_add(VALIDATED), 1);
        be2c30_validated(task, ped)
    }
});

/// Shared tail of `rw_00be2c30` after the validation preamble: the chance
/// roll, the dispatch gate, the middle path and the fallback.
#[inline(always)]
unsafe fn be2c30_validated(task: u32, ped: u32) -> u32 {
    unsafe {
        const STAGE: u32 = 0x34;
        const TARGET: u32 = 0x3c;
        const PED_FLAGS: u32 = 0x26c;
        const PED_MATRIX: u32 = 0x20;
        const PED_MODEL: u32 = 0x2e;
        const PED_VEHICLE: u32 = 0xb30;
        const PED_STATEB: u32 = 0xa60;
        const PED_LINK: u32 = 0x21c;
        const LINK_WORD: u32 = 0x12c;
        const TGT_KIND: u32 = 0x28;
        const TGT_B0: u32 = 0x218;
        const TGT_B1: u32 = 0x219;
        const KIND_MASK: u32 = 0x3c0;
        const KIND_DISPATCH: u32 = 0xc0;
        const KIND_WAKE: u32 = 0x80;
        const MODEL_TABLE: u32 = 0x1295cd8;
        const MODEL_FLAGS: u32 = 0xee;
        const MODEL_RANK: u32 = 0xef;
        const THRESH_A: u32 = 0x10475d4;
        const THRESH_B: u32 = 0x10475d8;
        const ROLL_SCALE: u32 = 0xfe8684;
        const DIST_LIMIT: u32 = 0xfe8b88;
        const AIM_BLEND: u32 = 0x10475e4;
        const RANK_LIMIT: u32 = 0x10475e8;
        const SWITCH: u32 = 0x10521a0;
        const DISPATCHER: u32 = 0x167e2a0;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u32 {
            unsafe { (a as *const u16).read_unaligned() as u32 }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
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
        #[inline(always)]
        unsafe fn pos_of(t: u32) -> u32 {
            unsafe {
                let up = rd32(t.wrapping_add(0x20));
                if up != 0 {
                    up.wrapping_add(0x30)
                } else {
                    t.wrapping_add(0x10)
                }
            }
        }

        let ceiling = if rd32(task.wrapping_add(STAGE)) == 3 {
            rdf(lf_checker_rt::relocated(THRESH_B))
        } else {
            rdf(lf_checker_rt::relocated(THRESH_A))
        };
        let roll: u32 = lf_checker_rt::callee_cdecl!(4, u32,);
        let f = mul(roll as f32, rdf(lf_checker_rt::relocated(ROLL_SCALE)));
        if f > ceiling {
            return 0;
        }
        // Stage-2 dispatch gate.
        if rd32(task.wrapping_add(STAGE)) == 2 {
            let t = rd32(task.wrapping_add(TARGET));
            if t != 0 && rd32(t.wrapping_add(TGT_KIND)) & KIND_MASK == KIND_DISPATCH {
                let mut prox = false;
                if rd32(ped.wrapping_add(PED_FLAGS)) & 4 == 0 {
                    prox = true;
                } else {
                    let vc = rd32(ped.wrapping_add(PED_VEHICLE));
                    if vc != 0 {
                        let vok: u32 = lf_checker_rt::callee_thiscall!(5, u32, vc, ped);
                        if vok & 0xff != 0 {
                            let idx = rd16(ped.wrapping_add(PED_MODEL));
                            let mdl = rd32(
                                lf_checker_rt::relocated(MODEL_TABLE).wrapping_add(idx.wrapping_mul(4)),
                            );
                            if rd8(mdl.wrapping_add(MODEL_FLAGS)) <= 2
                                && rd8(ped.wrapping_add(PED_STATEB)) == 1
                            {
                                let w = rd32(ped.wrapping_add(PED_LINK));
                                if rd32(w.wrapping_add(LINK_WORD)) != 2 {
                                    let pos = pos_of(t);
                                    let pm = rd32(ped.wrapping_add(PED_MATRIX));
                                    let dy = sub(rdf(pm.wrapping_add(0x34)), rdf(pos.wrapping_add(4)));
                                    let dx = sub(rdf(pm.wrapping_add(0x30)), rdf(pos));
                                    let dz = sub(rdf(pm.wrapping_add(0x38)), rdf(pos.wrapping_add(8)));
                                    let d2 = add(add(mul(dy, dy), mul(dx, dx)), mul(dz, dz));
                                    if rdf(lf_checker_rt::relocated(DIST_LIMIT)) > d2 {
                                        prox = true;
                                    }
                                }
                            }
                        }
                    }
                }
                if prox {
                    let g = rd32(lf_checker_rt::relocated(DISPATCHER));
                    let r: u32 = lf_checker_rt::callee_thiscall!(6, u32, g);
                    if r == 0 {
                        return 0;
                    }
                    return lf_checker_rt::callee_thiscall!(10, u32, r, rd32(task.wrapping_add(TARGET)));
                }
            }
        }
        // Middle path.
        let idle: u32 = lf_checker_rt::callee_thiscall!(11, u32, ped);
        if idle & 0xff == 0 {
            return be2c30_fallback(task, ped);
        }
        let u = rd32(task.wrapping_add(TARGET));
        if u == 0 {
            return be2c30_fallback(task, ped);
        }
        let mut wake = rd8(u.wrapping_add(TGT_B0)) == 0 && rd8(u.wrapping_add(TGT_B1)) != 0;
        if rd32(u.wrapping_add(TGT_KIND)) & KIND_MASK == KIND_WAKE {
            let w: u32 = lf_checker_rt::callee_thiscall!(12, u32, u);
            wake = w & 0xff != 0;
        }
        if !wake {
            return be2c30_fallback(task, ped);
        }
        let near: u32 = lf_checker_rt::callee_cdecl!(13, u32, ped, 0x11u32, 0u32, 0u32);
        if near & 0xff == 0 {
            return be2c30_fallback(task, ped);
        }
        let out: u32;
        if rd32(task.wrapping_add(STAGE)) == 3 {
            let u = rd32(task.wrapping_add(TARGET));
            let dp = pos_of(u);
            let pm = rd32(ped.wrapping_add(PED_MATRIX));
            let dx = sub(rdf(pm.wrapping_add(0x30)), rdf(dp));
            let dy = sub(rdf(pm.wrapping_add(0x34)), rdf(dp.wrapping_add(4)));
            let dz = sub(rdf(pm.wrapping_add(0x38)), rdf(dp.wrapping_add(8)));
            let deltas = [dx, dy, dz];
            let g = rd32(lf_checker_rt::relocated(DISPATCHER));
            let r: u32 = lf_checker_rt::callee_thiscall!(7, u32, g);
            if r == 0 {
                out = 0;
            } else {
                let blend = rdf(lf_checker_rt::relocated(AIM_BLEND));
                out = lf_checker_rt::callee_thiscall!(
                    14, u32, r, 0x1f4u32, 0x3e8u32, u, 0u32,
                    deltas.as_ptr() as u32, blend.to_bits(), 0u32, 0u32
                );
            }
            let _: u32 = lf_checker_rt::callee_thiscall!(15, u32, out, ped);
        } else {
            let g = rd32(lf_checker_rt::relocated(DISPATCHER));
            let r: u32 = lf_checker_rt::callee_thiscall!(8, u32, g);
            if r == 0 {
                out = 0;
            } else {
                let u = rd32(task.wrapping_add(TARGET));
                let dp = pos_of(u);
                out = lf_checker_rt::callee_thiscall!(16, u32, r, 0x1f4u32, 0x3e8u32, dp, u, 0u32);
            }
            if rd8(lf_checker_rt::relocated(SWITCH)) != 0 {
                let idx = rd16(ped.wrapping_add(PED_MODEL));
                let mdl = rd32(
                    lf_checker_rt::relocated(MODEL_TABLE).wrapping_add(idx.wrapping_mul(4)),
                );
                let rank = rd8(mdl.wrapping_add(MODEL_RANK)) as i32;
                let limit = rd32(lf_checker_rt::relocated(RANK_LIMIT)) as i32;
                if rank < limit {
                    wr32(out.wrapping_add(0x28), 5);
                } else {
                    wr32(out.wrapping_add(0x28), 2);
                }
            } else {
                wr32(out.wrapping_add(0x28), 2);
            }
        }
        let g = rd32(lf_checker_rt::relocated(DISPATCHER));
        let r: u32 = lf_checker_rt::callee_thiscall!(9, u32, g);
        if r == 0 {
            return 0;
        }
        lf_checker_rt::callee_thiscall!(17, u32, r, 0x1f4u32, 0x7530u32, out, 0u32)
    }
}

/// Fallback tail of `rw_00be2c30`: cleanup, then finish or reset.
#[inline(always)]
unsafe fn be2c30_fallback(task: u32, ped: u32) -> u32 {
    unsafe {
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        let _: u32 = lf_checker_rt::callee_thiscall!(18, u32, task, ped);
        if rd32(task.wrapping_add(0x38)) != 0 {
            return lf_checker_rt::callee_thiscall!(19, u32, task, 0x119u32, ped);
        }
        let _: u32 = lf_checker_rt::callee_thiscall!(20, u32, ped);
        0
    }
}
