// original: 0x00596ED0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_267, player_schema::LeaderboardInfo, 10>::vf14

/// Poll ranking rows for one episodic-race leaderboard and collect matches.
///
/// `this` is the leaderboard-info object: the dword at `+0x00` is its virtual
/// table, slot `+0x2c` answers the index of the active row for this poll and
/// slot `+0x30` maps a loop index (0..18) to a row handle. `session` is an
/// opaque handle passed to every helper call; `limit` bounds the cursor
/// (`base + limit`, wrapping). `pair_out` receives 8 bytes, `bits_out` a
/// 64-bit mask, `flag_out` one byte; all three are cleared first.
///
/// Up to 19 rows are visited while the running flag is non-zero. Row `i` is
/// skipped when helper 4 says so; otherwise the running flag is cleared and
/// helper 5 classifies the row's table word: answers 1, 2, 3 or 5 set the
/// flag to 8, anything else leaves it 0. When the active index equals `i`,
/// helper 6 fetches the row object and, if its helper-7 size is at most 8,
/// its 8 bytes at `+4` are copied to `pair_out` and the flag set to 1; the
/// flag byte is then stored to `flag_out`. Otherwise the cursor advances by
/// the flag and helper 8 must accept the step, setting bit `i` in `bits_out`
/// and the flag to 1; a cursor past the bound or a refused step clears the
/// flag and ends the scan. Returns the final flag (0 or 1).
///
/// Edge cases: a refused table fetch returns 0 at once; a null row object or
/// an oversize one skips the copy; the bit index never reaches 32 (19 rows).
///
/// Original: thiscall, six stack words; `session` is only passed through to
/// the helpers. The original homes two locals over its incoming argument
/// slots (cursor over `base`, active index over `limit`); both are dead
/// after the callee-cleaned return, so the proof runs with the stack check
/// off.
lf_checker_rt::export!(thiscall, rw_00596ED0(this: u32, base: u32, pair_out: u32, bits_out: u32, flag_out: u32, session: u32, limit: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x1df;
        const SLOT_ACTIVE: u32 = 0x2c;
        const SLOT_ROW: u32 = 0x30;
        const ROW_COUNT: u32 = 19;
        const STEP_BIG: u32 = 8;
        const COPY_LIMIT: u32 = 8;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }

        wr32(bits_out, 0);
        wr32(bits_out + 4, 0);
        (flag_out as *mut u8).write(0);
        let bound = limit.wrapping_add(base);
        let mut cursor = base;

        let vft = rd32(this);
        let query_active: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(rd32(vft + SLOT_ACTIVE) as usize);
        let active = query_active(this);

        // Table fetch; the stub writes the table pointer at [edx+8], exactly
        // where the original keeps it in its own frame.
        let mut scratch = [0u32; 3];
        scratch[2] = 0;
        let fetched: u8 = lf_checker_rt::callee_fastcall!(
            2, u8, LEADERBOARD_ID, scratch.as_mut_ptr() as u32
        );
        if fetched == 0 {
            return 0;
        }
        let table = scratch[2];

        let query_row: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(rd32(vft + SLOT_ROW) as usize);
        let mut flag: u8 = 1;
        let mut idx: u32 = 0;
        while idx < ROW_COUNT {
            if flag == 0 {
                break;
            }
            let row = query_row(this, idx);
            let skip: u8 = lf_checker_rt::callee_thiscall!(4, u8, session, row);
            if skip != 0 {
                idx += 1;
                continue;
            }
            flag = 0;
            let entry = rd32(table.wrapping_add(row.wrapping_mul(4)));
            let code: u32 = lf_checker_rt::callee_thiscall!(5, u32, entry);
            if code != 0xffff_ffff {
                let d = code.wrapping_sub(1);
                if d <= 4 && d != 3 {
                    flag = STEP_BIG as u8;
                }
            }
            if active == idx {
                flag = 0;
                let obj: u32 = lf_checker_rt::callee_thiscall!(6, u32, session, row);
                if obj != 0 {
                    let size: u32 = lf_checker_rt::callee_thiscall!(7, u32, obj);
                    if size <= COPY_LIMIT {
                        let pair = ((obj + 4) as *const u64).read_unaligned();
                        (pair_out as *mut u64).write_unaligned(pair);
                        flag = 1;
                    }
                }
                (flag_out as *mut u8).write(flag);
            } else {
                let prev = cursor;
                cursor = cursor.wrapping_add(flag as u32);
                // NOTE: the original also writes `cursor` back to its incoming
                // `base` slot; dead after the callee pops 0x18 bytes, unaddressable from safe
                // Rust (stack check off, see results.json `narrowed`).
                if cursor > bound {
                    flag = 0;
                } else {
                    let accepted: u8 =
                        lf_checker_rt::callee_thiscall!(8, u8, session, row, prev, flag as u32);
                    if accepted == 0 {
                        flag = 0;
                    } else {
                        flag = 1;
                        // Set bit `idx` in the 64-bit mask (`bts` semantics;
                        // `idx` < 19, so only the low half is ever reached).
                        let (lo, hi) = if idx < 32 {
                            (1u32 << idx, 0)
                        } else if idx < 64 {
                            (0, 1u32 << (idx & 31))
                        } else {
                            (0, 0)
                        };
                        wr32(bits_out, lo);
                        wr32(bits_out + 4, hi);
                    }
                }
            }
            idx += 1;
        }
        flag as u32
    }
});
