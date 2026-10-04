// original: 0x0054ea10 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_2, player_schema::LeaderboardInfo, 10>::vf8
// Entry size class, second leaderboard instantiation.
//
export!(stdcall, rs252_0054ea10(index: u32) -> u32 {
    let mut out = [0u32; 8];
    let ok: u8 = callee_fastcall!(1, u8, 0xb1, out.as_mut_ptr() as u32);
    if ok == 0 {
        return 0;
    }
    let table = out[5];
    let addr = table.wrapping_add(index.wrapping_mul(4));
    let val = unsafe { (addr as *const u32).read() };
    let kind = callee_thiscall!(2, u32, val);
    match kind {
        1 => 4,
        2 | 3 => 8,
        5 => 4,
        _ => 0,
    }
});
