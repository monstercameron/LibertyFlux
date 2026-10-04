// original: 0x005274b0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Race1Standard, player_schema::LeaderboardInfo, 10>::vf7

/// Read one leaderboard value by index.
///
/// The query callee fills a scratch struct (word 4 = value array) for
/// leaderboard id `QUERY_ID`. Returns the array entry at `index`, or -1
/// when the query fails. There is no bounds check; the caller supplies a
/// valid index.
///
/// Original: stdcall with one stack word; entry registers are ignored.
lf_checker_rt::export!(stdcall, rw_005274b0(index: u32) -> u32 {
    unsafe {
        const QUERY_ID: u32 = 0x4f;
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
        let values = st[4];
        rd32(values.wrapping_add(index.wrapping_mul(4)))
    }
});
