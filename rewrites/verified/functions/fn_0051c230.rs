// original: 0x0051c230 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Race20NoHolds, player_schema::LeaderboardInfo, 10>::vf9

/// Classify a leaderboard cell value into a small code or -1.
///
/// Asks the table helper (callee 1) for leaderboard 0x71, whose answer
/// puts the column base at struct +0x14; reads v = base[index] and asks
/// the kind helper (callee 2) for its kind r. Returns MAP[r - 1] for the
/// five-way map [0, 1, 3, -1, 2] (r - 1 compared unsigned), and -1 when
/// the table helper fails or r - 1 exceeds 4. stdcall.
lf_checker_rt::export!(stdcall, rw_0051c230(index: u32) -> u32 {
    unsafe {
        const LEADERBOARD: u32 = 0x71;
        const COL_BASE: usize = 5; // +0x14
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
