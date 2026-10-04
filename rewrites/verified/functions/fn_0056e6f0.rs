// original: 0x0056e6f0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_119, player_schema::LeaderboardInfo, 10>::vf13

/// Map a leaderboard key to its paired value.
///
/// `key` is the value sought. Calls the leaderboard fetch helper
/// (fastcall: ECX = leaderboard id 0x142, EDX = scratch record) and,
/// when it reports success, scans the key array the helper wrote at
/// record offset 0x10 in order; at the first position holding
/// `key` the word at the same position of the value array (record
/// offset 0x14) is returned. A non-positive row count (record
/// offset 0xc), a fetch failure or no match yields 0xffffffff.
/// (The original re-checks the found position against -1; it cannot be
/// -1 since the scan starts at 0, so the rewrite omits that dead check.)
///
/// Original: 0x0056e6f0 (rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_119, player_schema::LeaderboardInfo, 10>::vf13; stdcall, one stack word).
lf_checker_rt::export!(stdcall, rw_0056e6f0(key: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x142;
        const COUNT_WORD: usize = 3;
        const KEYS_WORD: usize = 4;
        const VALS_WORD: usize = 5;
        const NOT_FOUND: u32 = 0xffffffff;
        let mut record = [0u32; 8];
        record[COUNT_WORD] = 0;
        record[KEYS_WORD] = 0;
        record[VALS_WORD] = 0;
        let ok: u8 = lf_checker_rt::callee_fastcall!(
            1, u8, LEADERBOARD_ID, record.as_mut_ptr() as u32);
        if ok == 0 {
            return NOT_FOUND;
        }
        let count = record[COUNT_WORD] as i32;
        if count <= 0 {
            return NOT_FOUND;
        }
        let keys = record[KEYS_WORD];
        let mut i: u32 = 0;
        while (i as i32) < count {
            let v = (keys.wrapping_add(i.wrapping_mul(4)) as *const u32).read();
            if v == key {
                let vals = record[VALS_WORD];
                return (vals.wrapping_add(i.wrapping_mul(4)) as *const u32).read();
            }
            i = i.wrapping_add(1);
        }
        NOT_FOUND
    }
});
