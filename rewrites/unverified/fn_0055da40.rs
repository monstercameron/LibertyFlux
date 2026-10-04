// original: 0x0055da40 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_57, player_schema::LeaderboardInfo, 10>::vf7

/// Leaderboard row by index, or -1 when the fetch fails.
///
/// `index` selects a row. Calls the leaderboard fetch helper (callee 1) with
/// leaderboard id 0x104 and a six-word scratch query; when the helper
/// reports failure (low byte of its answer clear) the result is -1.
/// Otherwise query word 4 points at the row array and the word at `index` is
/// returned unchecked, exactly as the original's single indexed load.
///
/// Original: 0x0055da40 (stdcall, one stack word; entry registers ignored).
lf_checker_rt::export!(stdcall, rw_0055da40(index: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x104;
        const FETCH_CALLEE: u32 = 1;
        const ROWS_WORD: usize = 4;
        const FAILED: u32 = 0xFFFF_FFFF;
        let mut query = [0u32; 6];
        let answer: u32 = lf_checker_rt::callee_fastcall!(FETCH_CALLEE, u32,
            LEADERBOARD_ID, query.as_mut_ptr() as u32);
        if answer & 0xFF == 0 {
            return FAILED;
        }
        let rows = query[ROWS_WORD] as *const u32;
        rows.add(index as usize).read()
    }
});
