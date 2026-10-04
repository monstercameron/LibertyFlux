// original: 0x00552b50 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_17, player_schema::LeaderboardInfo, 10>::vf7

/// Look up one leaderboard row value by index.
///
/// Calls the leaderboard-schema helper (callee 1) with this board's schema
/// id; the helper fills a frame-local record whose value-table pointer (word
/// 4 of the record) the function then indexes with `index`. Returns the
/// selected value, or -1 when the helper reports failure. The index scales
/// by four with wraparound, so very large indexes read just before the table
/// or wrap back into it.
///
/// Original: 0x00552b50 (stdcall, one stack word).
lf_checker_rt::export!(stdcall, rw_00552b50(index: u32) -> u32 {
    unsafe {
        const SCHEMA_ID: u32 = 0xDC;
        const SCHEMA_CALLEE: u32 = 1;
        const VALUE_TABLE_SLOT: usize = 4;
        const NOT_FOUND: u32 = 0xFFFF_FFFF;
        let mut record = [0u32; 8];
        let status = lf_checker_rt::callee_fastcall!(
            SCHEMA_CALLEE, u32, SCHEMA_ID, record.as_mut_ptr() as u32);
        if status & 0xFF == 0 {
            return NOT_FOUND;
        }
        let table = record[VALUE_TABLE_SLOT] as *const u32;
        table.add(index as usize).read_unaligned()
    }
});
