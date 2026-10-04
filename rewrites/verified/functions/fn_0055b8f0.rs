// original: 0x0055b8f0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_50, player_schema::LeaderboardInfo, 10>::vf12

/// Index of an indirect row id in a leaderboard's first row array, or -1.
///
/// Calls the leaderboard fetch helper (checker callee 1, fastcall: board id in
/// ECX, out-buffer in EDX) for board 0xfd. The helper reports success in AL
/// and fills three out-buffer words: the row count at +4, the first
/// row-id array at +8 and the second row-id array at +20.
/// The word at `index` in the second array is the value to find; when that
/// word is NO_ROW (-1), when the helper fails, or when the count is zero, the
/// result is NOT_FOUND (-1). Otherwise the first array is scanned with an
/// UNSIGNED bound and the first matching index is returned, or NOT_FOUND.
///
/// `index` selects the value in the second array. Neither array access is
/// bounds-checked. A huge count with no match would scan far past the array,
/// exactly like the original.
///
/// Original: 0x0055B8F0 (stdcall, one stack word; ECX is dead on entry).
lf_checker_rt::export!(stdcall, rw_0055b8f0(index: u32) -> u32 {
    unsafe {
        const BOARD_ID: u32 = 0xfd;
        const FETCH_ROWS: u32 = 1;
        const COUNT_WORD: usize = 1;
        const FIRST_WORD: usize = 2;
        const SECOND_WORD: usize = 5;
        const NOT_FOUND: u32 = 0xffff_ffff;
        const NO_ROW: u32 = 0xffff_ffff;
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
        let second = out[SECOND_WORD];
        let v =
            (second.wrapping_add(index.wrapping_mul(4)) as *const u32).read();
        if v == NO_ROW {
            return NOT_FOUND;
        }
        let count = out[COUNT_WORD];
        if count == 0 {
            return NOT_FOUND;
        }
        let first = out[FIRST_WORD];
        let mut i: u32 = 0;
        while i < count {
            let w = (first.wrapping_add(i.wrapping_mul(4)) as *const u32).read();
            if w == v {
                return i;
            }
            i += 1;
        }
        NOT_FOUND
    }
});
