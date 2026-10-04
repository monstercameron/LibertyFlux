// original: 0x0054e780 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_2, player_schema::LeaderboardInfo, 10>::vf13
// Rank value of a leaderboard entry by key.
//
// Fetches the key table and a parallel value table, finds `target` in the
// keys, and returns the value at the same position, or -1 when the table is
// unavailable, empty, or holds no such key.
//
export!(stdcall, rs252_0054e780(target: u32) -> i32 {
    let mut out = [0u32; 8];
    let ok: u8 = callee_fastcall!(1, u8, 0xb1, out.as_mut_ptr() as u32);
    if ok == 0 {
        return -1;
    }
    let count = out[3] as i32;
    if count <= 0 {
        return -1;
    }
    let keys = out[4];
    let mut i = 0u32;
    loop {
        let v = unsafe { (keys.wrapping_add(i.wrapping_mul(4)) as *const u32).read() };
        if v == target {
            break;
        }
        i += 1;
        if (i as i32) >= count {
            return -1;
        }
    }
    // The original re-checks the found index against -1 here; that branch
    // is dead (the index is always >= 0), so only the live path remains.
    let values = out[5];
    unsafe { (values.wrapping_add(i.wrapping_mul(4)) as *const i32).read() }
});
