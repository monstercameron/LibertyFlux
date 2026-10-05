// original: 0x00593A50 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_255, player_schema::LeaderboardInfo, 10>::vf14

/// Collect one ranked-episodic-race leaderboard row (id 0x15a, vtable slot 14).
///
/// `this` is the leaderboard-info object (vtable pointer at `+0`, the
/// per-trial selector at slot `+0x2c`, the per-entry index query at slot
/// `+0x30`). `cursor` and `bound` form an unsigned cursor and its limit
/// (`bound` plus `cursor`, wrapping); `out_row` takes one 8-byte row,
/// `out_mask` an 8-byte single-bit mask, `out_flag` one status byte; `ctx`
/// is an opaque context handed to every helper call. `LEADERBOARD_ID`
/// (0x15a) is this instantiation's query id.
///
/// Behaviour: zero the three outputs; ask the selector for `sel`; issue the
/// ranked query for this id into a 12-byte frame buffer whose third word
/// arrives holding the entry table, and return that call's raw answer when
/// its low byte is zero. Otherwise run 19 entries (`i` = 0..18), stopping
/// early once the status flag clears: fetch entry index `idx`; ask the
/// filter and skip the entry when it answers nonzero; classify the table
/// object (`step` 8 for codes 1, 2, 3 and 5, else 0). When `sel == i`, fetch
/// the row and, when it and its measured size (at most 8) check out, copy
/// its 8 bytes at `+4` to `out_row` and set the flag; the flag is then
/// stored to `out_flag` either way. For any other entry, advance the cursor
/// by `step`, clearing the flag when it passes the limit, else asking the
/// commit helper and, on success, writing bit `i` as a 64-bit mask to
/// `out_mask` and setting the flag. Returns the flag in the low byte.
///
/// The mask formerly split at bit 32 and bit 64 is written by the same
/// compare-and-move sequence as the original; both high halves are dead
/// here (`i` never reaches 32 in 19 entries) but kept for exactness.
///
/// Calling convention: thiscall with six stack words; the callee pops 0x18.
/// The original keeps the running cursor and `sel` in its own incoming
/// argument slots; a Rust rewrite cannot store there, so the checker
/// contract leaves the stack-arg stores uncompared (everything else,
/// including both values through call arguments and branches, is compared).
lf_checker_rt::export!(thiscall, rw_00593A50(this: u32, cursor: u32, out_row: u32, out_mask: u32, out_flag: u32, ctx: u32, bound: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x15a;
        const VTABLE_SELECTOR: u32 = 0x2c;
        const VTABLE_INDEX: u32 = 0x30;
        const ITERATIONS: u32 = 19;
        const ROW_LEN_MAX: u32 = 8;
        const STEP_HIT: u32 = 8;
        const Q_FILTER: u32 = 4;
        const Q_CLASSIFY: u32 = 5;
        const Q_FETCH: u32 = 6;
        const Q_MEASURE: u32 = 7;
        const Q_COMMIT: u32 = 8;
        const Q_QUERY: u32 = 2;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }

        // Zero the three caller outputs.
        wr32(out_mask, 0);
        wr32(out_mask + 4, 0);
        (out_flag as *mut u8).write(0);

        let limit = bound.wrapping_add(cursor);
        let vtable = rd32(this);
        let selector: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(rd32(vtable + VTABLE_SELECTOR) as usize);
        let sel = selector(this);

        // Ranked query into a frame buffer; the third word arrives holding
        // the entry table. The stub writes it on both sides alike.
        let mut query = [0u32; 3];
        let gate: u32 = lf_checker_rt::callee_fastcall!(
            Q_QUERY, u32, LEADERBOARD_ID, query.as_mut_ptr() as u32
        );
        if gate & 0xff == 0 {
            return gate;
        }
        let table = query[2];

        let indexer: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(rd32(vtable + VTABLE_INDEX) as usize);
        let mut cursor = cursor;
        let mut flag: u8 = 1;
        let mut i: u32 = 0;
        while i < ITERATIONS {
            if flag == 0 {
                break;
            }
            let idx: u32 = indexer(this, i);
            let skip: u32 = lf_checker_rt::callee_thiscall!(Q_FILTER, u32, ctx, idx);
            if skip & 0xff == 0 {
                let obj = rd32(table.wrapping_add(idx.wrapping_mul(4)));
                let code: u32 = lf_checker_rt::callee_thiscall!(Q_CLASSIFY, u32, obj);
                // The original's switch on (code - 1): cases 0, 1, 2 and 4
                // set step 8, case 3 and the default leave it 0.
                let mut step: u32 = 0;
                if code != 0xffff_ffff {
                    let c = code.wrapping_sub(1);
                    if c < 5 && c != 3 {
                        step = STEP_HIT;
                    }
                }
                if sel == i {
                    flag = 0;
                    let row: u32 = lf_checker_rt::callee_thiscall!(Q_FETCH, u32, ctx, idx);
                    if row != 0 {
                        let size: u32 = lf_checker_rt::callee_thiscall!(Q_MEASURE, u32, row);
                        if size <= ROW_LEN_MAX {
                            let v = ((row + 4) as *const u64).read_unaligned();
                            (out_row as *mut u64).write_unaligned(v);
                            flag = 1;
                        }
                    }
                    (out_flag as *mut u8).write(flag);
                } else {
                    cursor = cursor.wrapping_add(step);
                    if cursor > limit {
                        flag = 0;
                    } else {
                        let ok: u32 = lf_checker_rt::callee_thiscall!(
                            Q_COMMIT, u32, ctx, idx, cursor, step
                        );
                        if ok & 0xff == 0 {
                            flag = 0;
                        } else {
                            // Bit-test-and-set of bit i into a 64-bit mask,
                            // in the original's compare-and-move form.
                            let mut lo: u32 = 0;
                            lo |= 1u32.wrapping_shl(i);
                            let mut hi: u32 = 0;
                            if i >= 0x20 {
                                hi = lo;
                            }
                            lo ^= hi;
                            if i >= 0x40 {
                                hi = lo;
                            }
                            wr32(out_mask, lo);
                            wr32(out_mask + 4, hi);
                            flag = 1;
                        }
                    }
                }
            }
            i += 1;
        }
        flag as u32
    }
});
