// original: 0x0051a2f0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Race13NoHolds, player_schema::LeaderboardInfo, 10>::vf7
/// Row lookup for one ranked leaderboard (`vf7 Race13`).
///
/// Fetches the leaderboard tables, fails with -1 when the fetch step reports
/// failure (low byte of its answer), else returns the row pointer at `index`.
/// The original tests only the low byte of the fetch answer.
/// Leaderboard id: 0x6A.
export!(stdcall, rw_0051a2f0(index: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x6A;
        let mut frame = [0u32; 6];
        let fetch: extern "fastcall" fn(u32, u32) -> u32 =
            core::mem::transmute(callee_addr(1) as usize);
        if (fetch(LEADERBOARD_ID, frame.as_mut_ptr() as u32) & 0xFF) == 0 {
            return 0xFFFF_FFFF;
        }
        let rows = frame.as_ptr().add(4).read();
        (rows.wrapping_add(index.wrapping_mul(4)) as *const u32).read()
    }
});
