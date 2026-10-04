// original: 0x005967f0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_265, player_schema::LeaderboardInfo, 10>::vf7
/// Fetch the leaderboard value column and return the entry at the requested
/// row, or -1 when the fetch fails. There is no bounds check on the row.
export!(stdcall, rw_005967f0(arg: u32) -> i32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x1dd;
        let mut buf = [0u32; 6];
        let ok: u32 = callee_fastcall!(1, u32, LEADERBOARD_ID, buf.as_mut_ptr() as u32);
        if (ok as u8) == 0 {
            return -1;
        }
        let values = buf[4] as *const u32;
        *values.add(arg as usize) as i32
    }
});
