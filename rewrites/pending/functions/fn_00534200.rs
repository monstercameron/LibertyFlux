// original: 0x00534200 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Race48Standard, player_schema::LeaderboardInfo, 10>::vf7
/// Ranked-race leaderboard cell lookup: ask the shared board helper for this
/// board's column table, then return the entry at `index`, or -1 when the
/// board reports itself unavailable.
lf_checker_rt::export!(thiscall, rw_00534200(_this: u32, index: u32) -> u32 {
    /// Board id this instantiation passes to the shared helper.
    const BOARD_ID: u32 = 0x92;
    /// Slot of the helper's out-block holding the column-table pointer.
    const TABLE_SLOT: usize = 4;
    let mut out = [0u32; 6];
    let ok: u32 = lf_checker_rt::callee_fastcall!(1, u32, BOARD_ID, out.as_mut_ptr() as u32);
    if ok & 0xFF == 0 {
        return 0xFFFF_FFFF;
    }
    let cell = out[TABLE_SLOT].wrapping_add(index.wrapping_mul(4));
    unsafe { (cell as *const u32).read() }
});
