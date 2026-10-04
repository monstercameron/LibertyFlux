// original: 0x0055a580 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_45, player_schema::LeaderboardInfo, 10>::vf6

/// Index of a row id in a ranked-race leaderboard's row array, or -1.
///
/// Calls the leaderboard fetch helper (checker callee 1, fastcall: board id in
/// ECX, out-buffer in EDX) for board 0xf9. The helper reports success in AL
/// and fills the out-buffer: the signed row count at +12 and the
/// row-id array pointer at +16; the words below are caller scratch
/// that this function zeroes first. When the helper fails, or the count is not
/// positive, the result is NOT_FOUND (-1). Otherwise the first `count` row ids
/// are scanned for `need` and the first matching index is returned, or
/// NOT_FOUND when no element matches.
///
/// `need` is the row id to find. The bound is signed: a negative count behaves
/// like zero. A count above the real array length would read past it, exactly
/// like the original.
///
/// Original: 0x0055A580 (stdcall, one stack word; ECX is dead on entry).
lf_checker_rt::export!(stdcall, rw_0055a580(need: u32) -> u32 {
    unsafe {
        const BOARD_ID: u32 = 0xf9;
        const FETCH_ROWS: u32 = 1;
        const COUNT_WORD: usize = 3;
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
        let count = out[COUNT_WORD] as i32;
        if count <= 0 {
            return NOT_FOUND;
        }
        let rows = out[ROWS_WORD];
        let mut i: i32 = 0;
        while i < count {
            let v =
                (rows.wrapping_add((i as u32).wrapping_mul(4)) as *const u32).read();
            if v == need {
                return i as u32;
            }
            i += 1;
        }
        NOT_FOUND
    }
});
