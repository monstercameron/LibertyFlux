// original: 0x0051ca80 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Race22NoHolds, player_schema::LeaderboardInfo, 10>::vf8

/// Classify a leaderboard cell value into a width (4 or 8) or 0.
///
/// Same shape as rw_0051c620 for leaderboard 0x73: column base at
/// struct +0x14, kind helper, five-way map [4, 8, 8, 0, 4], else 0.
/// stdcall.
lf_checker_rt::export!(stdcall, rw_0051ca80(index: u32) -> u32 {
    unsafe {
        const LEADERBOARD: u32 = 0x73;
        const COL_BASE: usize = 5;
        const MAP: [u32; 5] = [4, 8, 8, 0, 4];
        let mut info = [0u32; 8];
        let ok: u32 =
            lf_checker_rt::callee_fastcall!(1, u32, LEADERBOARD, info.as_mut_ptr() as u32);
        if (ok as u8) == 0 {
            return 0;
        }
        let v = (info[COL_BASE].wrapping_add(index.wrapping_mul(4)) as *const u32)
            .read_unaligned();
        let r: u32 = lf_checker_rt::callee_thiscall!(2, u32, v);
        let t = r.wrapping_sub(1);
        if t > 4 {
            return 0;
        }
        MAP[t as usize]
    }
});
