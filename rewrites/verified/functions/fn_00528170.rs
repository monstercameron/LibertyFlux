// original: 0x00528170 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Race4Standard, player_schema::LeaderboardInfo, 10>::vf6

/// Find a row id in the leaderboard id array.
///
/// The query callee fills a scratch struct (word 3 = count, word 4 = id
/// array) for leaderboard id `QUERY_ID`. Returns the position of `needle`
/// in the array, or -1 when the query fails, the count is not positive,
/// or the id is absent.
///
/// Original: stdcall with one stack word; entry registers are ignored.
/// Both the emptiness test and the loop bound are signed (`jle`/`jl`).
lf_checker_rt::export!(stdcall, rw_00528170(needle: u32) -> u32 {
    unsafe {
        const QUERY_ID: u32 = 0x52;
        const CALLEE_QUERY: u32 = 1;
        const NOT_FOUND: u32 = 0xffff_ffff;
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        let mut st = [0u32; 5];
        let r: u32 = lf_checker_rt::callee_fastcall!(CALLEE_QUERY, u32, QUERY_ID, st.as_mut_ptr() as u32);
        if (r & 0xff) == 0 {
            return NOT_FOUND;
        }
        let n = st[3] as i32;
        let ids = st[4];
        if n <= 0 {
            return NOT_FOUND;
        }
        let mut i = 0u32;
        loop {
            if rd32(ids.wrapping_add(i.wrapping_mul(4))) == needle {
                return i;
            }
            i = i.wrapping_add(1);
            if (i as i32) >= n {
                return NOT_FOUND;
            }
        }
    }
});
