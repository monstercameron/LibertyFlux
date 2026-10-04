// original: 0x0051c690 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Race21NoHolds, player_schema::LeaderboardInfo, 10>::vf9

/// Classify a leaderboard cell value into a small code or -1.
///
/// Same shape as rw_0051c230 for leaderboard 0x72: column base at
/// struct +0x14, kind helper, five-way map [0, 1, 3, -1, 2], else -1.
/// stdcall.
lf_checker_rt::export!(stdcall, rw_0051c690(index: u32) -> u32 {
    unsafe {
        const LEADERBOARD: u32 = 0x72;
        const COL_BASE: usize = 5;
        const MAP: [u32; 5] = [0, 1, 3, 0xFFFF_FFFF, 2];
        let mut info = [0u32; 8];
        let ok: u32 =
            lf_checker_rt::callee_fastcall!(1, u32, LEADERBOARD, info.as_mut_ptr() as u32);
        if (ok as u8) == 0 {
            return 0xFFFF_FFFF;
        }
        let v = (info[COL_BASE].wrapping_add(index.wrapping_mul(4)) as *const u32)
            .read_unaligned();
        let r: u32 = lf_checker_rt::callee_thiscall!(2, u32, v);
        let t = r.wrapping_sub(1);
        if t > 4 {
            return 0xFFFF_FFFF;
        }
        MAP[t as usize]
    }
});
