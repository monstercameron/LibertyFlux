// original: 0x00535be0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Race54Standard, player_schema::LeaderboardInfo, 10>::vf6

/// Position of the first list element equal to `want`, or -1.
///
/// Fetches a witness struct through callee 1 (fastcall id `BOARD`, six
/// words: signed element count at word 3, search list at word 4). Returns
/// -1 when the fetch fails or the count is not positive; otherwise scans
/// the list with a signed bound and returns the matching position, or -1.
/// Original: thiscall, one stack word; callee-cleaned.
lf_checker_rt::export!(thiscall, rw_00535be0(_this: u32, want: u32) -> u32 {
    unsafe {
        const BOARD: u32 = 0x95;
        const NONE: u32 = 0xFFFF_FFFF;
        const ST_COUNT: usize = 3;
        const ST_LIST: usize = 4;
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
        let mut i = 0i32;
        while i < count {
            if rd(list, i as u32) == want {
                return i as u32;
            }
            i += 1;
        }
        NONE
    }});
