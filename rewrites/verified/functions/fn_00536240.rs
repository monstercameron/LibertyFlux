// original: 0x00536240 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Race56Standard, player_schema::LeaderboardInfo, 10>::vf12

/// Position of an indexed entry's value inside a second list, or -1.
///
/// Fetches a witness struct through callee 1 (fastcall id `BOARD`, frame
/// struct of six words: element count at word 1, search list at word 2,
/// indexed table at word 5). Returns -1 when the fetch fails, when the
/// table entry at `index` is -1, or when the count is zero; otherwise the
/// unsigned index of the first list element equal to that entry, or -1.
/// The table read is unchecked and faults exactly like the original's for
/// a wild index. Original: thiscall, one stack word.
lf_checker_rt::export!(thiscall, rw_00536240(_this: u32, index: u32) -> u32 {
    unsafe {
        const BOARD: u32 = 0x82;
        const NONE: u32 = 0xFFFF_FFFF;
        const ST_COUNT: usize = 1;
        const ST_LIST: usize = 2;
        const ST_TABLE: usize = 5;
        let mut st = [0u32; 6];
        let ok: u32 = lf_checker_rt::callee_fastcall!(1, u32, BOARD, st.as_mut_ptr() as u32);
        if ok & 0xFF == 0 {
            return NONE;
        }
        let v = rd(st[ST_TABLE], index);
        if v == NONE {
            return NONE;
        }
        let count = st[ST_COUNT];
        if count == 0 {
            return NONE;
        }
        let list = st[ST_LIST];
        let mut i = 0u32;
        while i < count {
            if rd(list, i) == v {
                return i;
            }
            i += 1;
        }
        NONE
    }});
