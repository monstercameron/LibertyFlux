// original: 0x0056e480 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_118, player_schema::LeaderboardInfo, 10>::vf6
/// Position of `wanted` in the id table, or NOT_FOUND.
///
/// Fetches the tables for LEADERBOARD_ID and fails with NOT_FOUND when the
/// fetch fails. The entry count (frame word 3) is compared signed: zero or
/// negative finds nothing. Otherwise scans the id table (frame word 4) for
/// `wanted` and returns the first matching position, or NOT_FOUND.
///
/// stdcall with one stack argument; no `this`, no globals, no caller-visible
/// writes.
lf_checker_rt::export!(stdcall, rw_0056e480(wanted: u32) -> u32 {
    unsafe {
        const NOT_FOUND: u32 = 0xFFFF_FFFF;
        const COUNT_WORD: usize = 3;
        const IDS_WORD: usize = 4;
        const LEADERBOARD_ID: u32 = 321;
        const FETCH_CALLEE: u32 = 1;
        let mut frame = [0u32; 6];
        let answer = lf_checker_rt::callee_fastcall!(FETCH_CALLEE, u32, LEADERBOARD_ID,
            frame.as_mut_ptr() as u32);
        if answer & 0xFF == 0 {
            return NOT_FOUND;
        }
        let count = frame[COUNT_WORD];
        if (count as i32) <= 0 {
            return NOT_FOUND;
        }
        let ids = frame[IDS_WORD];
        let mut pos = 0u32;
        loop {
            let id = (ids.wrapping_add(pos.wrapping_mul(4)) as *const u32).read();
            if id == wanted {
                return pos;
            }
            pos += 1;
            if (pos as i32) >= count as i32 {
                return NOT_FOUND;
            }
        }
    }
});
