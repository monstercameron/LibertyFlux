// original: 0x0055f070 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_62, player_schema::LeaderboardInfo, 10>::vf8

/// Size class of a leaderboard row's kind, or 0.
///
/// `index` selects a row. Calls the leaderboard fetch helper (callee 1) with
/// leaderboard id 0x109 and a six-word scratch query; when the helper
/// reports failure the result is 0. Otherwise the row value at `index`
/// (query word 5 points at the row array) is handed to the kind probe
/// (callee 2, thiscall, value in ecx). A probe answer of -1 yields 0;
/// otherwise the answer minus one selects through a five-entry table:
/// 1 maps to 4, 2 and 3 to 8, 4 to 0, 5 to 4, and anything else to 0.
///
/// Original: 0x0055f070 (stdcall, one stack word; entry registers ignored).
lf_checker_rt::export!(stdcall, rw_0055f070(index: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x109;
        const FETCH_CALLEE: u32 = 1;
        const PROBE_CALLEE: u32 = 2;
        const ROWS_WORD: usize = 5;
        const UNKNOWN: u32 = 0;
        let mut query = [0u32; 6];
        let answer: u32 = lf_checker_rt::callee_fastcall!(FETCH_CALLEE, u32,
            LEADERBOARD_ID, query.as_mut_ptr() as u32);
        if answer & 0xFF == 0 {
            return UNKNOWN;
        }
        let rows = query[ROWS_WORD] as *const u32;
        let value = rows.add(index as usize).read();
        let kind: u32 = lf_checker_rt::callee_thiscall!(PROBE_CALLEE, u32, value);
        if kind == 0xFFFF_FFFF {
            return UNKNOWN;
        }
        match kind.wrapping_sub(1) {
            0 => 4,
            1 | 2 => 8,
            3 => 0,
            4 => 4,
            _ => 0,
        }
    }
});
