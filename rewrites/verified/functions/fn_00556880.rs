// original: 0x00556880 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_31, player_schema::LeaderboardInfo, 10>::vf7
/// Row id at an index in this board's row array, or -1.
///
/// Asks the row-fetch helper (checker callee 1: board id in ECX, out-buffer in
/// EDX) for board 0xeb. The helper answers in AL and, on success, leaves the
/// row-id array at out+16. A failed fetch yields NOT_FOUND (-1);
/// otherwise the word at `index` is returned with no bounds check, so a wild
/// index reads out of bounds exactly as the original does.
///
/// Original: 0x00556880 (stdcall, one stack word; ECX unused).
lf_checker_rt::export!(stdcall, rw_00556880(index: u32) -> u32 {
    unsafe {
        const BOARD_ID: u32 = 0xeb;
        const FETCH: u32 = 1;
        const ROWS_W: usize = 4;
        const NOT_FOUND: u32 = 0xffff_ffff;
        let mut out = [0u32; 5];
        let ok: u32 = lf_checker_rt::callee_fastcall!(
            FETCH, u32, BOARD_ID, out.as_mut_ptr() as u32
        );
        if (ok & 0xff) == 0 {
            return NOT_FOUND;
        }
        let rows = out[ROWS_W];
        let at = index;
        (rows.wrapping_add(at.wrapping_mul(4)) as *const u32).read()
    }
});
