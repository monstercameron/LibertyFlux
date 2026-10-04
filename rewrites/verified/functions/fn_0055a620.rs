// original: 0x0055a620 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_45, player_schema::LeaderboardInfo, 10>::vf8

/// Size code of a leaderboard row's mapped class, or UNKNOWN (0).
///
/// Calls the leaderboard fetch helper (checker callee 1, fastcall: board id in
/// ECX, out-buffer in EDX) for board 0xf9; it reports success in AL and fills
/// the row-id array pointer at out-buffer +20. The word at `index` is
/// passed to the row classifier (checker callee 2, thiscall, row in ECX). When
/// the helper fails, when the classifier reports -1, or when its answer minus
/// one is above 4, the result is 0. Otherwise the answer selects one
/// of five cases (4, 8, 8, 0, 4) through the original's jump table, folded here
/// into a plain table since every arm returns a constant.
///
/// `index` is not bounds-checked. The classifier's answers 1..=5 select the
/// five cases in order; anything else (including 0, whose minus-one wraps)
/// takes the default.
///
/// Original: 0x0055A620 (stdcall, one stack word; ECX is dead on entry).
lf_checker_rt::export!(stdcall, rw_0055a620(index: u32) -> u32 {
    unsafe {
        const BOARD_ID: u32 = 0xf9;
        const FETCH_ROWS: u32 = 1;
        const CLASSIFY_ROW: u32 = 2;
        const ROWS_WORD: usize = 5;
        const CASES: [u32; 5] = [4, 8, 8, 0, 4];
        const DEFAULT: u32 = 0;
        const NO_CLASS: u32 = 0xffff_ffff;
        let mut out = [0u32; 6];
        let ok: u32 = lf_checker_rt::callee_fastcall!(
            FETCH_ROWS,
            u32,
            BOARD_ID,
            out.as_mut_ptr() as u32
        );
        if (ok & 0xff) == 0 {
            return DEFAULT;
        }
        let rows = out[ROWS_WORD];
        let v = (rows.wrapping_add(index.wrapping_mul(4)) as *const u32).read();
        let r: u32 = lf_checker_rt::callee_thiscall!(CLASSIFY_ROW, u32, v);
        if r == NO_CLASS {
            return DEFAULT;
        }
        let i = r.wrapping_sub(1);
        if i > 4 {
            return DEFAULT;
        }
        CASES[i as usize]
    }
});
