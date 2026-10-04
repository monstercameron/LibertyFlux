// original: 0x00558320 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_37, player_schema::LeaderboardInfo, 10>::vf8
/// Size code of a leaderboard row's mapped class, or UNKNOWN (0).
///
/// Asks the row-fetch helper (checker callee 1: board id in ECX, out-buffer in
/// EDX) for board 0xea; it answers in AL and leaves the row-id array at
/// out+20. The word at `index` goes to the row classifier (checker
/// callee 2, thiscall, row in ECX). A failed fetch, a classifier answer of -1,
/// or an answer whose minus-one exceeds 4 yields 0. Otherwise answers
/// 1..=5 select the five cases (4, 8, 8, 0, 4), folded here into a table since each
/// arm returns a constant.
///
/// `index` is not bounds-checked.
///
/// Original: 0x00558320 (stdcall, one stack word; ECX unused).
lf_checker_rt::export!(stdcall, rw_00558320(index: u32) -> u32 {
    unsafe {
        const BOARD_ID: u32 = 0xea;
        const FETCH: u32 = 1;
        const CLASSIFY: u32 = 2;
        const ROWS_W: usize = 5;
        const CASES: [u32; 5] = [4, 8, 8, 0, 4];
        const DEFAULT: u32 = 0;
        const NO_CLASS: u32 = 0xffff_ffff;
        let mut out = [0u32; 6];
        let ok: u32 = lf_checker_rt::callee_fastcall!(
            FETCH, u32, BOARD_ID, out.as_mut_ptr() as u32
        );
        if (ok & 0xff) == 0 {
            return DEFAULT;
        }
        let rows = out[ROWS_W];
        let v = (rows.wrapping_add(index.wrapping_mul(4)) as *const u32).read();
        let r: u32 = lf_checker_rt::callee_thiscall!(CLASSIFY, u32, v);
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
