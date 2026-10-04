// original: 0x00570520 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_126, player_schema::LeaderboardInfo, 10>::vf12

/// Find the column position of one leaderboard row value.
///
/// `idx` is the row. Calls the leaderboard fetch helper (fastcall: ECX =
/// leaderboard id 0x149, EDX = scratch record) and, when it reports
/// success, loads the row's key from the key array the helper wrote at
/// record offset 0x14. A key of 0xffffffff (empty row) yields
/// 0xffffffff without searching. Otherwise the first count words of the
/// column array at record offset 0x8 are scanned in order
/// (unsigned bound) and the first position holding the key is returned;
/// an empty table or no match yields 0xffffffff.
///
/// Original: 0x00570520 (rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_126, player_schema::LeaderboardInfo, 10>::vf12; stdcall, one stack word).
lf_checker_rt::export!(stdcall, rw_00570520(idx: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x149;
        const COUNT_WORD: usize = 1;
        const COLS_WORD: usize = 2;
        const KEYS_WORD: usize = 5;
        const EMPTY_ROW: u32 = 0xffffffff;
        const NOT_FOUND: u32 = 0xffffffff;
        let mut record = [0u32; 8];
        record[COUNT_WORD] = 0;
        record[COLS_WORD] = 0;
        record[KEYS_WORD] = 0;
        let ok: u8 = lf_checker_rt::callee_fastcall!(
            1, u8, LEADERBOARD_ID, record.as_mut_ptr() as u32);
        if ok == 0 {
            return NOT_FOUND;
        }
        let keys = record[KEYS_WORD];
        let key = (keys.wrapping_add(idx.wrapping_mul(4)) as *const u32).read();
        if key == EMPTY_ROW {
            return NOT_FOUND;
        }
        let count = record[COUNT_WORD];
        if count == 0 {
            return NOT_FOUND;
        }
        let cols = record[COLS_WORD];
        let mut i: u32 = 0;
        while i < count {
            let v = (cols.wrapping_add(i.wrapping_mul(4)) as *const u32).read();
            if v == key {
                return i;
            }
            i = i.wrapping_add(1);
        }
        NOT_FOUND
    }
});
