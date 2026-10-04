// original: 0x00522a60 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Race44NoHolds, player_schema::LeaderboardInfo, 10>::vf7

/// Read one leaderboard value by position.
///
/// `value_index` selects an entry of the value list. The lookup callee
/// (id 1, leaderboard id 0x19) fills the value-list pointer at word 4 of a
/// five-word query block; only its low result byte matters. Returns the
/// entry, or all-bits-set when the lookup fails.
/// Original: stdcall, one stack word.
lf_checker_rt::export!(stdcall, rw_00522a60(value_index: u32) -> u32 {
    unsafe {
        const LOOKUP_CALLEE: u32 = 1;
        const LEADERBOARD_ID: u32 = 0x19;
        const NOT_FOUND: u32 = 0xFFFF_FFFF;

        let mut block = [0u32; 5];
        let answer: u32 = lf_checker_rt::callee_fastcall!(
            LOOKUP_CALLEE,
            u32,
            LEADERBOARD_ID,
            block.as_mut_ptr() as u32
        );
        if answer & 0xFF == 0 {
            return NOT_FOUND;
        }
        (block[4].wrapping_add(value_index.wrapping_mul(4)) as *const u32).read_unaligned()
    }
});
