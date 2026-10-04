// original: 0x00556a90 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_32, player_schema::LeaderboardInfo, 10>::vf13
/// Second-array row where a row id sits in the first array, or -1.
///
/// Asks the row-fetch helper (checker callee 1: board id in ECX, out-buffer in
/// EDX) for board 0xf2. The helper answers in AL and, on success, leaves the
/// signed row count at out+12, the first row-id array at
/// out+16 and the second at out+20. A failed fetch or a
/// non-positive count yields NOT_FOUND (-1); otherwise `need` is sought in the
/// first array under a SIGNED bound, and a hit at `i` returns the word at `i`
/// in the second array.
///
/// The original re-tests the hit index against -1 after a hit; that test can
/// never fire (hits are in [0, count)) and is omitted here. Neither array
/// access is bounds-checked.
///
/// Original: 0x00556A90 (stdcall, one stack word; ECX unused).
lf_checker_rt::export!(stdcall, rw_00556a90(need: u32) -> u32 {
    unsafe {
        const BOARD_ID: u32 = 0xf2;
        const FETCH: u32 = 1;
        const COUNT_W: usize = 3;
        const FIRST_W: usize = 4;
        const SECOND_W: usize = 5;
        const NOT_FOUND: u32 = 0xffff_ffff;
        let mut out = [0u32; 6];
        let ok: u32 = lf_checker_rt::callee_fastcall!(
            FETCH, u32, BOARD_ID, out.as_mut_ptr() as u32
        );
        if (ok & 0xff) == 0 {
            return NOT_FOUND;
        }
        let count = out[COUNT_W] as i32;
        if count <= 0 {
            return NOT_FOUND;
        }
        let first = out[FIRST_W];
        let second = out[SECOND_W];
        let mut i: i32 = 0;
        while i < count {
            let v = (first.wrapping_add((i as u32).wrapping_mul(4))
                as *const u32).read();
            if v == need {
                return (second.wrapping_add((i as u32).wrapping_mul(4)) as *const u32).read();
            }
            i += 1;
        }
        NOT_FOUND
    }
});
