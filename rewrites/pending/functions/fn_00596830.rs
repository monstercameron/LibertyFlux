// original: 0x00596830 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_265, player_schema::LeaderboardInfo, 10>::vf8
/// Fetch the leaderboard value column, classify the entry at the requested
/// row through the kind helper, and map the kind to a field width: kinds 1
/// and 5 give 4, kinds 2 and 3 give 8, anything else gives 0.
export!(stdcall, rw_00596830(arg: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x1dd;
        let mut buf = [0u32; 6];
        let ok: u32 = callee_fastcall!(1, u32, LEADERBOARD_ID, buf.as_mut_ptr() as u32);
        if (ok as u8) == 0 {
            return 0;
        }
        let values = buf[5] as *const u32;
        let elem = *values.add(arg as usize);
        let kind: u32 = callee_fastcall!(2, u32, elem, 0);
        match kind {
            1 => 4,
            2 | 3 => 8,
            5 => 4,
            _ => 0,
        }
    }
});
