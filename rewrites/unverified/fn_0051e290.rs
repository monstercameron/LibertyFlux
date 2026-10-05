// original: 0x0051e290 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Race28NoHolds, player_schema::LeaderboardInfo, 10>::vf14

/// Leaderboard_Ranked_Race28NoHolds: seven-round leaderboard column fetch for one ranked race table.
///
/// `this` is the leaderboard-info object (vtable pointer at `+0`). The six
/// stack arguments are: `depth` (running-total seed), `out_row` (8-byte slot
/// for the matched row's payload at `row+4`), `out_mask` (8-byte slot for a
/// one-hot round mask), `status` (status object whose byte at `+0` is cleared
/// here and set to the result on the match path), `filter` (context object
/// handed to two callees, never dereferenced here), `extra` (added to `depth`
/// to form the running-total limit).
///
/// The vtable slot at `+0x2c` (no arguments) yields the match round; the
/// fetch callee (fastcall: ECX = leaderboard id `0x3e`, EDX =
/// 12-byte scratch) must answer nonzero, else the function returns 0 at once,
/// and leaves the class table pointer at scratch word 2. Each of the 7
/// rounds, unless an earlier round already failed: the vtable slot at `+0x30`
/// maps the round index to a key; the skip callee (ECX = `filter`, key) ends
/// the round when nonzero; otherwise the class callee (ECX =
/// `table[key]`, read as `table + key*4`) classifies the key and a switch on
/// its answer (1, 2, 3 or 5) steps the running total by 8, else by 0. When
/// the match round equals the round index, the row callee (ECX = `filter`,
/// key) supplies a row object; if it is non-null and the width callee
/// (ECX = row) answers 8 or less, the 8 bytes at `row+4` are copied to
/// `out_row` and the round succeeds, and `status+0` is set to the round
/// outcome either way. On any other round the running total grows by the
/// step and must not pass the limit; the write callee (ECX = `filter`, key,
/// previous total, step) then stores the one-hot mask `1 << round` (low word;
/// the general form also folds indices past 31 into the high word, which the
/// 7-round bound never reaches) into `out_mask` when it answers nonzero,
/// else the round fails. A failed round ends the loop; the return value is
/// the final 1/0 in AL (upper bits of EAX are caller leftovers).
///
/// Original: 0x0051e290 (thiscall, ECX = this, six stack words, callee pops
/// 0x18). No floating-point operations; the 8-byte row copy is two word
/// moves. No global data is read.
lf_checker_rt::export!(thiscall, rw_0051e290(this: u32, depth: u32, out_row: u32, out_mask: u32, status: u32, filter: u32, extra: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x3e;
        const VT_MATCH_ROUND: u32 = 0x2c;
        const VT_KEY_OF_ROUND: u32 = 0x30;
        const ROUNDS: u32 = 7;
        const STEP_FULL: u32 = 8;
        const WIDTH_LIMIT: u32 = 8;
        const CAL_FETCH: u32 = 2;
        const CAL_SKIP: u32 = 4;
        const CAL_CLASS: u32 = 5;
        const CAL_ROW: u32 = 6;
        const CAL_WIDTH: u32 = 7;
        const CAL_WRITE: u32 = 8;

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

        wr32(out_mask, 0);
        wr32(out_mask.wrapping_add(4), 0);
        wr8(status, 0);
        let limit = extra.wrapping_add(depth);
        let mut running = depth;
        let mut result: u8 = 1;
        let vt = rd32(this);
        let match_round_of: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(rd32(vt.wrapping_add(VT_MATCH_ROUND)) as usize);
        let match_round = match_round_of(this);
        let mut probe = [0u32; 3];
        let fetch_ok: u32 = lf_checker_rt::callee_fastcall!(
            CAL_FETCH, u32, LEADERBOARD_ID, probe.as_mut_ptr() as u32);
        if (fetch_ok & 0xFF) == 0 {
            return 0;
        }
        let table = probe[2];
        let key_of_round: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(rd32(vt.wrapping_add(VT_KEY_OF_ROUND)) as usize);
        let mut round = 0u32;
        while round < ROUNDS {
            if result == 0 {
                break;
            }
            let key = key_of_round(this, round);
            let skip: u32 = lf_checker_rt::callee_thiscall!(CAL_SKIP, u32, filter, key);
            if (skip & 0xFF) != 0 {
                round += 1;
                continue;
            }
            let class: u32 = lf_checker_rt::callee_thiscall!(
                CAL_CLASS, u32, rd32(table.wrapping_add(key.wrapping_mul(4))));
            let step = match class {
                1 | 2 | 3 | 5 => STEP_FULL,
                _ => 0,
            };
            if match_round == round {
                let row: u32 = lf_checker_rt::callee_thiscall!(CAL_ROW, u32, filter, key);
                result = 0;
                if row != 0 {
                    let width: u32 = lf_checker_rt::callee_thiscall!(CAL_WIDTH, u32, row);
                    if width <= WIDTH_LIMIT {
                        wr32(out_row, rd32(row.wrapping_add(4)));
                        wr32(out_row.wrapping_add(4), rd32(row.wrapping_add(8)));
                        result = 1;
                    }
                }
                wr8(status, result);
            } else {
                let prev = running;
                running = running.wrapping_add(step);
                if running > limit {
                    result = 0;
                } else {
                    let wrote: u32 = lf_checker_rt::callee_thiscall!(
                        CAL_WRITE, u32, filter, key, prev, step);
                    if (wrote & 0xFF) == 0 {
                        result = 0;
                    } else {
                        let mut lo: u32 = 0;
                        lo |= 1u32 << (round & 31);
                        let mut hi: u32 = 0;
                        if round >= 0x20 {
                            hi = lo;
                        }
                        lo ^= hi;
                        if round >= 0x40 {
                            hi = lo;
                        }
                        wr32(out_mask, lo);
                        wr32(out_mask.wrapping_add(4), hi);
                        result = 1;
                    }
                }
            }
            round += 1;
        }
        result as u32
    }
});
