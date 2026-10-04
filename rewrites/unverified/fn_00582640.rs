// original: 0x00582640 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_192, player_schema::LeaderboardInfo, 10>::vf13

/// Map a value's rank on this board through the board's map array.
///
/// Searches exactly like the sibling rank lookup (count at buffer word 3,
/// base at word 4, signed bound) and, on a hit at index i, returns map[i]
/// from the parallel array at buffer word 5 instead of i. A hit at index -1
/// would return -1, but the upward scan from 0 can never produce it.
/// Returns 0xFFFF_FFFF when the service fails or nothing matches.
///
/// Original: stdcall, one stack word; one direct callee (patched).
lf_checker_rt::export!(stdcall, rw_00582640(needle: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x198;
        // Service-buffer words: entry count, value-array base, map array.
        const COUNT_WORD: usize = 3;
        const BASE_WORD: usize = 4;
        const MAP_WORD: usize = 5;
        const NOT_FOUND: u32 = 0xFFFF_FFFF;
        let mut lookup: [u32; 6] = [0; 6];
        let ok: u32 = lf_checker_rt::callee_fastcall!(1, u32, LEADERBOARD_ID, lookup.as_mut_ptr() as u32);
        if ok & 0xFF == 0 {
            return NOT_FOUND;
        }
        let count = lookup[COUNT_WORD] as i32;
        if count <= 0 {
            return NOT_FOUND;
        }
        let base = lookup[BASE_WORD];
        let map = lookup[MAP_WORD];
        let mut i: u32 = 0;
        loop {
            let v = (base.wrapping_add(i.wrapping_mul(4)) as *const u32).read_unaligned();
            if v == needle {
                if i == NOT_FOUND {
                    return NOT_FOUND;
                }
                return (map.wrapping_add(i.wrapping_mul(4)) as *const u32).read_unaligned();
            }
            i = i.wrapping_add(1);
            if (i as i32) >= count {
                return NOT_FOUND;
            }
        }
    }
});
