// original: 0x0056b6e0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_108, player_schema::LeaderboardInfo, 10>::vf13
/// Value lookup by id for one ranked leaderboard (`vf13 Race108`).
///
/// Fetches the tables and scans the id array for `id`, returning the value
/// from the parallel value array at the match. Returns -1 when the fetch
/// fails, when the count is not positive (signed), or when the id is absent.
/// The original re-checks the found index against -1 after the loop, which is
/// unreachable (the index counts up from 0) and is not repeated here.
/// Leaderboard id: 0x137.
export!(stdcall, rw_0056b6e0(id: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x137;
        let mut frame = [0u32; 6];
        let fetch: extern "fastcall" fn(u32, u32) -> u32 =
            core::mem::transmute(callee_addr(1) as usize);
        if (fetch(LEADERBOARD_ID, frame.as_mut_ptr() as u32) & 0xFF) == 0 {
            return 0xFFFF_FFFF;
        }
        let count = frame.as_ptr().add(3).read() as i32;
        if count <= 0 {
            return 0xFFFF_FFFF;
        }
        let ids = frame.as_ptr().add(4).read() as *const u32;
        let mut i = 0i32;
        loop {
            if i >= count {
                return 0xFFFF_FFFF;
            }
            if ids.add(i as usize).read() == id {
                break;
            }
            i = i.wrapping_add(1);
        }
        let values = frame.as_ptr().add(5).read();
        (values.wrapping_add((i as u32).wrapping_mul(4)) as *const u32).read()
    }
});
