// original: 0x00536540 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Race56Standard, player_schema::LeaderboardInfo, 10>::vf8

/// Small-integer code for the kind of the table entry at `index`.
///
/// Fetches a witness struct through callee 1 (fastcall id `BOARD`, six
/// words: table pointer at word 5), passes the indexed entry to callee 2
/// (thiscall, no stack words) and maps its answer: -1 or a fetch failure
/// yields 0, otherwise the answer minus one selects one of five
/// arms (4, 8, 8, 0, 4); anything else yields
/// 0. The indexed read is unchecked and faults exactly like
/// the original's for a wild index. Original: thiscall, one stack word.
/// (The original dispatches through a five-entry jump table; the mapping
/// above is that table's content, decoded per member and asserted equal.)
lf_checker_rt::export!(thiscall, rw_00536540(_this: u32, index: u32) -> u32 {
    unsafe {
        const BOARD: u32 = 0x82;
        const FAIL: u32 = 0;
        const ST_TABLE: usize = 5;
        let mut st = [0u32; 6];
        let ok: u32 = lf_checker_rt::callee_fastcall!(1, u32, BOARD, st.as_mut_ptr() as u32);
        if ok & 0xFF == 0 {
            return FAIL;
        }
        let v = rd(st[ST_TABLE], index);
        let kind: u32 = lf_checker_rt::callee_thiscall!(2, u32, v);
        if kind == 0xFFFF_FFFF {
            return FAIL;
        }
        match kind.wrapping_sub(1) {
    0 => 4,
    1 => 8,
    2 => 8,
    3 => 0,
    4 => 4,
            _ => FAIL,
        }
    }});
