// original: 0x00536500 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Race56Standard, player_schema::LeaderboardInfo, 10>::vf7

/// Table entry at `index`, or -1 when the fetch fails.
///
/// Fetches a witness struct through callee 1 (fastcall id `BOARD`, six
/// words: table pointer at word 4). The indexed read is unchecked and
/// faults exactly like the original's for a wild index. Original:
/// thiscall, one stack word; callee-cleaned.
lf_checker_rt::export!(thiscall, rw_00536500(_this: u32, index: u32) -> u32 {
    unsafe {
        const BOARD: u32 = 0x82;
        const NONE: u32 = 0xFFFF_FFFF;
        const ST_TABLE: usize = 4;
        let mut st = [0u32; 6];
        let ok: u32 = lf_checker_rt::callee_fastcall!(1, u32, BOARD, st.as_mut_ptr() as u32);
        if ok & 0xFF == 0 {
            return NONE;
        }
        let table = st[ST_TABLE];
        rd(table, index)
    }});
