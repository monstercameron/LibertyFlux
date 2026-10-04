// original: 0x00551e80 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_14, player_schema::LeaderboardInfo, 10>::vf8

/// Classify one leaderboard row value through the tag-size table.
///
/// Looks up the value exactly like the sibling index function (schema helper
/// call with this board's schema id, value-table pointer in word 5 of the
/// frame record, indexed by `index`), then passes it to the tag classifier
/// (callee 2). The classifier's answer maps to a storage size: 1 and 5 mean
/// four bytes, 2 and 3 mean eight bytes, anything else (including the -1
/// failure and out-of-range answers) means zero.
///
/// Original: 0x00551e80 (stdcall, one stack word).
lf_checker_rt::export!(stdcall, rw_00551e80(index: u32) -> u32 {
    unsafe {
        const SCHEMA_ID: u32 = 0xD9;
        const SCHEMA_CALLEE: u32 = 1;
        const TAG_CALLEE: u32 = 2;
        const VALUE_TABLE_SLOT: usize = 5;
        const NOT_FOUND: u32 = 0xFFFF_FFFF;
        let mut record = [0u32; 8];
        let status = lf_checker_rt::callee_fastcall!(
            SCHEMA_CALLEE, u32, SCHEMA_ID, record.as_mut_ptr() as u32);
        if status & 0xFF == 0 {
            return 0;
        }
        let table = record[VALUE_TABLE_SLOT] as *const u32;
        let tag = table.add(index as usize).read_unaligned();
        let kind = lf_checker_rt::callee_thiscall!(TAG_CALLEE, u32, tag);
        match kind {
            1 | 5 => 4,
            2 | 3 => 8,
            _ => 0,
        }
    }
});
