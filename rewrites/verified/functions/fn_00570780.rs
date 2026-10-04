// original: 0x00570780 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_126, player_schema::LeaderboardInfo, 10>::vf6

/// Find the position of a key in a leaderboard column.
///
/// `key` is the value sought. Calls the leaderboard fetch helper
/// (fastcall: ECX = leaderboard id 0x149, EDX = scratch record) and,
/// when it reports success, scans the column array the helper wrote at
/// record offset 0x10 in order and returns the first position
/// holding `key`. A non-positive row count (record offset 0xc)
/// yields 0xffffffff without reading the array; no match yields 0xffffffff.
///
/// Original: 0x00570780 (rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_126, player_schema::LeaderboardInfo, 10>::vf6; stdcall, one stack word).
lf_checker_rt::export!(stdcall, rw_00570780(key: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x149;
        const COUNT_WORD: usize = 3;
        const COLS_WORD: usize = 4;
        const NOT_FOUND: u32 = 0xffffffff;
        let mut record = [0u32; 8];
        record[COUNT_WORD] = 0;
        record[COLS_WORD] = 0;
        let ok: u8 = lf_checker_rt::callee_fastcall!(
            1, u8, LEADERBOARD_ID, record.as_mut_ptr() as u32);
        if ok == 0 {
            return NOT_FOUND;
        }
        let count = record[COUNT_WORD] as i32;
        if count <= 0 {
            return NOT_FOUND;
        }
        let cols = record[COLS_WORD];
        let mut i: u32 = 0;
        while (i as i32) < count {
            let v = (cols.wrapping_add(i.wrapping_mul(4)) as *const u32).read();
            if v == key {
                return i;
            }
            i = i.wrapping_add(1);
        }
        NOT_FOUND
    }
});
