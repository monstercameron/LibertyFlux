// original: 0x005816B0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_188, player_schema::LeaderboardInfo, 10>::vf6

/// Find a value's rank on this board.
///
/// Fetches the board's value array through the leaderboard service (board id
/// in ECX, six-word scratch buffer in EDX; answers in AL; count back at
/// buffer word 3, base at word 4), then linearly scans base[0..count] for
/// `needle` with a signed bound (a count at or below 0 finds nothing).
/// Returns the first matching index, or 0xFFFF_FFFF when the service fails
/// or no entry matches.
///
/// Original: stdcall, one stack word; one direct callee (patched).
lf_checker_rt::export!(stdcall, rw_005816b0(needle: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x194;
        // Service-buffer words: entry count, value-array base.
        const COUNT_WORD: usize = 3;
        const BASE_WORD: usize = 4;
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
        let mut i: u32 = 0;
        loop {
            let v = (base.wrapping_add(i.wrapping_mul(4)) as *const u32).read_unaligned();
            if v == needle {
                return i;
            }
            i = i.wrapping_add(1);
            if (i as i32) >= count {
                return NOT_FOUND;
            }
        }
    }
});
