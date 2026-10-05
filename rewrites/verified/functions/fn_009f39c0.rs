// original: 0x009F39C0 ped_task_resolve_chain (proposed)

/// Resolve a ped's pending task through up to three staged attempts.
///
/// `this` is the ped (thiscall, no stack arguments). Two virtual gate calls
/// (slot `+VT_GATE`) open the function: a null answer, or an answer whose
/// flag byte at `+GATE_FLAG` lacks `GATE_BIT`, returns at once.
///
/// The marker bytes then decide: a set `+MARK_A` or a clear `+MARK_B` goes
/// straight to the tail; a null queue head (`+QUEUE`) returns; a variant id
/// (`+VARIANT`) of `SP_A`, `SP_B` or `SP_C` runs the special block.
///
/// Otherwise up to three attempt blocks run in order. Block one probes the
/// queue's solver (callee A with the queue head plus `SOLVER_OFF`) and, on
/// a positive answer, runs the attempt; a non-positive answer, or the freak
/// case of a queue head of exactly `-SOLVER_OFF`, falls into block two.
/// Block two compares the queue's stamp (`+STAMP`) against the global tick:
/// above it runs the attempt, otherwise block three runs it. Each attempt
/// shares one shape: callee B resolves the variant argument, a global
/// one-time-init bit (`INIT_BIT` of the mask word, with its own slot and
/// name) is honored (running callee C on first use), callee D combines the
/// two, callee E tests the combination, and callee F commits it. A zero from
/// callee D instead runs the fail block (callee G, then callee H with the
/// weight `FAIL_WEIGHT` on success).
///
/// The special block and the tail share the two-step shape: callee I tries
/// the (variant, argument) pair and returns on success, else callee J
/// re-resolves it and callee K commits with the weight `COMMIT_WEIGHT`
/// unless it fails.
///
/// Returns nothing; the original is void.
lf_checker_rt::export!(thiscall, rw_009F39C0(this: u32) -> u32 {
    unsafe {
        const VT_GATE: u32 = 0xd0;
        const GATE_FLAG: u32 = 0x72;
        const GATE_BIT: u8 = 8;
        const MARK_A: u32 = 0x218;
        const MARK_B: u32 = 0x219;
        const QUEUE: u32 = 0x228;
        const VARIANT: u32 = 0xba0;
        const SP_A: u32 = 0x1ba;
        const SP_B: u32 = 0x1bb;
        const SP_C: u32 = 0x1bc;
        const ARG: u32 = 0xb9c;
        const AUX: u32 = 0x78;
        const SOLVER_OFF: u32 = 0x70;
        const STAMP: u32 = 0x568;
        const TICK_GLOBAL: u32 = 0x0117_35b4;
        const INIT_MASK: u32 = 0x012b_60dc;
        const FAIL_WEIGHT: u32 = 0xc100_0000;
        const COMMIT_WEIGHT: u32 = 0x41f0_0000;
        const CAL_SOLVER: u32 = 1;
        const CAL_RESOLVE: u32 = 2;
        const CAL_INIT: u32 = 3;
        const CAL_COMBINE: u32 = 4;
        const CAL_TEST: u32 = 5;
        const CAL_COMMIT: u32 = 6;
        const CAL_FAILQ: u32 = 7;
        const CAL_FAILW: u32 = 8;
        const CAL_TRY: u32 = 9;
        const CAL_RERESOLVE: u32 = 10;
        const CAL_TAILCOMMIT: u32 = 11;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn rd_global(file_va: u32) -> u32 {
            unsafe { lf_checker_rt::global::<u32>(file_va).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr_global(file_va: u32, v: u32) {
            unsafe { lf_checker_rt::global::<u32>(file_va).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn gate(this: u32) -> u32 {
            unsafe {
                let slot = rd32(rd32(this).wrapping_add(VT_GATE));
                let f: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(slot as usize);
                f(this)
            }
        }
        /// One attempt block: resolve, honor the one-time init, combine,
        /// test, commit or fail. `bit`/`slot`/`name` select the block's
        /// init row.
        #[inline(always)]
        unsafe fn attempt(this: u32, variant_arg: u32, bit: u32, slot: u32, name: u32) {
            unsafe {
                let aux = rd32(this.wrapping_add(AUX));
                let resolved =
                    lf_checker_rt::callee_cdecl!(CAL_RESOLVE, u32, variant_arg);
                let mask = rd_global(INIT_MASK);
                let token = if mask & bit == 0 {
                    wr_global(INIT_MASK, mask | bit);
                    // The name is pushed from a relocated immediate, so it
                    // must be relocated too, not used as a raw file VA.
                    let v = lf_checker_rt::callee_cdecl!(
                        CAL_INIT,
                        u32,
                        lf_checker_rt::relocated(name),
                        0
                    );
                    wr_global(slot, v);
                    v
                } else {
                    rd_global(slot)
                };
                let combo = lf_checker_rt::callee_cdecl!(CAL_COMBINE, u32, resolved, token);
                if combo == 0 {
                    let h = lf_checker_rt::callee_thiscall!(CAL_FAILQ, u32, aux, 0x40000, 1);
                    if h != 0 {
                        lf_checker_rt::callee_thiscall!(CAL_FAILW, u32, h, FAIL_WEIGHT);
                    }
                    return;
                }
                if lf_checker_rt::callee_thiscall!(CAL_TEST, u32, aux, combo) != 0 {
                    return;
                }
                lf_checker_rt::callee_thiscall!(
                    CAL_COMMIT,
                    u32,
                    aux,
                    resolved,
                    token,
                    0x4020,
                    5,
                    COMMIT_WEIGHT
                );
            }
        }
        /// The shared two-step tail: try, re-resolve, commit.
        #[inline(always)]
        unsafe fn tail(this: u32) {
            unsafe {
                let aux = rd32(this.wrapping_add(AUX));
                let variant = rd32(this.wrapping_add(VARIANT));
                let arg = rd32(this.wrapping_add(ARG));
                if lf_checker_rt::callee_thiscall!(CAL_TRY, u32, aux, arg, variant) != 0 {
                    return;
                }
                if lf_checker_rt::callee_cdecl!(CAL_RERESOLVE, u32, arg, variant) == 0 {
                    return;
                }
                lf_checker_rt::callee_thiscall!(
                    CAL_TAILCOMMIT,
                    u32,
                    aux,
                    arg,
                    variant,
                    COMMIT_WEIGHT,
                    0xffff_ffff
                );
            }
        }

        if gate(this) == 0 {
            return 0;
        }
        let g = gate(this);
        if rd8(g.wrapping_add(GATE_FLAG)) & GATE_BIT == 0 {
            return 0;
        }
        let mark_a = rd8(this.wrapping_add(MARK_A));
        let mark_b = rd8(this.wrapping_add(MARK_B));
        let queue = rd32(this.wrapping_add(QUEUE));
        let variant = rd32(this.wrapping_add(VARIANT));
        if mark_a == 0 {
            if mark_b == 0 {
                tail(this);
                return 0;
            }
            if queue == 0 {
                return 0;
            }
            if variant == SP_A || variant == SP_B || variant == SP_C {
                tail(this);
                return 0;
            }
            let solver_this = queue.wrapping_add(SOLVER_OFF);
            if solver_this != 0
                && (lf_checker_rt::callee_thiscall!(CAL_SOLVER, u32, solver_this) as i32) > 0
            {
                attempt(this, rd32(this.wrapping_add(ARG)), 1, 0x012b_60d8, 0x00e9_83e0);
                return 0;
            }
            let stamp = rd32(queue.wrapping_add(STAMP));
            if stamp > rd_global(TICK_GLOBAL) {
                attempt(this, rd32(this.wrapping_add(ARG)), 2, 0x012b_60e0, 0x00e9_83f0);
                return 0;
            }
            attempt(this, rd32(this.wrapping_add(ARG)), 4, 0x012b_60e4, 0x00e9_8418);
            return 0;
        }
        tail(this);
        0
    }
});
