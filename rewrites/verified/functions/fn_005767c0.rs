// original: 0x005767c0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_148, player_schema::LeaderboardInfo, 10>::vf6

/// Find the position of one row id within the board's row list.
///
/// `want` is the row id. The schema query (fastcall callee: ECX = board id,
/// EDX = out-block) provides the row count at word `OUT_COUNT` and the row
/// id array at word `OUT_ROWS`. The function returns the first position
/// holding `want`, or `NOT_FOUND` when the query fails, the count is not
/// positive, or no entry matches.
///
/// The count is compared signed, matching the original's conditional jumps.
/// The incoming ECX is ignored and one stack word is popped (stdcall).
lf_checker_rt::export!(stdcall, rw_005767c0(want: u32) -> u32 {
    unsafe {
        /// Board id passed to the schema query in ECX.
        const BOARD_ID: u32 = 0x16c;
        /// Word offsets within the query out-block: row count, row id array.
        const OUT_COUNT: usize = 3;
        const OUT_ROWS: usize = 4;
        /// Returned when the row id is not in the list.
        const NOT_FOUND: u32 = 0xFFFF_FFFF;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }

        let mut out = [0u32; 5];
        let ok: u32 =
            lf_checker_rt::callee_fastcall!(1, u32, BOARD_ID, out.as_mut_ptr() as u32);
        if ok & 0xFF == 0 {
            return NOT_FOUND;
        }
        let count = out[OUT_COUNT] as i32;
        if count <= 0 {
            return NOT_FOUND;
        }
        let rows = out[OUT_ROWS];
        let mut i = 0u32;
        loop {
            if rd32(rows.wrapping_add(i.wrapping_mul(4))) == want {
                return i;
            }
            i = i.wrapping_add(1);
            if i as i32 >= count {
                return NOT_FOUND;
            }
        }
    }
});
