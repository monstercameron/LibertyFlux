// original: 0x0055dbe0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_58, player_schema::LeaderboardInfo, 10>::vf12

/// Row index of an ordered leaderboard value, or -1.
///
/// `index` selects an entry of the order array. Calls the leaderboard fetch
/// helper (callee 1) with leaderboard id 0x105 and a seven-word scratch
/// query; when the helper reports failure the result is -1. Otherwise query
/// word 1 holds the key count, word 2 points at the key array and word 5 at
/// the order array: the order entry at `index` is looked up in the key array
/// and its first matching index returned. An order entry of -1, a zero count
/// or no match all yield -1. The scan is an unsigned 32-bit loop.
///
/// Original: 0x0055dbe0 (stdcall, one stack word; entry registers ignored).
lf_checker_rt::export!(stdcall, rw_0055dbe0(index: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x105;
        const FETCH_CALLEE: u32 = 1;
        const COUNT_WORD: usize = 1;
        const KEYS_WORD: usize = 2;
        const ORDER_WORD: usize = 5;
        const NOT_FOUND: u32 = 0xFFFF_FFFF;
        let mut query = [0u32; 7];
        let answer: u32 = lf_checker_rt::callee_fastcall!(FETCH_CALLEE, u32,
            LEADERBOARD_ID, query.as_mut_ptr() as u32);
        if answer & 0xFF == 0 {
            return NOT_FOUND;
        }
        let order = query[ORDER_WORD] as *const u32;
        let want = order.add(index as usize).read();
        if want == NOT_FOUND {
            return NOT_FOUND;
        }
        let count = query[COUNT_WORD];
        if count == 0 {
            return NOT_FOUND;
        }
        let keys = query[KEYS_WORD] as *const u32;
        let mut found = 0u32;
        while found < count {
            if keys.add(found as usize).read() == want {
                return found;
            }
            found = found.wrapping_add(1);
        }
        NOT_FOUND
    }
});
