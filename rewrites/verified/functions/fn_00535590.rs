// original: 0x00535590 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Race53Standard, player_schema::LeaderboardInfo, 10>::vf13

/// Parallel-slot value for the first list element equal to `want`, or -1.
///
/// Fetches a witness struct through callee 1 (fastcall id `BOARD`, six
/// words: signed element count at word 3, search list at word 4, parallel
/// value table at word 5). Returns -1 when the fetch fails or the count is
/// not positive; otherwise scans the list with a signed bound and returns
/// the value-table entry at the matching position, or -1. (The original
/// re-checks the found position against -1 after the loop; that position
/// counts up from zero and the check cannot fire, so it is omitted.)
/// Original: thiscall, one stack word; callee-cleaned.
lf_checker_rt::export!(thiscall, rw_00535590(_this: u32, want: u32) -> u32 {
    unsafe {
        const BOARD: u32 = 0x8a;
        const NONE: u32 = 0xFFFF_FFFF;
        const ST_COUNT: usize = 3;
        const ST_LIST: usize = 4;
        const ST_VALS: usize = 5;
        let mut st = [0u32; 6];
        let ok: u32 = lf_checker_rt::callee_fastcall!(1, u32, BOARD, st.as_mut_ptr() as u32);
        if ok & 0xFF == 0 {
            return NONE;
        }
        let count = st[ST_COUNT] as i32;
        if count <= 0 {
            return NONE;
        }
        let list = st[ST_LIST];
        let vals = st[ST_VALS];
        let mut i = 0i32;
        while i < count {
            if rd(list, i as u32) == want {
                return rd(vals, i as u32);
            }
            i += 1;
        }
        NONE
    }});
