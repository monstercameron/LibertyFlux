// original: 0x0058d3e0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_231, player_schema::LeaderboardInfo, 10>::vf9
/// Leaderboard cell format: resolve this leaderboard, classify row `index`
/// of its column through the kind helper, and map the kind to a format
/// code. Unknown kinds and helper failures read -1.
export!(stdcall, rw_0058d3e0(index: u32) -> u32 {
    unsafe {
        /// Leaderboard id passed to the descriptor helper.
        const LEADERBOARD_ID: u32 = 0x1bf;
        /// Sentinel for unknown kinds and failures.
        const UNKNOWN: u32 = 0xFFFF_FFFF;
        let mut desc = [0u32; 6];
        let ok: u8 = callee_fastcall!(1, u8, LEADERBOARD_ID, desc.as_mut_ptr() as u32);
        if ok == 0 {
            return UNKNOWN;
        }
        let column = desc[5] as *const u32;
        let kind: u32 = callee_thiscall!(2, u32, *column.add(index as usize));
        if kind == UNKNOWN {
            return UNKNOWN;
        }
        match kind.wrapping_sub(1) {
            0 => 0,
            1 => 1,
            2 => 3,
            4 => 2,
            _ => UNKNOWN,
        }
    }
});
