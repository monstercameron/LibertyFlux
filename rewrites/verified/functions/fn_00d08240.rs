// original: 0x00D08240 CTaskComplexNewUseCover::vf20 (symbols)
//
// Task tick for the "use cover" complex task: refresh flags on the ped,
// make sure its current subtask is accepted, advance a timer, and pick a
// cover-related cost value through several helper calls.
//
// Arguments (thiscall): `this` is the task, `ped` the ped it runs on. Entry
// sets bit 0x400 at ped+0x2a0 always; when the dword at ped+0xd68 is set it
// also sets bit 0x800 at ped+0x26c and skips the subtask check, otherwise
// the subtask object at this+8 must accept (flag bit 1 already set, or its
// virtual slot 0x14 answering true for probe 1 or 2, which then sets flag
// bit 2); an accepted subtask here ends the tick through helper 1 with
// result 0. The main path adds a global tick delta to the float at this+0x40,
// runs helper 2, and resolves a candidate through helper 3, which must be
// non-null, of kind 3 (dword at +0x1c) and not in state 7 (this+0x38).
// Helper 5's answer selects one of two cost slots (+0x9c/+0x94) off helper
// 4's table; the second branch additionally gates on helpers 6 and 7 and may
// upgrade the cost to slot +0x98. When a global mode word equals 2 and two
// further gates (helper 8, virtual slot 0x128 on the ped) pass, the cost is
// scaled by the float at [ped+0x228]+0x5ac if that exceeds a global limit,
// then added to a global base and stored at candidate+0x20. The tail asks
// helper 9 for a replacement subtask: a non-null answer that the subtask
// check accepts is returned, otherwise the current subtask is returned
// unless this+0x38 says 0x16, in which case the check runs once more and an
// accept ends through helper 1 with 0. Float operation order is the
// original's.
lf_checker_rt::export!(thiscall, rw_00D08240(this: u32, ped: u32) -> u32 {
    unsafe {
        const SUB_AT: u32 = 0x08;
        const SUB_FLAG: u32 = 0x0c;
        const SUB_ACCEPT_SLOT: u32 = 0x14;
        const STATE_AT: u32 = 0x38;
        const TIMER_AT: u32 = 0x40;
        const PED_BUSY: u32 = 0x2a0;
        const PED_HAS_COVER: u32 = 0xd68;
        const PED_IN_COVER: u32 = 0x26c;
        const PED_AUX: u32 = 0x228;
        const PED_INFO: u32 = 0x2b0;
        const PED_COST_SLOT: u32 = 0x128;
        const CAND_KIND: u32 = 0x1c;
        const CAND_KEY: u32 = 0x18;
        const CAND_COST: u32 = 0x20;
        const KIND_WANTED: u32 = 3;
        const STATE_SKIP: u32 = 7;
        const STATE_RETRY: u32 = 0x16;
        const TABLE_W28: u32 = 0x28;
        const COST_B: u32 = 0x94;
        const COST_C: u32 = 0x98;
        const COST_A: u32 = 0x9c;
        const AUX_SCALE: u32 = 0x5ac;
        const G_TICK: u32 = 0x11735bc;
        const G_BASE: u32 = 0x11735b4;
        const G_MODE: u32 = 0x11d6fd4;
        const G_LIMIT: u32 = 0xfe8874;
        const MODE_SCALED: u32 = 2;
        const C_BASE: u32 = 1;
        const C_ADVANCE: u32 = 2;
        const C_CANDIDATE: u32 = 3;
        const C_TABLE: u32 = 4;
        const C_SELECT: u32 = 5;
        const C_GATE: u32 = 6;
        const C_SCORE: u32 = 7;
        const C_MODE: u32 = 8;
        const C_REPLACE: u32 = 9;
        const C_SUB_ACCEPT: u32 = 10;
        const C_PED_COST: u32 = 11;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
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
            unsafe { (a as *mut u32).write_unaligned(v.to_bits()) }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        /// Truncating float-to-int exactly like cvttss2si, including the
        /// indefinite (i32::MIN) result for NaN and out-of-range inputs,
        /// where a plain `as` cast would saturate instead.
        #[inline(always)]
        fn cvttss2si(x: f32) -> i32 {
            if x.is_nan() || x >= 2147483648.0 || x < -2147483648.0 {
                i32::MIN
            } else {
                x as i32
            }
        }
        /// Virtual slot 0x14 on the subtask object (thiscall, 3 stack args).
        unsafe fn sub_accept(sub: u32, ped: u32, probe: u32) -> u32 {
            unsafe {
                let slot = rd32(rd32(sub).wrapping_add(SUB_ACCEPT_SLOT));
                let f: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
                    core::mem::transmute(slot as usize);
                f(sub, ped, probe, 0)
            }
        }
        /// The repeated "current subtask accepts" check: flag bit 1 set, or
        /// probe 1 or 2 answered (setting flag bit 2 on success).
        unsafe fn check_sub(this: u32, ped: u32) -> bool {
            unsafe {
                let mut sub = rd32(this.wrapping_add(SUB_AT));
                if rd8(sub.wrapping_add(SUB_FLAG)) & 1 != 0 {
                    return true;
                }
                if sub_accept(sub, ped, 1) as u8 != 0 {
                    wr8(sub.wrapping_add(SUB_FLAG), rd8(sub.wrapping_add(SUB_FLAG)) | 2);
                    return true;
                }
                sub = rd32(this.wrapping_add(SUB_AT));
                if rd8(sub.wrapping_add(SUB_FLAG)) & 1 != 0 {
                    return true;
                }
                if sub_accept(sub, ped, 2) as u8 != 0 {
                    wr8(sub.wrapping_add(SUB_FLAG), rd8(sub.wrapping_add(SUB_FLAG)) | 2);
                    return true;
                }
                false
            }
        }

        wr32(ped.wrapping_add(PED_BUSY), rd32(ped.wrapping_add(PED_BUSY)) | 0x400);
        if rd32(ped.wrapping_add(PED_HAS_COVER)) == 0 {
            if check_sub(this, ped) {
                let _: u32 = lf_checker_rt::callee_thiscall!(C_BASE, u32, this);
                return 0;
            }
        } else {
            wr32(
                ped.wrapping_add(PED_IN_COVER),
                rd32(ped.wrapping_add(PED_IN_COVER)) | 0x800,
            );
        }
        // Main path.
        let tick = f32::from_bits(rd32(lf_checker_rt::relocated(G_TICK)));
        wrf(this.wrapping_add(TIMER_AT), add(rdf(this.wrapping_add(TIMER_AT)), tick));
        let _: u32 = lf_checker_rt::callee_thiscall!(C_ADVANCE, u32, this, ped);
        let cand: u32 =
            lf_checker_rt::callee_thiscall!(C_CANDIDATE, u32, ped.wrapping_add(PED_INFO));
        let mut have_cand = cand != 0;
        if have_cand && rd32(cand.wrapping_add(CAND_KIND)) != KIND_WANTED {
            have_cand = false;
        }
        if have_cand && rd32(this.wrapping_add(STATE_AT)) == STATE_SKIP {
            have_cand = false;
        }
        if have_cand {
            let key = rd32(cand.wrapping_add(CAND_KEY));
            let _: u32 = lf_checker_rt::callee_cdecl!(C_TABLE, u32, key);
            let sel: u32 = lf_checker_rt::callee_thiscall!(C_SELECT, u32, ped);
            let mut cost: u32;
            if (sel as u8) != 0 {
                let t: u32 = lf_checker_rt::callee_cdecl!(C_TABLE, u32, key);
                cost = rd32(t.wrapping_add(COST_A));
            } else {
                let t: u32 = lf_checker_rt::callee_cdecl!(C_TABLE, u32, key);
                cost = rd32(t.wrapping_add(COST_B));
                let g: u32 = lf_checker_rt::callee_thiscall!(C_GATE, u32, ped);
                if (g as u8) != 0 {
                    let _: u32 = lf_checker_rt::callee_cdecl!(C_TABLE, u32, key);
                    let t2: u32 = lf_checker_rt::callee_cdecl!(C_TABLE, u32, key);
                    let s: u32 = lf_checker_rt::callee_cdecl!(
                        C_SCORE, u32, rd32(t2.wrapping_add(TABLE_W28)), 0xc8, key, key
                    );
                    if s != 0 {
                        let t3: u32 = lf_checker_rt::callee_cdecl!(C_TABLE, u32, key);
                        cost = rd32(t3.wrapping_add(COST_C));
                    }
                }
            }
            if rd32(lf_checker_rt::relocated(G_MODE)) == MODE_SCALED {
                let m: u32 = lf_checker_rt::callee_cdecl!(C_MODE, u32,);
                if (m as u8) != 0 {
                    let slot = rd32(rd32(ped).wrapping_add(PED_COST_SLOT));
                    let f: extern "thiscall" fn(u32) -> u32 =
                        core::mem::transmute(slot as usize);
                    let ok: u32 = f(ped);
                    if (ok as u8) != 0 {
                        let aux = rd32(ped.wrapping_add(PED_AUX));
                        if aux != 0 {
                            let limit =
                                f32::from_bits(rd32(lf_checker_rt::relocated(G_LIMIT)));
                            let x = rdf(aux.wrapping_add(AUX_SCALE));
                            if x > limit {
                                cost = cvttss2si(mul(cost as i32 as f32, x)) as u32;
                            }
                        }
                    }
                }
            }
            let base = rd32(lf_checker_rt::relocated(G_BASE));
            wr32(cand.wrapping_add(CAND_COST), base.wrapping_add(cost));
        }
        // Tail: replacement subtask or current one.
        let rep: u32 = lf_checker_rt::callee_thiscall!(C_REPLACE, u32, this, ped);
        if rep != 0 && check_sub(this, ped) {
            return rep;
        }
        if rd32(this.wrapping_add(STATE_AT)) != STATE_RETRY {
            return rd32(this.wrapping_add(SUB_AT));
        }
        if check_sub(this, ped) {
            let _: u32 = lf_checker_rt::callee_thiscall!(C_BASE, u32, this);
            return 0;
        }
        rd32(this.wrapping_add(SUB_AT))
    }
});
