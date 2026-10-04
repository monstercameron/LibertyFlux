// original: 0x0054e570 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_1, player_schema::LeaderboardInfo, 10>::vf7
// Row lookup in a leaderboard table.
//
// Fetches the table for this leaderboard, then returns the entry at `index`.
// Returns -1 when the helper reports the table is unavailable.
//
export!(stdcall, rs252_0054e570(index: u32) -> i32 {
    let mut out = [0u32; 8];
    let ok: u8 = callee_fastcall!(1, u8, 0xb0, out.as_mut_ptr() as u32);
    if ok == 0 {
        return -1;
    }
    let table = out[4];
    let addr = table.wrapping_add(index.wrapping_mul(4));
    unsafe { (addr as *const i32).read() }
});
