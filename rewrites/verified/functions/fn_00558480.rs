// original: 0x00558480 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_38, player_schema::LeaderboardInfo, 10>::vf12
/// Index of an indirect row id in this board's first array, or -1.
///
/// Asks the row-fetch helper (checker callee 1: board id in ECX, out-buffer in
/// EDX) for board 0xf4. The helper answers in AL and, on success, leaves the
/// row count at out+4, the first row-id array at out+8 and
/// the second at out+20. The word at `index` in the second array is
/// the value sought in the first array under an UNSIGNED bound. A failed
/// fetch, a NO_ROW (-1) selector, a zero count, or no hit yields NOT_FOUND.
///
/// Neither array access is bounds-checked: a large count scans on past the
/// array, as the original does.
///
/// Original: 0x00558480 (stdcall, one stack word; ECX unused).
lf_checker_rt::export!(stdcall, rw_00558480(index: u32) -> u32 {
    unsafe {
        const BOARD_ID: u32 = 0xf4;
        const FETCH: u32 = 1;
        const COUNT_W: usize = 1;
        const FIRST_W: usize = 2;
        const SECOND_W: usize = 5;
        const NOT_FOUND: u32 = 0xffff_ffff;
        const NO_ROW: u32 = 0xffff_ffff;
        let mut out = [0u32; 6];
        let ok: u32 = lf_checker_rt::callee_fastcall!(
            FETCH, u32, BOARD_ID, out.as_mut_ptr() as u32
        );
        if (ok & 0xff) == 0 {
            return NOT_FOUND;
        }
        let second = out[SECOND_W];
        let v = (second.wrapping_add(index.wrapping_mul(4)) as *const u32).read();
        if v == NO_ROW {
            return NOT_FOUND;
        }
        let count = out[COUNT_W];
        if count == 0 {
            return NOT_FOUND;
        }
        let first = out[FIRST_W];
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
