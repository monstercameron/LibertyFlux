// original: 0x00527f80 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Race4Standard, player_schema::LeaderboardInfo, 10>::vf13

/// Map a row id to its leaderboard value.
///
/// The query callee fills a scratch struct (word 3 = count, word 4 = id
/// array, word 5 = value array) for leaderboard id `QUERY_ID`. Returns
/// the value at the position of `needle` in the id array, or -1 when the
/// query fails, the count is not positive, or the id is absent.
///
/// Original: stdcall with one stack word; entry registers are ignored.
/// The loop bound is signed (`jle`/`jl`). After a match the original
/// compares the index against -1, which can never be equal; that dead
/// branch is not reproduced.
lf_checker_rt::export!(stdcall, rw_00527f80(needle: u32) -> u32 {
    unsafe {
        const QUERY_ID: u32 = 0x52;
        const CALLEE_QUERY: u32 = 1;
        const NOT_FOUND: u32 = 0xffff_ffff;
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        let mut st = [0u32; 6];
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
                break;
            }
            i = i.wrapping_add(1);
            if (i as i32) >= n {
                return NOT_FOUND;
            }
        }
        let values = st[5];
        rd32(values.wrapping_add(i.wrapping_mul(4)))
    }
});
