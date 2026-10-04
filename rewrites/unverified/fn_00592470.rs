// original: 0x00592470 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_250, player_schema::LeaderboardInfo, 10>::vf14
/// Read one leaderboard row-set for a ranked episodic race (template
/// instantiation for race 250, leaderboard id 0x1d2).
///
/// `this` is the leaderboard-info object (vtable pointer at `+0`). `ctx` is an
/// opaque context passed through to the row helpers. `base` and `span` bound
/// a cursor: it starts at `base` and must stay within `base + span` (wrapping
/// addition). `row_out` receives an 8-byte row snapshot, `mask_out` a 64-bit
/// one-hot position mask, `flag_out` a single success byte; all three are
/// zeroed before anything else runs.
///
/// Algorithm: fetch the round selector through vtable slot `0x2c`, then fetch
/// the row-pointer table through the table helper (called with the leaderboard
/// id; its out-pointer lands 14 bytes past the scratch pointer). A zero answer
/// there returns 0 at once. Otherwise run up to 19 rounds: fetch the round
/// index through vtable slot `0x30`; the filter helper may skip the round.
/// When it does not, classify the table entry through the kind helper: kinds
/// 1, 2, 3 and 5 advance the cursor by 8, kind 4 and anything else by 0 (the
/// original selects this through a jump table). When the selector equals the
/// round number, fetch the row through the row helpers, copying its 8 bytes at
/// `+4` when the size check passes, and report success in the flag byte.
/// Otherwise advance the cursor (failing when it passes the bound), confirm
/// through the commit helper, and set bit `round` of the 64-bit mask. The
/// first failure sticks: the loop exits and the result is 0, else 1.
///
/// Original: 0x00592470 (thiscall, `this` in ECX, six stack words; returns a bool
/// in AL with undefined upper bits). All eight race siblings share this code;
/// only the leaderboard id differs. No floating point, no globals.
lf_checker_rt::export!(thiscall, rw_00592470(this: u32, base: u32, row_out: u32, mask_out: u32, flag_out: u32, ctx: u32, span: u32) -> u32 {
    unsafe {
        const VT_SELECT: u32 = 0x2c;
        const VT_ROUND_INDEX: u32 = 0x30;
        const LEADERBOARD_ID: u32 = 0x1d2;
        const MAX_ROUNDS: u32 = 19;
        const CURSOR_STEP: u32 = 8;
        const MAX_ROW_SIZE: u32 = 8;
        const TABLE_CALLEE: u32 = 2;
        const FILTER_CALLEE: u32 = 4;
        const KIND_CALLEE: u32 = 5;
        const ROW_CALLEE: u32 = 6;
        const SIZE_CALLEE: u32 = 7;
        const COMMIT_CALLEE: u32 = 8;
        const TABLE_OUT_OFF: u32 = 14;
        const ROW_DATA_OFF: u32 = 4;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }

        wr32(mask_out, 0);
        wr32(mask_out.wrapping_add(4), 0);
        (flag_out as *mut u8).write(0);
        let mut cursor = base;
        let bound = span.wrapping_add(base);
        let select: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(rd32(rd32(this).wrapping_add(VT_SELECT)) as usize);
        let selector = select(this);
        let mut scratch = [0u32; 8];
        let scratch_ptr = scratch.as_mut_ptr() as u32;
        let table_ok: u32 =
            lf_checker_rt::callee_fastcall!(TABLE_CALLEE, u32, LEADERBOARD_ID, scratch_ptr);
        if (table_ok as u8) == 0 {
            return 0;
        }
        let table = rd32(scratch_ptr.wrapping_add(TABLE_OUT_OFF));
        let mut ok = true;
        let mut round = 0u32;
        while round < MAX_ROUNDS {
            if !ok {
                break;
            }
            let round_fn: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(rd32(rd32(this).wrapping_add(VT_ROUND_INDEX)) as usize);
            let index = round_fn(this, round);
            let keep: u32 = lf_checker_rt::callee_thiscall!(FILTER_CALLEE, u32, ctx, index);
            if (keep as u8) == 0 {
                let entry = rd32(table.wrapping_add(index.wrapping_mul(4)));
                let kind: u32 = lf_checker_rt::callee_thiscall!(KIND_CALLEE, u32, entry);
                let step = if kind == 0xFFFF_FFFF {
                    0
                } else {
                    let d = kind.wrapping_sub(1);
                    if d > 4 || d == 3 {
                        0
                    } else {
                        CURSOR_STEP
                    }
                };
                if selector == round {
                    ok = false;
                    let row: u32 = lf_checker_rt::callee_thiscall!(ROW_CALLEE, u32, ctx, index);
                    if row != 0 {
                        let size: u32 = lf_checker_rt::callee_thiscall!(SIZE_CALLEE, u32, row);
                        if size <= MAX_ROW_SIZE {
                            wr32(row_out, rd32(row.wrapping_add(ROW_DATA_OFF)));
                            wr32(
                                row_out.wrapping_add(4),
                                rd32(row.wrapping_add(ROW_DATA_OFF + 4)),
                            );
                            ok = true;
                        }
                    }
                    (flag_out as *mut u8).write(ok as u8);
                } else {
                    let advanced = cursor.wrapping_add(step);
                    if advanced > bound {
                        ok = false;
                    } else {
                        let done: u32 = lf_checker_rt::callee_thiscall!(
                            COMMIT_CALLEE,
                            u32,
                            ctx,
                            index,
                            cursor,
                            step
                        );
                        if (done as u8) == 0 {
                            ok = false;
                        } else {
                            let bit = round.wrapping_add(0);
                            let (lo, hi) = if bit < 32 {
                                (1u32 << bit, 0)
                            } else if bit < 64 {
                                (0, 1u32 << (bit - 32))
                            } else {
                                (0, 0)
                            };
                            wr32(mask_out, lo);
                            wr32(mask_out.wrapping_add(4), hi);
                            ok = true;
                        }
                    }
                    cursor = advanced;
                }
            }
            round = round.wrapping_add(1);
        }
        ok as u32
    }});
