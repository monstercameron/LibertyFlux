// original: 0x00596530 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_265, player_schema::LeaderboardInfo, 10>::vf12
/// Look up one leaderboard value by row, then find that value's position in
/// the key column. Returns the column index, or -1 when the fetch fails, the
/// row holds -1, the table is empty, or the value is absent.
export!(stdcall, rw_00596530(arg: u32) -> i32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x1dd;
        let mut buf = [0u32; 6];
        let ok: u32 = callee_fastcall!(1, u32, LEADERBOARD_ID, buf.as_mut_ptr() as u32);
        if (ok as u8) == 0 {
            return -1;
        }
        let count = buf[1];
        let keys = buf[2] as *const u32;
        let values = buf[5] as *const u32;
        let elem = *values.add(arg as usize);
        if elem == 0xFFFF_FFFF {
            return -1;
        }
        if count == 0 {
            return -1;
        }
        let mut i: u32 = 0;
        loop {
            if *keys.add(i as usize) == elem {
                return i as i32;
            }
            i = i.wrapping_add(1);
            if i >= count {
                return -1;
            }
        }
    }
});
