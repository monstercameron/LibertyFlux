// original: 0x0051c5e0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Race21NoHolds, player_schema::LeaderboardInfo, 10>::vf7

/// Look up one leaderboard column value by row index.
///
/// `index` is the row. Asks the table helper (callee 1) for leaderboard
/// 0x72, which answers through a stack struct whose word at +0x10 receives
/// the column base; returns the word at base[index], or -1 when the helper
/// reports failure (low byte of its answer is zero). Out-of-range indexes
/// read or fault exactly like the original's indexed load. stdcall.
lf_checker_rt::export!(stdcall, rw_0051c5e0(index: u32) -> u32 {
    unsafe {
        const LEADERBOARD: u32 = 0x72;
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
