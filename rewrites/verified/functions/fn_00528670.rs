// original: 0x00528670 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Race5Standard, player_schema::LeaderboardInfo, 10>::vf8

/// Classify one leaderboard entry by index.
///
/// The query callee fills a scratch struct (word 5 = entry array) for
/// leaderboard id `QUERY_ID`; the entry at `index` is classified by a
/// second callee, and the class (minus one, 0..4) selects a width from a
/// five-way table: classes 1 and 5 give 4, classes 2 and 3 give 8, class
/// 4 gives 0. Returns 0 when either query fails, the class is -1, or the
/// class falls outside 1..5.
///
/// Original: stdcall with one stack word; entry registers are ignored.
/// The original dispatches through a jump table in its code section; the
/// rewrite spells the same mapping as a match.
lf_checker_rt::export!(stdcall, rw_00528670(index: u32) -> u32 {
    unsafe {
        const QUERY_ID: u32 = 0x53;
        const CALLEE_QUERY: u32 = 1;
        const CALLEE_CLASSIFY: u32 = 2;
        const WIDTH_A: u32 = 4;
        const WIDTH_B: u32 = 8;
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        let mut st = [0u32; 6];
        let r: u32 = lf_checker_rt::callee_fastcall!(CALLEE_QUERY, u32, QUERY_ID, st.as_mut_ptr() as u32);
        if (r & 0xff) == 0 {
            return 0;
        }
        let entries = st[5];
        let v = rd32(entries.wrapping_add(index.wrapping_mul(4)));
        let kind: u32 = lf_checker_rt::callee_thiscall!(CALLEE_CLASSIFY, u32, v);
        if kind == 0xffff_ffff {
            return 0;
        }
        let t = kind.wrapping_sub(1);
        if t > 4 {
            return 0;
        }
        match t {
            0 => WIDTH_A,
            1 => WIDTH_B,
            2 => WIDTH_B,
            3 => 0,
            _ => WIDTH_A,
        }
    }
});
