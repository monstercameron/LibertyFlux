// original: 0x00526E70 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Race60NoHolds, player_schema::LeaderboardInfo, 10>::vf14

/// Poll one ranked leaderboard over 7 rounds, collecting the watched
/// round's row and a mask of the rounds that produced one.
///
/// `this` is the leaderboard-info object: dword 0 points at its virtual table,
/// slot `+0x2c` picks the watched round and slot `+0x30` maps a round index to
/// a row id. `base` is the starting cursor and `budget` caps its advance
/// (`base + budget`, wrapping). `out8` receives the watched round's 8-byte
/// row, `mask` a 64-bit mask with one bit per row-producing round, `flag` the
/// watched round's outcome byte, and `mgr` is passed through as the object of
/// the polled helpers. `KIND` (0x4c) identifies which leaderboard this
/// instantiation polls; it is passed to the gate helper and compared there.
///
/// Algorithm: clear the outputs, ask slot `+0x2c` for the watched round, ask
/// the gate helper (kind `KIND`) for the row table. A zero low byte from the
/// gate ends the call at once, returning the gate's answer. Otherwise each
/// round maps its index to a row id (slot `+0x30`), lets the skip helper veto
/// the round, and ranks the row's gate word (helper 5): ranks 1, 2, 3 and 5
/// cost 8 cursor steps, every other rank is free. The watched round fetches
/// its row object and, when the object's class word is 8 or less (signed),
/// copies the 8 bytes at object `+4` into `out8` and reports 1 in `flag`,
/// else 0. Every other round advances the cursor by its cost (past the limit
/// fails the round), offers the round to the sink helper, and on acceptance
/// sets its bit in `mask`. Any failed round stops the loop after it; the
/// return is the last helper answer with its low byte replaced by the final
/// outcome (1 if the last round ran, 0 if it failed or the loop stopped).
///
/// Edge cases: a picked round past the last means no round is watched (every
/// round sinks); a null row object or a class word above 8 skips the copy but
/// still writes `flag`; a wrapping `base + budget` fails every sinking round.
/// The rank switch is decoded from the function's jump table ([L,L,L,F,L]
/// over rank minus one); the table and its base carry relocation entries,
/// so the dispatch runs as is and every rank is exercised.
///
/// Original: 0x00526E70 (thiscall, ECX is `this`, six stack words).
lf_checker_rt::export!(thiscall, rw_00526e70(this: u32, base: u32, out8: u32, mask: u32, flag: u32, mgr: u32, budget: u32) -> u32 {
    unsafe {
        const KIND: u32 = 0x4c;
        const ROUNDS: u32 = 7;
        const SLOT_PICK: u32 = 0x2c;
        const SLOT_ROWID: u32 = 0x30;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }

        wr32(mask, 0);
        wr32(mask.wrapping_add(4), 0);
        (flag as *mut u8).write(0);
        let limit = budget.wrapping_add(base);

        let vtbl = rd32(this);
        let pick: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(rd32(vtbl.wrapping_add(SLOT_PICK)) as usize);
        let picked = pick(this);
        let mut eax = picked;

        let mut feat = [0u32; 3];
        feat[2] = 0;
        let gate_ans: u32 =
            lf_checker_rt::callee_fastcall!(2, u32, KIND, feat.as_mut_ptr() as u32);
        eax = gate_ans;
        if gate_ans & 0xFF == 0 {
            return gate_ans;
        }
        let table = feat[2];

        let rowid: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(rd32(vtbl.wrapping_add(SLOT_ROWID)) as usize);

        let mut ok: u8 = 1;
        let mut cursor = base;
        let mut round: u32 = 0;
        loop {
            if ok == 0 {
                break;
            }
            let row = rowid(this, round);
            eax = row;
            let skip: u32 = lf_checker_rt::callee_thiscall!(4, u32, mgr, row);
            eax = skip;
            if skip & 0xFF == 0 {
                let gate = rd32(table.wrapping_add(row.wrapping_mul(4)));
                let rank: u32 = lf_checker_rt::callee_thiscall!(5, u32, gate);
                // The switch decrements the rank first (except on the -1 arm,
                // which jumps out before it); the decremented value is what a
                // later budget failure returns in the high bytes.
                eax = if rank == 0xFFFFFFFF {
                    rank
                } else {
                    rank.wrapping_sub(1)
                };
                // Cost class from the rank: ranks 1, 2, 3 and 5 cost 8
                // cursor steps, every other rank is free.
                let step: u32 = match rank {
                    1 | 2 | 3 | 5 => 8,
                    _ => 0,
                };
                if picked == round {
                    ok = 0;
                    let obj: u32 = lf_checker_rt::callee_thiscall!(6, u32, mgr, row);
                    eax = obj;
                    if obj != 0 {
                        let cls: u32 = lf_checker_rt::callee_thiscall!(7, u32, obj);
                        eax = cls;
                        if (cls as i32) <= 8 {
                            let v =
                                ((obj.wrapping_add(4)) as *const u64).read_unaligned();
                            (out8 as *mut u64).write_unaligned(v);
                            ok = 1;
                        }
                    }
                    (flag as *mut u8).write(ok);
                    eax = flag;
                } else {
                    // The sink is offered the round-start cursor: the original
                    // pushes the carried value from its spill slot, while the
                    // limit check below already uses the advanced cursor.
                    let offered = cursor;
                    cursor = cursor.wrapping_add(step);
                    if cursor > limit {
                        ok = 0;
                    } else {
                        let acc: u32 = lf_checker_rt::callee_thiscall!(
                            8, u32, mgr, row, offered, step
                        );
                        eax = acc;
                        if acc & 0xFF == 0 {
                            ok = 0;
                        } else {
                            ok = 1;
                            // Set bit `round` in the 64-bit mask. The original
                            // sets the bit then splits the pair for indices
                            // past 31/63; the loop bound keeps `round` below
                            // 8, so those arms are dead but mirrored.
                            let bit = 1u32.wrapping_shl(round);
                            let (lo, hi) = if round >= 0x20 {
                                if round >= 0x40 { (0, 0) } else { (0, bit) }
                            } else {
                                (bit, 0)
                            };
                            wr32(mask, lo);
                            wr32(mask.wrapping_add(4), hi);
                            eax = mask;
                        }
                    }
                }
            }
            round += 1;
            if round >= ROUNDS {
                break;
            }
        }
        (eax & 0xFFFFFF00) | (ok as u32)
    }
});
