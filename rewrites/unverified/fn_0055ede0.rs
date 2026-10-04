// original: 0x0055ede0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_62, player_schema::LeaderboardInfo, 10>::vf13

/// Value paired with a leaderboard key, or -1.
///
/// `key` is the value to find. Calls the leaderboard fetch helper (callee 1)
/// with leaderboard id 0x109 and a six-word scratch query; when the helper
/// reports failure the result is -1. Otherwise query word 3 holds the key
/// count, word 4 points at the key array and word 5 at the parallel value
/// array: `key` is scanned for with a signed 32-bit loop and, on a match, the
/// value at the same index is returned. A non-positive count or no match
/// yields -1. (The original compares the found index against -1 afterwards;
/// that branch is dead -- a found index is never negative -- and is not
/// mirrored.)
///
/// Original: 0x0055ede0 (stdcall, one stack word; entry registers ignored).
lf_checker_rt::export!(stdcall, rw_0055ede0(key: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x109;
        const FETCH_CALLEE: u32 = 1;
        const COUNT_WORD: usize = 3;
        const KEYS_WORD: usize = 4;
        const VALUES_WORD: usize = 5;
        const NOT_FOUND: u32 = 0xFFFF_FFFF;
        let mut query = [0u32; 6];
        let answer: u32 = lf_checker_rt::callee_fastcall!(FETCH_CALLEE, u32,
            LEADERBOARD_ID, query.as_mut_ptr() as u32);
        if answer & 0xFF == 0 {
            return NOT_FOUND;
        }
        let count = query[COUNT_WORD] as i32;
        if count <= 0 {
            return NOT_FOUND;
        }
        let keys = query[KEYS_WORD] as *const u32;
        let mut index = 0i32;
        while index < count {
            if keys.add(index as usize).read() == key {
                let values = query[VALUES_WORD] as *const u32;
                return values.add(index as usize).read();
            }
            index += 1;
        }
        NOT_FOUND
    }
});
