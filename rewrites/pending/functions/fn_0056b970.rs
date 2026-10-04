// original: 0x0056b970 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_108, player_schema::LeaderboardInfo, 10>::vf8
/// Column-width query for one ranked leaderboard (`vf8 Race108`).
///
/// Fetches the tables, loads the key at `index`, classifies it through the
/// shared classify step, and maps the class to a width: class 1 takes 4,
/// classes 2-3 take 8, class 5 takes 4, anything else (including failure or
/// an unknown class) takes 0.
/// Leaderboard id: 0x137.
export!(stdcall, rw_0056b970(index: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x137;
        let mut frame = [0u32; 6];
        let fetch: extern "fastcall" fn(u32, u32) -> u32 =
            core::mem::transmute(callee_addr(1) as usize);
        if (fetch(LEADERBOARD_ID, frame.as_mut_ptr() as u32) & 0xFF) == 0 {
            return 0;
        }
        let keys = frame.as_ptr().add(5).read();
        let key = (keys.wrapping_add(index.wrapping_mul(4)) as *const u32).read();
        let classify: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(callee_addr(2) as usize);
        let class = classify(key);
        if class == 0xFFFF_FFFF {
            return 0;
        }
        match class.wrapping_sub(1) {
            0 => 4,
            1 | 2 => 8,
            3 => 0,
            4 => 4,
            _ => 0,
        }
    }
});
