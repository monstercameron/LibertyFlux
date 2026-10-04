// original: 0x005763c0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_147, player_schema::LeaderboardInfo, 10>::vf7

/// Return one column value of this leaderboard by row index.
///
/// `index` is the row. The function queries the leaderboard schema (fastcall
/// callee: ECX = board id, EDX = out-block) and reads the column array
/// pointer from word `OUT_ARR` of the out-block. It returns `array[index]`,
/// or `NOT_FOUND` when the query reports failure in AL.
///
/// The incoming ECX (`this`) is ignored and one stack word is popped, so the
/// export is stdcall. There is no bounds check on `index`.
lf_checker_rt::export!(stdcall, rw_005763c0(index: u32) -> u32 {
    unsafe {
        /// Board id passed to the schema query in ECX.
        const BOARD_ID: u32 = 0x16b;
        /// Word offset of the column array pointer within the query out-block.
        const OUT_ARR: usize = 4;
        /// Returned when the schema query fails.
        const NOT_FOUND: u32 = 0xFFFF_FFFF;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }

        let mut out = [0u32; 6];
        let ok: u32 =
            lf_checker_rt::callee_fastcall!(1, u32, BOARD_ID, out.as_mut_ptr() as u32);
        if ok & 0xFF == 0 {
            return NOT_FOUND;
        }
        rd32(out[OUT_ARR].wrapping_add(index.wrapping_mul(4)))
    }
});
