// original: 0x005975C0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_268, player_schema::LeaderboardInfo, 10>::vf9

/// Rank the INDEX-th row of leaderboard table 0x1e0.
///
/// `index` selects a row. The function asks the leaderboard registry
/// (callee 1) for table 0x1e0, passing a scratch record in edx: on success the
/// record's word at +0x14 points at the row-id array. The INDEX-th row id
/// (unchecked) goes to the ranker (callee 2, id in ecx), whose answer 1..5
/// maps to {1: 0, 2: 1, 3: 3, 4: -1, 5: 2}. Anything else (registry failure,
/// answer -1, out of range) yields -1.
///
/// Original: stdcall, one stack argument; the registry callee takes
/// (table id, record) in (ecx, edx) and answers in al.
lf_checker_rt::export!(stdcall, rw_005975c0(index: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x1e0;
        const REGISTRY_LOOKUP: u32 = 1;
        const RANK: u32 = 2;
        const LIST_OFF: u32 = 0x14;
        const NOT_FOUND: u32 = 0xffff_ffff;
        let mut record = [0u32; 6];
        let base = record.as_mut_ptr() as u32;
        let answer: u32 =
            lf_checker_rt::callee_fastcall!(REGISTRY_LOOKUP, u32, LEADERBOARD_ID, base);
        if (answer & 0xff) == 0 {
            return NOT_FOUND;
        }
        let list = (base.wrapping_add(LIST_OFF) as *const u32).read_unaligned();
        let row = (list.wrapping_add(index.wrapping_mul(4)) as *const u32).read_unaligned();
        let judged: u32 = lf_checker_rt::callee_thiscall!(RANK, u32, row);
        if judged == NOT_FOUND {
            return NOT_FOUND;
        }
        match judged.wrapping_sub(1) {
            0 => 0, 1 => 1, 2 => 3, 3 => NOT_FOUND, 4 => 2,
            _ => NOT_FOUND,
        }
    }
});
