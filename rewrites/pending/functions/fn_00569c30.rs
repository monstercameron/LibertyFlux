// original: 0x569c30 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_102, player_schema::LeaderboardInfo, 10>::vf12
/// Reverse row search for one ranked leaderboard. (`vf12 Race102`).
///
/// Fetches the tables, loads the row id at `index`, and scans the id array for it,
/// returning the position. Returns -1 when the fetch fails, the loaded id is -1,
/// the array is empty, or the id is absent (unsigned count).
/// Leaderboard id: 0x131.
export!(stdcall, rw_00569c30(index: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x131;
        let mut frame = [0u32; 6];
        let fetch: extern "fastcall" fn(u32, u32) -> u32 =
            core::mem::transmute(callee_addr(1) as usize);
        if fetch(LEADERBOARD_ID, frame.as_mut_ptr() as u32) as u8 == 0 {
            return 0xFFFF_FFFF;
        }
        let rows = frame[5];
        let want = (rows.wrapping_add(index.wrapping_mul(4)) as *const u32).read();
        if want == 0xFFFF_FFFF {
            return 0xFFFF_FFFF;
        }
        let count = frame[1];
        if count == 0 {
            return 0xFFFF_FFFF;
        }
        let ids = frame[2] as *const u32;
        let mut i = 0u32;
        while i < count {
            if ids.add(i as usize).read() == want {
                return i;
            }
            i = i.wrapping_add(1);
        }
        0xFFFF_FFFF
    }
});
