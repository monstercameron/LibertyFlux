// original: 0x00597050 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_267, player_schema::LeaderboardInfo, 10>::vf6

/// Find KEY in leaderboard table 0x1df's row-id list; return its position.
///
/// `key` is the row id to look for. The function asks the leaderboard
/// registry (callee 1) for table 0x1df, passing a scratch record in edx: on
/// success the record holds the row count (word at +0x0c) and a pointer to
/// the row-id array (word at +0x10). A zero low byte in the answer means the
/// table is unavailable and the result is NOT_FOUND (-1). Otherwise the first
/// COUNT entries are scanned in order (the count is signed: zero or negative
/// yields NOT_FOUND) and the index of the first entry equal to KEY wins; no
/// match yields NOT_FOUND.
///
/// Original: stdcall, one stack argument; the registry callee takes
/// (table id, record) in (ecx, edx) and answers in al.
lf_checker_rt::export!(stdcall, rw_00597050(key: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x1df;
        const REGISTRY_LOOKUP: u32 = 1;
        const COUNT_OFF: u32 = 0x0c;
        const LIST_OFF: u32 = 0x10;
        const NOT_FOUND: u32 = 0xffff_ffff;
        let mut record = [0u32; 5];
        let base = record.as_mut_ptr() as u32;
        let answer: u32 =
            lf_checker_rt::callee_fastcall!(REGISTRY_LOOKUP, u32, LEADERBOARD_ID, base);
        if (answer & 0xff) == 0 {
            return NOT_FOUND;
        }
        let count = (base.wrapping_add(COUNT_OFF) as *const u32).read_unaligned() as i32;
        if count <= 0 {
            return NOT_FOUND;
        }
        let list = (base.wrapping_add(LIST_OFF) as *const u32).read_unaligned();
        let mut index: i32 = 0;
        while index < count {
            let entry = (list.wrapping_add((index as u32).wrapping_mul(4)) as *const u32)
                .read_unaligned();
            if entry == key {
                return index as u32;
            }
            index = index.wrapping_add(1);
        }
        NOT_FOUND
    }
});
