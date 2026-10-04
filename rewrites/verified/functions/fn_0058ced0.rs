// original: 0x0058ced0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_230, player_schema::LeaderboardInfo, 10>::vf7
/// Leaderboard cell read: resolve this leaderboard through the shared
/// descriptor helper and return row `index` of its column, or -1 when
/// the helper fails.
export!(stdcall, rw_0058ced0(index: u32) -> u32 {
    unsafe {
        /// Leaderboard id passed to the descriptor helper.
        const LEADERBOARD_ID: u32 = 0x1be;
        /// Sentinel for the failure path.
        const NOT_FOUND: u32 = 0xFFFF_FFFF;
        let mut desc = [0u32; 5];
        let ok: u8 = callee_fastcall!(1, u8, LEADERBOARD_ID, desc.as_mut_ptr() as u32);
        if ok == 0 {
            return NOT_FOUND;
        }
        let column = desc[4] as *const u32;
        *column.add(index as usize)
    }
});
