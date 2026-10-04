// original: 0x005805a0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_184, player_schema::LeaderboardInfo, 10>::vf7

/// Leaderboard column lookup for one episodic-race leaderboard (id 0x190).
///
/// Calls the leaderboard query with this leaderboard's id and a six-word
/// scratch descriptor; the query fills words 1..=5 with `countA`, `arrA`,
/// `countB`, `arrB`, `arrC` (byte offsets +4..+20) and answers nonzero in
/// its low byte on success. Only the low byte of the answer is significant.
/// Returns `arrB[index]` with 32-bit wraparound on the scaled index.
/// Original: stdcall, one stack word, returns -1 when the query fails.
lf_checker_rt::export!(stdcall, rw_005805a0(index: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x190;
        const DESC_ARR_B: usize = 4;
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
        let arr = desc[DESC_ARR_B];
        rd32(arr.wrapping_add(index.wrapping_mul(4)))
    }
});
