// original: 0x0051c780 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Race22NoHolds, player_schema::LeaderboardInfo, 10>::vf12

/// Map a row index through one column, then find that value's row in another.
///
/// Same shape as rw_0051c320 for leaderboard 0x73. stdcall.
lf_checker_rt::export!(stdcall, rw_0051c780(index: u32) -> u32 {
    unsafe {
        const LEADERBOARD: u32 = 0x73;
        const COUNT: usize = 1;
        const COL1: usize = 2;
        const COL2: usize = 5;
        let mut info = [0u32; 8];
        let ok: u32 =
            lf_checker_rt::callee_fastcall!(1, u32, LEADERBOARD, info.as_mut_ptr() as u32);
        if (ok as u8) == 0 {
            return 0xFFFF_FFFF;
        }
        let v = (info[COL2].wrapping_add(index.wrapping_mul(4)) as *const u32)
            .read_unaligned();
        if v == 0xFFFF_FFFF {
            return 0xFFFF_FFFF;
        }
        let count = info[COUNT];
        if count == 0 {
            return 0xFFFF_FFFF;
        }
        let base = info[COL1];
        let mut i = 0u32;
        loop {
            let w = (base.wrapping_add(i.wrapping_mul(4)) as *const u32).read_unaligned();
            if w == v {
                return i;
            }
            i = i.wrapping_add(1);
            if !(i < count) {
                return 0xFFFF_FFFF;
            }
        }
    }
});
