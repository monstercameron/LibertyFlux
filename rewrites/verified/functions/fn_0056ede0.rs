// original: 0x0056ede0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_120, player_schema::LeaderboardInfo, 10>::vf8

/// Classify one leaderboard row value into a size code.
///
/// `idx` is the row. Calls the leaderboard fetch helper (fastcall: ECX =
/// leaderboard id 0x143, EDX = scratch record) and, when it reports
/// success, passes the `idx`-th entry of the value array at record
/// offset 0x14 to the type classifier (ECX = entry). The
/// classifier's answer maps to a size: 1 -> 4, 2 -> 8, 3 -> 8, 4 -> 0,
/// 5 -> 4. A fetch failure, a classifier answer of -1 (unknown) or any
/// other answer yields 0.
///
/// Original: 0x0056ede0 (rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_120, player_schema::LeaderboardInfo, 10>::vf8; stdcall, one stack word).
lf_checker_rt::export!(stdcall, rw_0056ede0(idx: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x143;
        const VALS_WORD: usize = 5;
        let mut record = [0u32; 8];
        record[VALS_WORD] = 0;
        let ok: u8 = lf_checker_rt::callee_fastcall!(
            1, u8, LEADERBOARD_ID, record.as_mut_ptr() as u32);
        if ok == 0 {
            return 0;
        }
        let vals = record[VALS_WORD];
        let entry = (vals.wrapping_add(idx.wrapping_mul(4)) as *const u32).read();
        let kind: u32 = lf_checker_rt::callee_thiscall!(2, u32, entry);
        if kind == 0xffffffff {
            return 0;
        }
        match kind.wrapping_sub(1) {
            0 => 4,
            1 => 8,
            2 => 8,
            3 => 0,
            4 => 4,
            _ => 0,
        }
    }
});
