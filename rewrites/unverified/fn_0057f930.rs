// original: 0x0057f930 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_181, player_schema::LeaderboardInfo, 10>::vf9

/// Leaderboard column lookup for one episodic-race leaderboard (id 0x18d).
///
/// Calls the leaderboard query with this leaderboard's id and a six-word
/// scratch descriptor; the query fills words 1..=5 with `countA`, `arrA`,
/// `countB`, `arrB`, `arrC` (byte offsets +4..+20) and answers nonzero in
/// its low byte on success. Only the low byte of the answer is significant.
/// Reads `v = arrC[index]`, classifies it through a second query
/// (single register argument), and maps the answer: 1 -> 0, 2 -> 1,
/// 3 -> 3, 4 -> -1, 5 -> 2, anything else (including -1) -> -1.
/// Original: stdcall, one stack word, returns -1 when the query fails.
lf_checker_rt::export!(stdcall, rw_0057f930(index: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x18d;
        const DESC_ARR_C: usize = 5;
        const CALLEE_QUERY: u32 = 1;
        const CALLEE_CLASSIFY: u32 = 2;
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
        let r: u32 = lf_checker_rt::callee_thiscall!(CALLEE_CLASSIFY, u32, v);
        if r == NOT_FOUND {
            return NOT_FOUND;
        }
        let k = r.wrapping_sub(1);
        if k > 4 {
            return NOT_FOUND;
        }
        match k {
            0 => 0,
            1 => 1,
            2 => 3,
            3 => NOT_FOUND,
            _ => 2,
        }
    }
});
