// original: 0x005279c0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Race2Standard, player_schema::LeaderboardInfo, 10>::vf9

/// Rank one leaderboard entry by index.
///
/// The query callee fills a scratch struct (word 5 = entry array) for
/// leaderboard id `QUERY_ID`; the entry at `index` is classified by a
/// second callee, and the class (minus one, 0..4) selects a rank from a
/// five-way table: 0, 1, 3, -1, 2. Returns -1 when either query fails,
/// the class is -1, or the class falls outside 1..5.
///
/// Original: stdcall with one stack word; entry registers are ignored.
/// The original dispatches through a jump table in its code section; the
/// rewrite spells the same mapping as a match.
lf_checker_rt::export!(stdcall, rw_005279c0(index: u32) -> u32 {
    unsafe {
        const QUERY_ID: u32 = 0x50;
        const CALLEE_QUERY: u32 = 1;
        const CALLEE_CLASSIFY: u32 = 2;
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
        let entries = st[5];
        let v = rd32(entries.wrapping_add(index.wrapping_mul(4)));
        let kind: u32 = lf_checker_rt::callee_thiscall!(CALLEE_CLASSIFY, u32, v);
        if kind == NOT_FOUND {
            return NOT_FOUND;
        }
        let t = kind.wrapping_sub(1);
        if t > 4 {
            return NOT_FOUND;
        }
        match t {
            0 => 0,
            1 => 1,
            2 => 3,
            3 => NOT_FOUND,
            _ => 2,
        }
    }
});
