// original: 0x0055ae90 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_47, player_schema::LeaderboardInfo, 10>::vf7

/// Row id at an index in a ranked-race leaderboard's row array, or -1.
///
/// Calls the leaderboard fetch helper (checker callee 1, fastcall: board id in
/// ECX, out-buffer in EDX) for board 0xfa. The helper reports success in AL
/// and fills the row-id array pointer at out-buffer +16. On helper
/// failure the result is NOT_FOUND (-1); otherwise it is the array word at
/// `index`. There is no bounds check: an out-of-range index reads past the
/// array, exactly like the original.
///
/// Original: 0x0055AE90 (stdcall, one stack word; ECX is dead on entry).
lf_checker_rt::export!(stdcall, rw_0055ae90(index: u32) -> u32 {
    unsafe {
        const BOARD_ID: u32 = 0xfa;
        const FETCH_ROWS: u32 = 1;
        const ROWS_WORD: usize = 4;
        const NOT_FOUND: u32 = 0xffff_ffff;
        let mut out = [0u32; 5];
        let ok: u32 = lf_checker_rt::callee_fastcall!(
            FETCH_ROWS,
            u32,
            BOARD_ID,
            out.as_mut_ptr() as u32
        );
        if (ok & 0xff) == 0 {
            return NOT_FOUND;
        }
        let rows = out[ROWS_WORD];
        let at = index;
        (rows.wrapping_add(at.wrapping_mul(4)) as *const u32).read()
    }
});
