// original: 0x00594B60 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_259, player_schema::LeaderboardInfo, 10>::vf13

/// Map KEY through leaderboard table 0x1d7 to its second-table entry.
///
/// `key` is the row id to look for. The function asks the leaderboard
/// registry (callee 1) for table 0x1d7, passing a scratch record in edx: on
/// success the record holds the row count (word at +0x0c), the row-id array
/// (word at +0x10) and a second table (word at +0x14). A zero low byte in the
/// answer means the table is unavailable and the result is NOT_FOUND (-1).
/// Otherwise the first COUNT row ids are scanned in order (the count is
/// signed: zero or negative yields NOT_FOUND); on a match at index I the
/// result is the second table's I-th entry. The original re-checks the found
/// index against -1 before indexing the second table; that check cannot fail
/// (a found index is never negative) and is mirrored here.
///
/// Original: stdcall, one stack argument; the registry callee takes
/// (table id, record) in (ecx, edx) and answers in al.
lf_checker_rt::export!(stdcall, rw_00594b60(key: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x1d7;
        const REGISTRY_LOOKUP: u32 = 1;
        const COUNT_OFF: u32 = 0x0c;
        const LIST_OFF: u32 = 0x10;
        const TABLE2_OFF: u32 = 0x14;
        const NOT_FOUND: u32 = 0xffff_ffff;
        let mut record = [0u32; 6];
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
        let mut found: i32 = -1;
        while index < count {
            let entry = (list.wrapping_add((index as u32).wrapping_mul(4)) as *const u32)
                .read_unaligned();
            if entry == key {
                found = index;
                break;
            }
            index = index.wrapping_add(1);
        }
        if found == -1 {
            return NOT_FOUND;
        }
        let table = (base.wrapping_add(TABLE2_OFF) as *const u32).read_unaligned();
        (table.wrapping_add((found as u32).wrapping_mul(4)) as *const u32).read_unaligned()
    }
});
