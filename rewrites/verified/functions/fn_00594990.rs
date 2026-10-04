// original: 0x00594990 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_258, player_schema::LeaderboardInfo, 10>::vf8

/// Classify the INDEX-th row of leaderboard table 0x15d.
///
/// `index` selects a row. The function asks the leaderboard registry
/// (callee 1) for table 0x15d, passing a scratch record in edx: on success the
/// record's word at +0x14 points at the row-id array. The INDEX-th row id
/// (unchecked) goes to the classifier (callee 2, id in ecx), whose answer
/// 1..5 maps to {1: 4, 2: 8, 3: 8, 4: 0, 5: 4}. Anything else (registry
/// failure, answer -1, out of range) yields 0.
///
/// Original: stdcall, one stack argument; the registry callee takes
/// (table id, record) in (ecx, edx) and answers in al.
lf_checker_rt::export!(stdcall, rw_00594990(index: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x15d;
        const REGISTRY_LOOKUP: u32 = 1;
        const CLASSIFY: u32 = 2;
        const LIST_OFF: u32 = 0x14;
        let mut record = [0u32; 6];
        let base = record.as_mut_ptr() as u32;
        let answer: u32 =
            lf_checker_rt::callee_fastcall!(REGISTRY_LOOKUP, u32, LEADERBOARD_ID, base);
        if (answer & 0xff) == 0 {
            return 0;
        }
        let list = (base.wrapping_add(LIST_OFF) as *const u32).read_unaligned();
        let row = (list.wrapping_add(index.wrapping_mul(4)) as *const u32).read_unaligned();
        let judged: u32 = lf_checker_rt::callee_thiscall!(CLASSIFY, u32, row);
        if judged == 0xffff_ffff {
            return 0;
        }
        match judged.wrapping_sub(1) {
            0 => 4, 1 => 8, 2 => 8, 3 => 0, 4 => 4,
            _ => 0,
        }
    }
});
