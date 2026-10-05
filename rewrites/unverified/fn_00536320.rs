// original: 0x00536320 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Race56Standard, player_schema::LeaderboardInfo, 10>::vf14

/// Ranked-leaderboard row fill, virtual slot 14 of this leaderboard's
/// concrete info object: over 5 row indexes, decide per row whether the
/// backend claims it and record the outcome in the caller's buffers.
///
/// `this` is the info object (vtable slot 11 initialises the query, slot 12
/// resolves one row index to a row handle). The six stack words are output
/// and limit parameters, not the rows themselves:
/// `base` is the running cursor's start, `copy_dst` receives an 8-byte row
/// key, `bits_out` receives two words with exactly one bit set for the row
/// index (low word for indexes 0-31), `flag_out` receives the last outcome
/// byte written, `ctx` is only passed through to the backend callees, and
/// `limit` caps the cursor (`base + limit`, wrapping).
///
/// Algorithm: zero `bits_out` and `flag_out`; ask slot 11 for the query
/// token; ask the backend (id 130) to validate the token into a scratch
/// buffer, whose third word is a handle table. A false answer returns 0.
/// Otherwise for each row index while the sticky flag is set: resolve the
/// handle through slot 12; if the backend claims the row, keep the flag and
/// move on. If not, classify the handle through a second backend call and a
/// five-way switch (cases 0, 1, 2 and 4 step the cursor by 8, case 3 and the
/// default leave it). When the token equals the row index, fetch the row's
/// key object instead: with no object, or a key longer than 8 bytes, clear
/// the flag, else copy the 8 key bytes to `copy_dst` and set it; either way
/// store the flag into `flag_out`. When the token differs, advance the
/// cursor by the step: past the limit, or on a backend refusal, clear the
/// flag, else set the row's bit in `bits_out` and set the flag. Once
/// cleared the flag stays cleared and the remaining rows are skipped.
/// Returns the flag (0 or 1).
///
/// Edge cases: a zero token answer returns before touching the loop; an
/// out-of-range classification falls through to the cursor logic with a zero
/// step; cursor arithmetic wraps modulo 2^32.
///
/// Original: 0x00536320 (thiscall, `this` in ECX, six stack words, callee
/// cleans 0x18, boolean in AL). The switch is a jump table in the original;
/// the five cases are a plain range check here. The bit-set logic keeps the
/// original's general form (low word under 32 rows, high word under 64).
lf_checker_rt::export!(thiscall, rw_00536320(this: u32, base: u32, copy_dst: u32, bits_out: u32, flag_out: u32, ctx: u32, limit: u32) -> u8 {
    unsafe {
        const LEADERBOARD_ID: u32 = 130;
        const ROWS: u32 = 5;
        const VTBL_INIT: u32 = 0x2c;
        const VTBL_RESOLVE: u32 = 0x30;
        const CURSOR_STEP: u32 = 8;
        const MAX_KEY_LEN: u32 = 8;
        const CAL_VALIDATE: u32 = 2;
        const CAL_CLAIMED: u32 = 4;
        const CAL_CLASSIFY: u32 = 5;
        const CAL_FETCH: u32 = 6;
        const CAL_KEYLEN: u32 = 7;
        const CAL_ADVANCE: u32 = 8;

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

        wr32(bits_out, 0);
        wr32(bits_out.wrapping_add(4), 0);
        wr8(flag_out, 0);
        let cursor_limit = limit.wrapping_add(base);

        let init: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(rd32(rd32(this).wrapping_add(VTBL_INIT)) as usize);
        let token = init(this);

        // Scratch answer buffer; the original zeroes only word 2, the rest
        // is zero stack fill on both sides, made explicit here.
        let mut answer = [0u32; 4];
        let valid: u8 = lf_checker_rt::callee_fastcall!(
            CAL_VALIDATE, u8, LEADERBOARD_ID, answer.as_mut_ptr() as u32
        );
        if valid == 0 {
            return 0;
        }
        let handle_table = answer[2];

        let resolve: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(rd32(rd32(this).wrapping_add(VTBL_RESOLVE)) as usize);
        let mut flag: u8 = 1;
        let mut cursor = base;
        let mut row: u32 = 0;
        while row < ROWS {
            if flag != 0 {
                let handle = resolve(this, row);
                let claimed: u8 =
                    lf_checker_rt::callee_thiscall!(CAL_CLAIMED, u8, ctx, handle);
                if claimed == 0 {
                    let class: u32 = lf_checker_rt::callee_thiscall!(
                        CAL_CLASSIFY, u32, rd32(handle_table.wrapping_add(handle.wrapping_mul(4)))
                    );
                    let mut step = 0u32;
                    if class != 0 {
                        let case = class.wrapping_sub(1);
                        if case < 5 && case != 3 {
                            step = CURSOR_STEP;
                        }
                    }
                    if token == row {
                        flag = 0;
                        let key = lf_checker_rt::callee_thiscall!(CAL_FETCH, u32, ctx, handle);
                        if key != 0 {
                            let len: u32 =
                                lf_checker_rt::callee_thiscall!(CAL_KEYLEN, u32, key);
                            if len <= MAX_KEY_LEN {
                                wr32(copy_dst, rd32(key.wrapping_add(4)));
                                wr32(copy_dst.wrapping_add(4), rd32(key.wrapping_add(8)));
                                flag = 1;
                            }
                        }
                        wr8(flag_out, flag);
                    } else {
                        cursor = cursor.wrapping_add(step);
                        if cursor > cursor_limit {
                            flag = 0;
                        } else {
                            let advanced: u8 = lf_checker_rt::callee_thiscall!(
                                CAL_ADVANCE, u8, ctx, handle, cursor, step
                            );
                            if advanced == 0 {
                                flag = 0;
                            } else {
                                let (lo, hi) = if row < 32 {
                                    (1u32 << row, 0)
                                } else if row < 64 {
                                    (0, 1u32 << (row - 32))
                                } else {
                                    (0, 0)
                                };
                                wr32(bits_out, lo);
                                wr32(bits_out.wrapping_add(4), hi);
                                flag = 1;
                            }
                        }
                    }
                }
            }
            row = row.wrapping_add(1);
        }
        flag
    }
});
