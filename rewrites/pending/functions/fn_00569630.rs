// original: 0x569630 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_100, player_schema::LeaderboardInfo, 10>::vf7
/// Row lookup for one ranked leaderboard. (`vf7 Race100`).
///
/// Fetches the tables and returns the row word at `index`, or -1 when the fetch
/// step reports failure in the low byte of its answer.
/// Leaderboard id: 0x12f.
export!(stdcall, rw_00569630(index: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x12f;
        let mut frame = [0u32; 6];
        let fetch: extern "fastcall" fn(u32, u32) -> u32 =
            core::mem::transmute(callee_addr(1) as usize);
        if fetch(LEADERBOARD_ID, frame.as_mut_ptr() as u32) as u8 == 0 {
            return 0xFFFF_FFFF;
        }
        let rows = frame[4];
        (rows.wrapping_add(index.wrapping_mul(4)) as *const u32).read()
    }
});
