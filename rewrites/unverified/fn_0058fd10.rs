// original: 0x0058FD10 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_241, player_schema::LeaderboardInfo, 10>::vf14

/// leaderboard_episodic_race_241_vf14 (proposed): scan one ranked-episodic-race leaderboard's entries.
///
/// `thiscall`: `this` in ECX plus six stack words. `count` is the running
/// entry count, `out_pair` takes an 8-byte value copied from the matching
/// entry, `out_mask` takes a 64-bit bitmask with one bit set, `out_flag`
/// takes a success byte, `ctx` is an opaque object handed to the callees,
/// and `budget` caps how far `count` may grow. Returns nonzero on success.
///
/// Behaviour: zero the three outputs, then ask the object's own vtable slot
/// `0x2c` for the expected entry index and the leaderboard lookup (fastcall
/// with the constant id `LEADERBOARD_ID` and a pointer to a 12-byte local)
/// for the entry table. A false lookup ends the call with 0. Otherwise run
/// at most 19 rounds: stop when the flag byte is clear; each round asks
/// vtable slot `0x30` for the round's entry index and the gate callee
/// whether to skip it. A kept round reads the entry pointer from the table,
/// asks the rank callee for its rank, and advances `count` by 8 for ranks
/// 1, 2, 3 and 5 (0 otherwise). The round matching the expected index
/// instead resolves the entry through the find callee and, when its kind is
/// 8 or less, copies 8 bytes from `found+4` to `out_pair`; it always stores
/// the flag byte. Other kept rounds fail when `count` passes
/// `count+budget`, else report to the store callee and set bit `round` of
/// the 64-bit mask at `out_mask` (rounds stay below 19, so the high word is
/// always 0).
///
/// The original also reuses two of its incoming argument slots as scratch
/// (the running count and the expected index). Those stores are dead on
/// return -- no caller reads its outgoing arguments -- so the contract
/// compares everything except the stack page (see `narrowed`).
lf_checker_rt::export!(thiscall, rw_0058fd10(this: u32, count: u32, out_pair: u32, out_mask: u32, out_flag: u32, ctx: u32, budget: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x1c9;
        const VT_EXPECTED: u32 = 0x2c;
        const VT_ENTRY: u32 = 0x30;
        const ROUNDS: u32 = 0x13;
        const STEP_HIT: u32 = 8;
        const KIND_LIMIT: u32 = 8;
        const CAL_LOOKUP: u32 = 2;
        const CAL_GATE: u32 = 4;
        const CAL_RANK: u32 = 5;
        const CAL_FIND: u32 = 6;
        const CAL_KIND: u32 = 7;
        const CAL_STORE: u32 = 8;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }

        wr32(out_mask, 0);
        wr32(out_mask.wrapping_add(4), 0);
        (out_flag as *mut u8).write(0);
        let limit = budget.wrapping_add(count);

        let vt = rd32(this);
        let expected_fn: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(rd32(vt.wrapping_add(VT_EXPECTED)) as usize);
        let expected = expected_fn(this);

        let mut info = [0u32; 3];
        info[2] = 0;
        let ok: u32 = lf_checker_rt::callee_fastcall!(
            CAL_LOOKUP, u32, LEADERBOARD_ID, info.as_mut_ptr() as u32);
        if (ok & 0xFF) == 0 {
            return 0;
        }

        let mut count = count;
        let mut alive: u8 = 1;
        let mut round: u32 = 0;
        while round < ROUNDS {
            if alive == 0 {
                break;
            }
            let entry_fn: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(rd32(vt.wrapping_add(VT_ENTRY)) as usize);
            let idx = entry_fn(this, round);
            let gate: u32 = lf_checker_rt::callee_thiscall!(CAL_GATE, u32, ctx, idx);
            if (gate & 0xFF) == 0 {
                let table = info[2];
                let entry = rd32(table.wrapping_add(idx.wrapping_mul(4)));
                let rank: u32 = lf_checker_rt::callee_thiscall!(CAL_RANK, u32, entry);
                let step = match rank {
                    1 | 2 | 3 | 5 => STEP_HIT,
                    _ => 0,
                };
                if expected == round {
                    let found: u32 =
                        lf_checker_rt::callee_thiscall!(CAL_FIND, u32, ctx, idx);
                    alive = 0;
                    if found != 0 {
                        let kind: u32 =
                            lf_checker_rt::callee_thiscall!(CAL_KIND, u32, found);
                        if kind <= KIND_LIMIT {
                            wr32(out_pair, rd32(found.wrapping_add(4)));
                            wr32(out_pair.wrapping_add(4), rd32(found.wrapping_add(8)));
                            alive = 1;
                        }
                    }
                    (out_flag as *mut u8).write(alive);
                } else {
                    let prev = count;
                    count = count.wrapping_add(step);
                    if count > limit {
                        alive = 0;
                    } else {
                        let stored: u32 = lf_checker_rt::callee_thiscall!(
                            CAL_STORE, u32, ctx, idx, prev, step);
                        if (stored & 0xFF) == 0 {
                            alive = 0;
                        } else {
                            alive = 1;
                            wr32(out_mask, 1u32 << round);
                            wr32(out_mask.wrapping_add(4), 0);
                        }
                    }
                }
            }
            round += 1;
        }
        alive as u32
    }
});
