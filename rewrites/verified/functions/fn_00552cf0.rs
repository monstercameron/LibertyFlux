// original: 0x00552cf0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_18, player_schema::LeaderboardInfo, 10>::vf12

/// Find the position of one leaderboard row value among the board's keys.
///
/// Calls the schema helper (callee 1) with this board's schema id; the helper
/// fills a frame-local record holding a key count (word 1), a key array
/// pointer (word 2) and a value array pointer (word 5). The function reads
/// the value at `index` (wrapping index arithmetic, as in the sibling lookup)
/// and returns -1 when the helper fails or that value is -1 itself; otherwise
/// it linearly scans the keys (unsigned bound) and returns the first position
/// holding the value, or -1 when no key matches.
///
/// Original: 0x00552cf0 (stdcall, one stack word).
lf_checker_rt::export!(stdcall, rw_00552cf0(index: u32) -> u32 {
    unsafe {
        const SCHEMA_ID: u32 = 0xDD;
        const SCHEMA_CALLEE: u32 = 1;
        const COUNT_SLOT: usize = 1;
        const KEYS_SLOT: usize = 2;
        const VALUES_SLOT: usize = 5;
        const NOT_FOUND: u32 = 0xFFFF_FFFF;
        let mut record = [0u32; 8];
        let status = lf_checker_rt::callee_fastcall!(
            SCHEMA_CALLEE, u32, SCHEMA_ID, record.as_mut_ptr() as u32);
        if status & 0xFF == 0 {
            return NOT_FOUND;
        }
        let count = record[COUNT_SLOT];
        let keys = record[KEYS_SLOT] as *const u32;
        let values = record[VALUES_SLOT] as *const u32;
        let target = values.add(index as usize).read_unaligned();
        if target == NOT_FOUND {
            return NOT_FOUND;
        }
        let mut i = 0u32;
        while i < count {
            if keys.add(i as usize).read_unaligned() == target {
                return i;
            }
            i = i.wrapping_add(1);
        }
        NOT_FOUND
    }
});
