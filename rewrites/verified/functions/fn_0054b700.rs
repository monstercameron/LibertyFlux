// original: 0x0054B700 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_BG_18, player_schema::LeaderboardInfo, 10>::vf12
/// Reverse row search for one ranked leaderboard board (this board: id 0xc9).
///
/// Calls the shared fetch step with (`LEADERBOARD_ID`, frame) and, when its
/// answer has a nonzero low byte, loads the row id at `index` from the row
/// table the fetch step filled in, then scans the id array it also filled in
/// for that id, returning the position. Returns -1 when the fetch step
/// reports failure (low byte zero), when the loaded id is -1, when the array
/// is empty (compared unsigned: zero only), or when the id is absent. Only
/// the low byte of the fetch answer is tested.
///
/// The fetch step takes its arguments in ECX (board id) and EDX (frame
/// pointer) with no stack arguments, so it is declared fastcall/0; its EDX
/// argument points into this function's own frame and is therefore not part
/// of the compared call (only ECX is).
///
/// Original: stdcall with one stack word; ECX is overwritten before any read,
/// so the method takes no `this` despite living in a vtable.
lf_checker_rt::export!(stdcall, rw_0054b700(index: u32) -> u32 {
    unsafe {
        /// Board id this instantiation passes to the fetch step in ECX.
        const LEADERBOARD_ID: u32 = 0xc9;
        /// Frame words the fetch step fills: count, id array, row table.
        const FRAME_COUNT: usize = 1;
        const FRAME_IDS: usize = 2;
        const FRAME_ROWS: usize = 5;
        const NOT_FOUND: u32 = 0xFFFF_FFFF;
        let mut frame = [0u32; 6];
        let answer = lf_checker_rt::callee_fastcall!(1, u32, LEADERBOARD_ID, frame.as_mut_ptr() as u32);
        if (answer & 0xFF) == 0 {
            return NOT_FOUND;
        }
        let rows = frame.as_ptr().add(FRAME_ROWS).read();
        let want = (rows.wrapping_add(index.wrapping_mul(4)) as *const u32).read();
        if want == NOT_FOUND {
            return NOT_FOUND;
        }
        let count = frame.as_ptr().add(FRAME_COUNT).read();
        if count == 0 {
            return NOT_FOUND;
        }
        let ids = frame.as_ptr().add(FRAME_IDS).read() as *const u32;
        let mut i = 0u32;
        while i < count {
            if ids.add(i as usize).read() == want {
                return i;
            }
            i = i.wrapping_add(1);
        }
        NOT_FOUND
    }
});
