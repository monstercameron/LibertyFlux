// original: 0x00534240 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Race48Standard, player_schema::LeaderboardInfo, 10>::vf8
/// Ranked-race leaderboard column classifier: look the column key up in this
/// board's table, ask the shared kind helper to classify it, then map the
/// kind to a width (4 or 8 bytes) or 0 when unknown or unavailable.
lf_checker_rt::export!(thiscall, rw_00534240(_this: u32, index: u32) -> u32 {
    /// Board id this instantiation passes to the shared helper.
    const BOARD_ID: u32 = 0x92;
    /// Slot of the helper's out-block holding the column-table pointer.
    const TABLE_SLOT: usize = 5;
    let mut out = [0u32; 6];
    let ok: u32 = lf_checker_rt::callee_fastcall!(1, u32, BOARD_ID, out.as_mut_ptr() as u32);
    if ok & 0xFF == 0 {
        return 0;
    }
    let key = unsafe {
        (out[TABLE_SLOT].wrapping_add(index.wrapping_mul(4)) as *const u32).read()
    };
    let kind: u32 = lf_checker_rt::callee_thiscall!(2, u32, key);
    match kind {
        1 => 4,
        2 | 3 => 8,
        5 => 4,
        _ => 0,
    }
});
