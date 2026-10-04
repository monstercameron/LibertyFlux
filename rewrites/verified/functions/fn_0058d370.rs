// original: 0x0058d370 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_231, player_schema::LeaderboardInfo, 10>::vf8
/// Leaderboard cell width: resolve this leaderboard, classify row `index`
/// of its column through the kind helper, and map the kind to a width.
/// Unknown kinds and helper failures read 0.
export!(stdcall, rw_0058d370(index: u32) -> u32 {
    unsafe {
        /// Leaderboard id passed to the descriptor helper.
        const LEADERBOARD_ID: u32 = 0x1bf;
        let mut desc = [0u32; 6];
        let ok: u8 = callee_fastcall!(1, u8, LEADERBOARD_ID, desc.as_mut_ptr() as u32);
        if ok == 0 {
            return 0;
        }
        let column = desc[5] as *const u32;
        let kind: u32 = callee_thiscall!(2, u32, *column.add(index as usize));
        if kind == 0xFFFF_FFFF {
            return 0;
        }
        match kind.wrapping_sub(1) {
            0 => 4,
            1 | 2 => 8,
            4 => 4,
            _ => 0,
        }
    }
});
