// original: 0x00596C50 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_266, player_schema::LeaderboardInfo, 10>::vf7

/// Return the INDEX-th row id of leaderboard table 0x1de.
///
/// `index` selects a row. The function asks the leaderboard registry
/// (callee 1) for table 0x1de, passing a scratch record in edx: on success the
/// record's word at +0x10 points at the row-id array. A zero low byte in the
/// answer yields -1. The index is not bounds-checked: a wild index reads (or
/// faults) exactly as the original does.
///
/// Original: stdcall, one stack argument; the registry callee takes
/// (table id, record) in (ecx, edx) and answers in al.
lf_checker_rt::export!(stdcall, rw_00596c50(index: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x1de;
        const REGISTRY_LOOKUP: u32 = 1;
        const LIST_OFF: u32 = 0x10;
        const NOT_FOUND: u32 = 0xffff_ffff;
        let mut record = [0u32; 5];
        let base = record.as_mut_ptr() as u32;
        let answer: u32 =
            lf_checker_rt::callee_fastcall!(REGISTRY_LOOKUP, u32, LEADERBOARD_ID, base);
        if (answer & 0xff) == 0 {
            return NOT_FOUND;
        }
        let list = (base.wrapping_add(LIST_OFF) as *const u32).read_unaligned();
        (list.wrapping_add(index.wrapping_mul(4)) as *const u32).read_unaligned()
    }
});
