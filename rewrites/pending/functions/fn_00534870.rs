// original: 0x00534870 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Race50Standard, player_schema::LeaderboardInfo, 10>::vf13
/// Ranked-race leaderboard mapped lookup: find `key` in this board's row
/// list, then return the entry at that position from the board's value
/// table, or -1 when unavailable, empty or absent.
lf_checker_rt::export!(thiscall, rw_00534870(_this: u32, key: u32) -> u32 {
    /// Board id this instantiation passes to the shared helper.
    const BOARD_ID: u32 = 0x8e;
    /// Slots of the helper's out-block: row count, row list, value table.
    const COUNT_SLOT: usize = 3;
    const ROWS_SLOT: usize = 4;
    const TABLE_SLOT: usize = 5;
    let mut out = [0u32; 6];
    let ok: u32 = lf_checker_rt::callee_fastcall!(1, u32, BOARD_ID, out.as_mut_ptr() as u32);
    if ok & 0xFF == 0 {
        return 0xFFFF_FFFF;
    }
    let count = out[COUNT_SLOT] as i32;
    if count <= 0 {
        return 0xFFFF_FFFF;
    }
    let rows = out[ROWS_SLOT];
    let table = out[TABLE_SLOT];
    let mut i = 0i32;
    while i < count {
        let v = unsafe {
            (rows.wrapping_add((i as u32).wrapping_mul(4)) as *const u32).read()
        };
        if v == key {
            return unsafe {
                (table.wrapping_add((i as u32).wrapping_mul(4)) as *const u32).read()
            };
        }
        i += 1;
    }
    0xFFFF_FFFF
});
