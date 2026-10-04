// original: 0x00559ec0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_44, player_schema::LeaderboardInfo, 10>::vf12
/// Reverse row search for one ranked leaderboard (`vf12 Race44`).
///
/// Fetches the tables, loads the row id at `index`, and scans the id array
/// for it, returning the position. Returns -1 when the fetch fails, when the
/// loaded id is -1, when the array is empty, or when the id is absent.
/// Leaderboard id: 0xe4.
export!(stdcall, rw_00559ec0(index: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0xe4;
        let mut frame = [0u32; 6];
        let fetch: extern "fastcall" fn(u32, u32) -> u32 =
            core::mem::transmute(callee_addr(1) as usize);
        if (fetch(LEADERBOARD_ID, frame.as_mut_ptr() as u32) & 0xFF) == 0 {
            return 0xFFFF_FFFF;
        }
        let rows = frame.as_ptr().add(5).read();
        let want = (rows.wrapping_add(index.wrapping_mul(4)) as *const u32).read();
        if want == 0xFFFF_FFFF {
            return 0xFFFF_FFFF;
        }
        let count = frame.as_ptr().add(1).read();
        if count == 0 {
            return 0xFFFF_FFFF;
        }
        let ids = frame.as_ptr().add(2).read() as *const u32;
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
