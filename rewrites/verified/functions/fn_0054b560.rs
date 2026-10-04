// original: 0x0054B560 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_BG_17, player_schema::LeaderboardInfo, 10>::vf7
/// Row lookup for one ranked leaderboard board (this board: id 0xc8).
///
/// Calls the shared fetch step with (`LEADERBOARD_ID`, frame) and, when its
/// answer has a nonzero low byte, returns the row pointer at `index` from the
/// row table the fetch step filled in. Returns -1 when the fetch step reports
/// failure. Only the low byte of the fetch answer is tested.
///
/// The fetch step takes its arguments in ECX (board id) and EDX (frame
/// pointer) with no stack arguments, so it is declared fastcall/0; its EDX
/// argument points into this function's own frame and is therefore not part
/// of the compared call (only ECX is).
///
/// Original: stdcall with one stack word; ECX is overwritten before any read,
/// so the method takes no `this` despite living in a vtable.
lf_checker_rt::export!(stdcall, rw_0054b560(index: u32) -> u32 {
    unsafe {
        /// Board id this instantiation passes to the fetch step in ECX.
        const LEADERBOARD_ID: u32 = 0xc8;
        /// Frame word the fetch step fills with the row-table pointer.
        const FRAME_ROWS: usize = 4;
        const NOT_FOUND: u32 = 0xFFFF_FFFF;
        let mut frame = [0u32; 6];
        let answer = lf_checker_rt::callee_fastcall!(1, u32, LEADERBOARD_ID, frame.as_mut_ptr() as u32);
        if (answer & 0xFF) == 0 {
            return NOT_FOUND;
        }
        let rows = frame.as_ptr().add(FRAME_ROWS).read();
        (rows.wrapping_add(index.wrapping_mul(4)) as *const u32).read()
    }
});
