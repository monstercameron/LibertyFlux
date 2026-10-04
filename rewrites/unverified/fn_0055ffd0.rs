// original: 0x0055FFD0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_66, player_schema::LeaderboardInfo, 10>::vf14

/// Resolve one row of ranked episodic-race leaderboard 66.
///
/// `this` is the leaderboard-info object. Two virtual calls drive the
/// resolution: slot `VT_WANTED` (`+0x2c`) returns the wanted iteration, and
/// slot `VT_TOKEN` (`+0x30`) maps each iteration counter to a row token.
/// `aux` passes through as context to every helper call. `copy_dst`,
/// `mask_dst` and `flag_dst` receive the outputs: an 8-byte row fragment, an
/// 8-byte one-hot iteration mask, and one status byte. `acc` seeds a running
/// total capped by `acc + limit_add` (wrapping).
///
/// The probe helper for id `LEADERBOARD_ID` fills a three-word frame struct
/// whose third word points at the row-cell array; each of at most 19
/// iterations classifies one cell (only classes 1, 2, 3 and 5 advance the
/// total, by `STRIDE`), then either fetches the wanted row -- copying 8
/// bytes from `ROW_DATA` past the returned pointer when the row size is at
/// most `SIZE_LIMIT` -- or commits the running total and records the
/// iteration bit. Returns nonzero on success; only AL is significant.
/// The two spills into the incoming argument area are dead scratch (the
/// callee pops its arguments) and are not reproduced. Original is thiscall
/// with six stack words.
lf_checker_rt::export!(thiscall, rw_0055FFD0(this: u32, acc: u32, copy_dst: u32, mask_dst: u32, flag_dst: u32, aux: u32, limit_add: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x10D;
        const VT_WANTED: u32 = 0x2c;
        const VT_TOKEN: u32 = 0x30;
        const ROW_DATA: u32 = 4;
        const LOOP_BOUND: u32 = 0x13;
        const STRIDE: u32 = 8;
        const SIZE_LIMIT: u32 = 8;
        const CALLEE_PROBE: u32 = 2;
        const CALLEE_GATE: u32 = 4;
        const CALLEE_CLASSIFY: u32 = 5;
        const CALLEE_FETCH: u32 = 6;
        const CALLEE_SIZE: u32 = 7;
        const CALLEE_COMMIT: u32 = 8;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }

        wr32(mask_dst, 0);
        wr32(mask_dst.wrapping_add(4), 0);
        wr8(flag_dst, 0);
        let limit = limit_add.wrapping_add(acc);
        let wanted: u32 = {
            let vt = rd32(this);
            let f: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(rd32(vt.wrapping_add(VT_WANTED)) as usize);
            f(this)
        };
        let mut probe = [0u32; 3];
        let probe_ok: u8 = lf_checker_rt::callee_fastcall!(
            CALLEE_PROBE, u8, LEADERBOARD_ID, probe.as_mut_ptr() as u32);
        if probe_ok == 0 {
            return 0;
        }
        let array = probe[2];
        let mut run = acc;
        let mut ok: u8 = 1;
        let mut iter: u32 = 0;
        loop {
            if ok == 0 {
                break;
            }
            let tok: u32 = {
                let vt = rd32(this);
                let f: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(rd32(vt.wrapping_add(VT_TOKEN)) as usize);
                f(this, iter)
            };
            let gate: u8 = lf_checker_rt::callee_thiscall!(CALLEE_GATE, u8, aux, tok);
            if gate == 0 {
                let cell = rd32(array.wrapping_add(tok.wrapping_mul(4)));
                let class: u32 = lf_checker_rt::callee_thiscall!(CALLEE_CLASSIFY, u32, cell);
                // Jump-table switch over (class - 1): only indexes 0, 1, 2
                // and 4 take the stride arm; every other value keeps 0.
                let mut step: u32 = 0;
                if matches!(class, 1 | 2 | 3 | 5) {
                    step = STRIDE;
                }
                if wanted == iter {
                    ok = 0;
                    let row: u32 = lf_checker_rt::callee_thiscall!(CALLEE_FETCH, u32, aux, tok);
                    if row != 0 {
                        let size: u32 = lf_checker_rt::callee_thiscall!(CALLEE_SIZE, u32, row);
                        if size <= SIZE_LIMIT {
                            let lo = rd32(row.wrapping_add(ROW_DATA));
                            let hi = rd32(row.wrapping_add(ROW_DATA + 4));
                            wr32(copy_dst, lo);
                            wr32(copy_dst.wrapping_add(4), hi);
                            ok = 1;
                        }
                    }
                    wr8(flag_dst, ok);
                } else {
                    // The commit call takes the pre-add total: the original
                    // pushes it from its spill slot, which is refreshed only
                    // after the call, while the limit check uses the new sum.
                    let prev = run;
                    run = run.wrapping_add(step);
                    if run > limit {
                        ok = 0;
                    } else {
                        let done: u8 = lf_checker_rt::callee_thiscall!(
                            CALLEE_COMMIT, u8, aux, tok, prev, step);
                        if done == 0 {
                            ok = 0;
                        } else {
                            ok = 1;
                            wr32(mask_dst, 1u32 << iter);
                            wr32(mask_dst.wrapping_add(4), 0);
                        }
                    }
                }
            }
            iter += 1;
            if iter >= LOOP_BOUND {
                break;
            }
        }
        ok as u32
    }
});
