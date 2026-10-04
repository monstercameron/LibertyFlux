// original: 0x0051cea0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Race23NoHolds, player_schema::LeaderboardInfo, 10>::vf7

/// Look up one leaderboard column value by row index.
///
/// Same shape as rw_0051c5e0 for leaderboard 0x74: column base arrives at
/// struct +0x10, returns base[index] or -1 on helper failure. stdcall.
lf_checker_rt::export!(stdcall, rw_0051cea0(index: u32) -> u32 {
    unsafe {
        const LEADERBOARD: u32 = 0x74;
        const COL_BASE: usize = 4; // +0x10
        let mut info = [0u32; 8];
        let ok: u32 =
            lf_checker_rt::callee_fastcall!(1, u32, LEADERBOARD, info.as_mut_ptr() as u32);
        if (ok as u8) == 0 {
            return 0xFFFF_FFFF;
        }
        let base = info[COL_BASE];
        (base.wrapping_add(index.wrapping_mul(4)) as *const u32).read_unaligned()
    }
});
