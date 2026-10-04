// original: 0x0056d7c0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_115, player_schema::LeaderboardInfo, 10>::vf7
/// Row lookup for one ranked leaderboard.
///
/// Fetches the leaderboard tables for LEADERBOARD_ID through the shared fetch
/// step (id in ECX, six-word frame in EDX) and fails with NOT_FOUND when the
/// fetch reports failure in the low byte of its answer. Otherwise returns the
/// row word at `index` from the rows table the fetch filled in at frame word 4.
///
/// The original takes no `this` (ECX is overwritten with the id before any
/// read) and is stdcall with one stack argument. It never touches globals and
/// writes only its own stack frame.
lf_checker_rt::export!(stdcall, rw_0056d7c0(index: u32) -> u32 {
    unsafe {
        const NOT_FOUND: u32 = 0xFFFF_FFFF;
        const ROWS_WORD: usize = 4;
        const LEADERBOARD_ID: u32 = 318;
        const FETCH_CALLEE: u32 = 1;
        let mut frame = [0u32; 6];
        let answer = lf_checker_rt::callee_fastcall!(FETCH_CALLEE, u32, LEADERBOARD_ID,
            frame.as_mut_ptr() as u32);
        if answer & 0xFF == 0 {
            return NOT_FOUND;
        }
        let rows = frame[ROWS_WORD];
        let v = (rows.wrapping_add(index.wrapping_mul(4)) as *const u32).read();
        v
    }
});
