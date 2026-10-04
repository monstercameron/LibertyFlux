// original: 0x0051c620 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Race21NoHolds, player_schema::LeaderboardInfo, 10>::vf8

/// Classify a leaderboard cell value into a width (4 or 8) or 0.
///
/// Asks the table helper (callee 1) for leaderboard 0x72, whose answer
/// puts the column base at struct +0x14; reads v = base[index] and asks
/// the kind helper (callee 2, thiscall-style with v in ECX) for its
/// kind r. Returns MAP[r - 1] for the five-way map [4, 8, 8, 0, 4]
/// (r - 1 compared unsigned, so r = 0 and r > 5 also land here), and 0
/// when the table helper fails. stdcall.
lf_checker_rt::export!(stdcall, rw_0051c620(index: u32) -> u32 {
    unsafe {
        const LEADERBOARD: u32 = 0x72;
        const COL_BASE: usize = 5; // +0x14
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
