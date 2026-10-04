// original: 0x0056eda0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_120, player_schema::LeaderboardInfo, 10>::vf7

/// Look up one leaderboard column value by row index.
///
/// `idx` is the row. Calls the leaderboard fetch helper (fastcall: ECX =
/// leaderboard id 0x143, EDX = scratch record) and, when it reports
/// success, returns the `idx`-th 32-bit entry of the value array the helper
/// wrote at record offset 0x10. When the helper reports failure the
/// result is 0xffffffff (all bits set). There is no bounds check: an
/// out-of-range index reads past the array exactly like the original.
///
/// Original: 0x0056eda0 (rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_120, player_schema::LeaderboardInfo, 10>::vf7; stdcall, one stack word).
lf_checker_rt::export!(stdcall, rw_0056eda0(idx: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x143;
        const ARR_WORD: usize = 4;
        const FAILED: u32 = 0xffffffff;
        let mut record = [0u32; 8];
        record[ARR_WORD] = 0;
        let ok: u8 = lf_checker_rt::callee_fastcall!(
            1, u8, LEADERBOARD_ID, record.as_mut_ptr() as u32);
        if ok == 0 {
            return FAILED;
        }
        let arr = record[ARR_WORD];
        (arr.wrapping_add(idx.wrapping_mul(4)) as *const u32).read()
    }
});
