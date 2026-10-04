// original: 0x00556c80 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_32, player_schema::LeaderboardInfo, 10>::vf6
/// Index of a row id in this board's row array, or -1.
///
/// Asks the row-fetch helper (checker callee 1: board id in ECX, out-buffer in
/// EDX) for board 0xf2. The helper answers in AL and, on success, leaves the
/// signed row count at out+12 and the row-id array at out+16.
/// A failed fetch or a non-positive count yields NOT_FOUND (-1); otherwise the
/// first `count` ids are scanned for `need` and the first hit index wins.
///
/// `need` is the row id sought. The bound is signed, so negative counts act as
/// zero. Counts past the true array length read out of bounds, as the original
/// does.
///
/// Original: 0x00556C80 (stdcall, one stack word; ECX unused).
lf_checker_rt::export!(stdcall, rw_00556c80(need: u32) -> u32 {
    unsafe {
        const BOARD_ID: u32 = 0xf2;
        const FETCH: u32 = 1;
        const COUNT_W: usize = 3;
        const ROWS_W: usize = 4;
        const NOT_FOUND: u32 = 0xffff_ffff;
        let mut out = [0u32; 5];
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
        let rows = out[ROWS_W];
        let mut i: i32 = 0;
        while i < count {
            let v = (rows.wrapping_add((i as u32).wrapping_mul(4))
                as *const u32).read();
            if v == need {
                return i as u32;
            }
            i += 1;
        }
        NOT_FOUND
    }
});
