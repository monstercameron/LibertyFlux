// original: 0x0051c9e0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Race22NoHolds, player_schema::LeaderboardInfo, 10>::vf6

/// Find the row holding a value in a leaderboard column.
///
/// Same shape as rw_0051c580 for leaderboard 0x73: signed count at
/// struct +0x0C, column base at +0x10, first match or -1. stdcall.
lf_checker_rt::export!(stdcall, rw_0051c9e0(value: u32) -> u32 {
    unsafe {
        const LEADERBOARD: u32 = 0x73;
        const COUNT: usize = 3;
        const COL_BASE: usize = 4;
        let mut info = [0u32; 8];
        let ok: u32 =
            lf_checker_rt::callee_fastcall!(1, u32, LEADERBOARD, info.as_mut_ptr() as u32);
        if (ok as u8) == 0 {
            return 0xFFFF_FFFF;
        }
        let count = info[COUNT] as i32;
        if count <= 0 {
            return 0xFFFF_FFFF;
        }
        let base = info[COL_BASE];
        let mut i = 0i32;
        loop {
            let w = (base.wrapping_add((i as u32).wrapping_mul(4)) as *const u32)
                .read_unaligned();
            if w == value {
                return i as u32;
            }
            i = i.wrapping_add(1);
            if !(i < count) {
                return 0xFFFF_FFFF;
            }
        }
    }
});
