// original: 0x569e90 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_102, player_schema::LeaderboardInfo, 10>::vf6
/// Index lookup by id for one ranked leaderboard. (`vf6 Race102`).
///
/// Fetches the tables and scans the id array for `id`, returning the match
/// position. Returns -1 when the fetch fails, the count is not positive
/// (signed), or the id is absent.
/// Leaderboard id: 0x131.
export!(stdcall, rw_00569e90(id: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x131;
        let mut frame = [0u32; 6];
        let fetch: extern "fastcall" fn(u32, u32) -> u32 =
            core::mem::transmute(callee_addr(1) as usize);
        if fetch(LEADERBOARD_ID, frame.as_mut_ptr() as u32) as u8 == 0 {
            return 0xFFFF_FFFF;
        }
        let count = frame[3] as i32;
        if count <= 0 {
            return 0xFFFF_FFFF;
        }
        let ids = frame[4] as *const u32;
        let mut i = 0i32;
        while i < count {
            if ids.add(i as usize).read() == id {
                return i as u32;
            }
            i = i.wrapping_add(1);
        }
        0xFFFF_FFFF
    }
});
