// original: 0x00595ce0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_263, player_schema::LeaderboardInfo, 10>::vf13
/// Find the requested key in the leaderboard key column and return the value
/// stored alongside it. Returns -1 when the fetch fails, the table is empty,
/// or the key is absent. The row count is compared signed.
export!(stdcall, rw_00595ce0(arg: u32) -> i32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x1db;
        let mut buf = [0u32; 6];
        let ok: u32 = callee_fastcall!(1, u32, LEADERBOARD_ID, buf.as_mut_ptr() as u32);
        if (ok as u8) == 0 {
            return -1;
        }
        let count = buf[3] as i32;
        if count <= 0 {
            return -1;
        }
        let keys = buf[4] as *const u32;
        let values = buf[5] as *const u32;
        let mut i: i32 = 0;
        loop {
            if *keys.add(i as usize) == arg {
                return *values.add(i as usize) as i32;
            }
            i = i.wrapping_add(1);
            if i >= count {
                return -1;
            }
        }
    }
});
