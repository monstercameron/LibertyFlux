// original: 0x00582A30 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_193, player_schema::LeaderboardInfo, 10>::vf12

/// Find a leaderboard row's rank by value.
///
/// Resolves the row value table[`index`] through the leaderboard service
/// (board id in ECX, six-word scratch buffer in EDX; answers in AL; row-table
/// pointer back at buffer word 5), rejects a 0xFFFF_FFFF row, then linearly
/// scans the board's value array (count at buffer word 1, base at word 2)
/// for that value with an unsigned bound. Returns the first matching index,
/// or 0xFFFF_FFFF when the service fails, the row is -1, the count is 0, or
/// no entry matches.
///
/// Original: stdcall, one stack word; one direct callee (patched).
lf_checker_rt::export!(stdcall, rw_00582a30(index: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x199;
        // Service-buffer words: entry count, value-array base, row table.
        const COUNT_WORD: usize = 1;
        const BASE_WORD: usize = 2;
        const ROWS_WORD: usize = 5;
        const NOT_FOUND: u32 = 0xFFFF_FFFF;
        let mut lookup: [u32; 6] = [0; 6];
        let ok: u32 = lf_checker_rt::callee_fastcall!(1, u32, LEADERBOARD_ID, lookup.as_mut_ptr() as u32);
        if ok & 0xFF == 0 {
            return NOT_FOUND;
        }
        let rows = lookup[ROWS_WORD];
        let target = (rows.wrapping_add(index.wrapping_mul(4)) as *const u32).read_unaligned();
        if target == NOT_FOUND {
            return NOT_FOUND;
        }
        let count = lookup[COUNT_WORD];
        if count == 0 {
            return NOT_FOUND;
        }
        let base = lookup[BASE_WORD];
        let mut i: u32 = 0;
        loop {
            let v = (base.wrapping_add(i.wrapping_mul(4)) as *const u32).read_unaligned();
            if v == target {
                return i;
            }
            i = i.wrapping_add(1);
            if i >= count {
                return NOT_FOUND;
            }
        }
    }
});
