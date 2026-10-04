// original: 0x00551340 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_12, player_schema::LeaderboardInfo, 10>::vf13

/// Map one leaderboard key to its row value.
///
/// Calls the schema helper (callee 1) with this board's schema id; the helper
/// fills a frame-local record holding a key count (word 3), a key array
/// pointer (word 4) and a value array pointer (word 5). Returns -1 when the
/// helper fails or the count is not positive; otherwise linearly scans the
/// keys for `want` (signed bound) and returns the value at the matching
/// position, or -1 when no key matches.
///
/// Original: 0x00551340 (stdcall, one stack word).
lf_checker_rt::export!(stdcall, rw_00551340(want: u32) -> u32 {
    unsafe {
        const SCHEMA_ID: u32 = 0xD7;
        const SCHEMA_CALLEE: u32 = 1;
        const COUNT_SLOT: usize = 3;
        const KEYS_SLOT: usize = 4;
        const VALUES_SLOT: usize = 5;
        const NOT_FOUND: u32 = 0xFFFF_FFFF;
        let mut record = [0u32; 8];
        let status = lf_checker_rt::callee_fastcall!(
            SCHEMA_CALLEE, u32, SCHEMA_ID, record.as_mut_ptr() as u32);
        if status & 0xFF == 0 {
            return NOT_FOUND;
        }
        let count = record[COUNT_SLOT] as i32;
        if count <= 0 {
            return NOT_FOUND;
        }
        let keys = record[KEYS_SLOT] as *const u32;
        let values = record[VALUES_SLOT] as *const u32;
        let mut i = 0i32;
        while i < count {
            if keys.add(i as usize).read_unaligned() == want {
                return values.add(i as usize).read_unaligned();
            }
            i = i.wrapping_add(1);
        }
        NOT_FOUND
    }
});
