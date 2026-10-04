// original: 0x00547C80 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_BG_4, player_schema::LeaderboardInfo, 10>::vf7
/// Row lookup for one ranked leaderboard board (virtual slot 7).
///
/// Fetches the board tables for board id 0xB5, then returns the row word at
/// `index`: `rows[index]`, or -1 when the fetch step reports failure (low
/// byte of its answer is zero).
///
/// Frame layout (offsets from the out-struct pointer in EDX): the rows-table
/// pointer lands at `+0x10`.
/// The shared fetch step fills a six-word frame passed by pointer in EDX
/// with the board id in ECX; only the low byte of its answer is tested.
///
/// Edge cases: a failing fetch returns -1 without reading any table; the
/// index is scaled by 4 with wraparound and read with one 32-bit load, so a
/// wild index faults exactly like the original's load.
/// Calling convention: stdcall with one stack argument; the incoming ECX
/// (`this`) is ignored by the original (its first use is the board-id store)
/// and is likewise ignored here.
lf_checker_rt::export!(stdcall, rw_00547c80(index: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0xB5;
        const FETCH: u32 = 1;
        const ROWS_SLOT: usize = 4; // frame offset 0x10
        let mut frame = [0u32; 6];
        let answer =
            lf_checker_rt::callee_fastcall!(FETCH, u32, LEADERBOARD_ID, frame.as_mut_ptr() as u32);
        if answer & 0xFF == 0 {
            return 0xFFFF_FFFF;
        }
        let rows = frame[ROWS_SLOT];
        (rows.wrapping_add(index.wrapping_mul(4)) as *const u32).read()
    }
});
