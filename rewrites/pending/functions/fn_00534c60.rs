// original: 0x00534C60 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Race51Standard, player_schema::LeaderboardInfo, 10>::vf12
/// Ranked-race leaderboard reverse lookup: resolve `index` to its column key
/// through this board's table, then return the key's position in the board's
/// row list, or -1 when unavailable, empty or absent.
lf_checker_rt::export!(thiscall, rw_00534C60(_this: u32, index: u32) -> u32 {
    /// Board id this instantiation passes to the shared helper.
    const BOARD_ID: u32 = 0x91;
    /// Slots of the helper's out-block: row count, row list, column table.
    const COUNT_SLOT: usize = 1;
    const ROWS_SLOT: usize = 2;
    const TABLE_SLOT: usize = 5;
    let mut out = [0u32; 6];
    let ok: u32 = lf_checker_rt::callee_fastcall!(1, u32, BOARD_ID, out.as_mut_ptr() as u32);
    if ok & 0xFF == 0 {
        return 0xFFFF_FFFF;
    }
    let key = unsafe {
        (out[TABLE_SLOT].wrapping_add(index.wrapping_mul(4)) as *const u32).read()
    };
    if key == 0xFFFF_FFFF {
        return 0xFFFF_FFFF;
    }
    let count = out[COUNT_SLOT];
    if count == 0 {
        return 0xFFFF_FFFF;
    }
    let rows = out[ROWS_SLOT];
    let mut i = 0u32;
    while i < count {
        let v = unsafe { (rows.wrapping_add(i.wrapping_mul(4)) as *const u32).read() };
        if v == key {
            return i;
        }
        i = i.wrapping_add(1);
    }
    0xFFFF_FFFF
});
