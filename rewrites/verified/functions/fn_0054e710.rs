// original: 0x0054e710 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_2, player_schema::LeaderboardInfo, 10>::vf12
// Position of a row key inside a leaderboard ordering.
//
// Fetches the row table and an ordering array, reads the key of row `index`,
// and returns the position of that key in the ordering, or -1 when the
// table is unavailable, the key is the empty marker (-1), the ordering is
// empty, or the key is absent.
//
export!(stdcall, rs252_0054e710(index: u32) -> i32 {
    let mut out = [0u32; 8];
    let ok: u8 = callee_fastcall!(1, u8, 0xb1, out.as_mut_ptr() as u32);
    if ok == 0 {
        return -1;
    }
    let rows = out[5];
    let target = unsafe { (rows.wrapping_add(index.wrapping_mul(4)) as *const u32).read() };
    if target == 0xffff_ffff {
        return -1;
    }
    let count = out[1];
    if count == 0 {
        return -1;
    }
    let order = out[2];
    let mut i = 0u32;
    loop {
        let v = unsafe { (order.wrapping_add(i.wrapping_mul(4)) as *const u32).read() };
        if v == target {
            return i as i32;
        }
        i += 1;
        if i >= count {
            return -1;
        }
    }
});
