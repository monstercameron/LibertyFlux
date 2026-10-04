// original: 0x005812B0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_187, player_schema::LeaderboardInfo, 10>::vf7

/// Fetch this board's row table through the leaderboard service, then
/// return one row: a fastcall with the board id in ECX and a six-word scratch
/// buffer in EDX. The service answers true/false in AL and, on success,
/// leaves the row-table pointer at buffer word 4 (byte +0x10). Returns
/// table[`index`] (4-byte rows), or 0xFFFF_FFFF when the service fails.
///
/// Original: stdcall, one stack word; one direct callee (patched).
lf_checker_rt::export!(stdcall, rw_005812b0(index: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x193;
        // Service-buffer word holding the row-table pointer (byte +0x10).
        const ROWS_WORD: usize = 4;
        const LOOKUP_FAILED: u32 = 0xFFFF_FFFF;
        let mut lookup: [u32; 6] = [0; 6];
        let ok: u32 = lf_checker_rt::callee_fastcall!(1, u32, LEADERBOARD_ID, lookup.as_mut_ptr() as u32);
        if ok & 0xFF == 0 {
            return LOOKUP_FAILED;
        }
        let rows = lookup[ROWS_WORD];
        (rows.wrapping_add(index.wrapping_mul(4)) as *const u32).read_unaligned()
    }
});
