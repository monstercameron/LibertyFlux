// original: 0x0055ac50 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_47, player_schema::LeaderboardInfo, 10>::vf13

/// Second-array row at the position where a row id sits in the first, or -1.
///
/// Calls the leaderboard fetch helper (checker callee 1, fastcall: board id in
/// ECX, out-buffer in EDX) for board 0xfa. The helper reports success in AL
/// and fills three out-buffer words: the signed row count at +12,
/// the first row-id array at +16 and the second row-id array at
/// +20. When the helper fails, or the count is not positive, the
/// result is NOT_FOUND (-1). Otherwise `need` is searched in the first array
/// with a SIGNED bound; a hit at index `i` yields the word at `i` in the
/// second array, and no hit yields NOT_FOUND.
///
/// Note: after a hit the original re-tests the index against -1, which can
/// never fire (the loop only exits with an index in [0, count)); the rewrite
/// omits that dead test. Neither array access is bounds-checked.
///
/// Original: 0x0055AC50 (stdcall, one stack word; ECX is dead on entry).
lf_checker_rt::export!(stdcall, rw_0055ac50(need: u32) -> u32 {
    unsafe {
        const BOARD_ID: u32 = 0xfa;
        const FETCH_ROWS: u32 = 1;
        const COUNT_WORD: usize = 3;
        const FIRST_WORD: usize = 4;
        const SECOND_WORD: usize = 5;
        const NOT_FOUND: u32 = 0xffff_ffff;
        let mut out = [0u32; 6];
        let ok: u32 = lf_checker_rt::callee_fastcall!(
            FETCH_ROWS,
            u32,
            BOARD_ID,
            out.as_mut_ptr() as u32
        );
        if (ok & 0xff) == 0 {
            return NOT_FOUND;
        }
        let count = out[COUNT_WORD] as i32;
        if count <= 0 {
            return NOT_FOUND;
        }
        let first = out[FIRST_WORD];
        let mut i: i32 = 0;
        while i < count {
            let v =
                (first.wrapping_add((i as u32).wrapping_mul(4)) as *const u32).read();
            if v != need {
                i += 1;
                continue;
            }
            let second = out[SECOND_WORD];
            return (second.wrapping_add((i as u32).wrapping_mul(4)) as *const u32).read();
        }
        NOT_FOUND
    }
});
