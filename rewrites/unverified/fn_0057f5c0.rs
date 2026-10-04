// original: 0x0057f5c0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_181, player_schema::LeaderboardInfo, 10>::vf12

/// Leaderboard column lookup for one episodic-race leaderboard (id 0x18d).
///
/// Calls the leaderboard query with this leaderboard's id and a six-word
/// scratch descriptor; the query fills words 1..=5 with `countA`, `arrA`,
/// `countB`, `arrB`, `arrC` (byte offsets +4..+20) and answers nonzero in
/// its low byte on success. Only the low byte of the answer is significant.
/// Reads `v = arrC[index]` (32-bit wraparound); -1 there means
/// not found. Otherwise searches `arrA[0..countA]` for `v` and
/// returns the first matching index, or -1. The count is compared
/// UNSIGNED, so a large count reads past the array.
/// Original: stdcall, one stack word, returns -1 when the query fails.
lf_checker_rt::export!(stdcall, rw_0057f5c0(index: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x18d;
        const DESC_COUNT_A: usize = 1;
        const DESC_ARR_A: usize = 2;
        const DESC_ARR_C: usize = 5;
        const CALLEE_QUERY: u32 = 1;
        const NOT_FOUND: u32 = 0xffff_ffff;
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        let mut desc = [0u32; 6];
        let ok: u32 = lf_checker_rt::callee_fastcall!(CALLEE_QUERY, u32, LEADERBOARD_ID, desc.as_mut_ptr() as u32);
        if ok & 0xFF == 0 {
            return NOT_FOUND;
        }
        let v = rd32(desc[DESC_ARR_C].wrapping_add(index.wrapping_mul(4)));
        if v == NOT_FOUND {
            return NOT_FOUND;
        }
        let count = desc[DESC_COUNT_A];
        if count == 0 {
            return NOT_FOUND;
        }
        let arr = desc[DESC_ARR_A];
        let mut i: u32 = 0;
        loop {
            if rd32(arr.wrapping_add(i.wrapping_mul(4))) == v {
                return i;
            }
            i = i.wrapping_add(1);
            if i >= count {
                return NOT_FOUND;
            }
        }
    }
});
