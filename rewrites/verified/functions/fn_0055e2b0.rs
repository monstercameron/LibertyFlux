// original: 0x0055e2b0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_59, player_schema::LeaderboardInfo, 10>::vf6

/// Row index of a leaderboard value, or -1.
///
/// `key` is the value to find. Calls the leaderboard fetch helper (callee 1)
/// with leaderboard id 0x106 and a six-word scratch query; when the helper
/// reports failure (low byte of its answer clear) the result is -1.
/// Otherwise query word 3 holds the row count and word 4 points at the row
/// array: the first index whose row equals `key` is returned, or -1 when the
/// count is not positive or no row matches. The scan is a signed 32-bit loop;
/// the stack argument is read once.
///
/// Original: 0x0055e2b0 (stdcall, one stack word; entry registers ignored).
lf_checker_rt::export!(stdcall, rw_0055e2b0(key: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x106;
        const FETCH_CALLEE: u32 = 1;
        const COUNT_WORD: usize = 3;
        const ROWS_WORD: usize = 4;
        const NOT_FOUND: u32 = 0xFFFF_FFFF;
        let mut query = [0u32; 6];
        let answer: u32 = lf_checker_rt::callee_fastcall!(FETCH_CALLEE, u32,
            LEADERBOARD_ID, query.as_mut_ptr() as u32);
        if answer & 0xFF == 0 {
            return NOT_FOUND;
        }
        let count = query[COUNT_WORD] as i32;
        if count <= 0 {
            return NOT_FOUND;
        }
        let rows = query[ROWS_WORD] as *const u32;
        let mut index = 0i32;
        while index < count {
            if rows.add(index as usize).read() == key {
                return index as u32;
            }
            index += 1;
        }
        NOT_FOUND
    }
});
