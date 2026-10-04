// original: 0x0058d2d0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_231, player_schema::LeaderboardInfo, 10>::vf6
/// Leaderboard key search: resolve this leaderboard through the shared
/// descriptor helper and return the row holding `value` in the key
/// column, or -1 on any failure.
export!(stdcall, rw_0058d2d0(value: u32) -> u32 {
    unsafe {
        /// Leaderboard id passed to the descriptor helper.
        const LEADERBOARD_ID: u32 = 0x1bf;
        /// Sentinel for every failure path.
        const NOT_FOUND: u32 = 0xFFFF_FFFF;
        let mut desc = [0u32; 5];
        let ok: u8 = callee_fastcall!(1, u8, LEADERBOARD_ID, desc.as_mut_ptr() as u32);
        if ok == 0 {
            return NOT_FOUND;
        }
        let count = desc[3] as i32;
        if count <= 0 {
            return NOT_FOUND;
        }
        let keys = desc[4] as *const u32;
        let mut k = 0i32;
        while k < count {
            if *keys.add(k as usize) == value {
                return k as u32;
            }
            k += 1;
        }
        NOT_FOUND
    }
});
