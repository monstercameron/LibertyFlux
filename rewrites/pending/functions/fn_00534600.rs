// original: 0x00534600 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Race49Standard, player_schema::LeaderboardInfo, 10>::vf6
/// Ranked-race leaderboard index search: return the position of `key` in
/// this board's row list, or -1 when the board is unavailable, the list is
/// empty or the key is absent.
lf_checker_rt::export!(thiscall, rw_00534600(_this: u32, key: u32) -> u32 {
    /// Board id this instantiation passes to the shared helper.
    const BOARD_ID: u32 = 0x99;
    /// Slots of the helper's out-block: row count and row list.
    const COUNT_SLOT: usize = 3;
    const ROWS_SLOT: usize = 4;
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
    let mut i = 0i32;
    while i < count {
        let v = unsafe {
            (rows.wrapping_add((i as u32).wrapping_mul(4)) as *const u32).read()
        };
        if v == key {
            return i as u32;
        }
        i += 1;
    }
    0xFFFF_FFFF
});
