// original: 0x0054f040 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_4, player_schema::LeaderboardInfo, 10>::vf13
// Rank value by key, fourth leaderboard instantiation.
//
export!(stdcall, rs252_0054f040(target: u32) -> i32 {
    let mut out = [0u32; 8];
    let ok: u8 = callee_fastcall!(1, u8, 0xb3, out.as_mut_ptr() as u32);
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
    let values = out[5];
    unsafe { (values.wrapping_add(i.wrapping_mul(4)) as *const i32).read() }
});
