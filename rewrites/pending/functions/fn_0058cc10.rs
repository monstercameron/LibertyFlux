// original: 0x0058cc10 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_230, player_schema::LeaderboardInfo, 10>::vf12
/// Leaderboard column-index lookup: resolve this leaderboard through the
/// shared descriptor helper, read row `index` from the value column, and
/// return its position in the key column, or -1 when the helper fails,
/// the row holds -1, or the value is absent.
export!(stdcall, rw_0058cc10(index: u32) -> u32 {
    unsafe {
        /// Leaderboard id passed to the descriptor helper.
        const LEADERBOARD_ID: u32 = 0x1be;
        /// Sentinel for every failure path.
        const NOT_FOUND: u32 = 0xFFFF_FFFF;
        let mut desc = [0u32; 6];
        let ok: u8 = callee_fastcall!(1, u8, LEADERBOARD_ID, desc.as_mut_ptr() as u32);
        if ok == 0 {
            return NOT_FOUND;
        }
        let count = desc[1];
        let keys = desc[2] as *const u32;
        let values = desc[5] as *const u32;
        let wanted = *values.add(index as usize);
        if wanted == NOT_FOUND || count == 0 {
            return NOT_FOUND;
        }
        let mut k = 0u32;
        while k < count {
            if *keys.add(k as usize) == wanted {
                return k;
            }
            k = k.wrapping_add(1);
        }
        NOT_FOUND
    }
});
