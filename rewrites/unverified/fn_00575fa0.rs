// original: 0x00575fa0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_146, player_schema::LeaderboardInfo, 10>::vf8

/// Map one leaderboard cell through the column-type query to a field width.
///
/// `index` is the row. After the schema query (fastcall callee: ECX = board
/// id, EDX = out-block, column array at word `OUT_ARR`), the cell value is
/// passed to the column-type query (thiscall callee, object in ECX). Its
/// answer selects a width: kind minus one indexes the mapping
/// (0, 1, 2, 3, 4) -> (4, 8, 8, 0, 4); kind -1, kind 0, kinds above 5, and a
/// failed schema query all yield 0.
///
/// The mapping above was read from the image's own jump table for this
/// function. The incoming ECX is ignored and one stack word is popped
/// (stdcall). There is no bounds check on `index`.
lf_checker_rt::export!(stdcall, rw_00575fa0(index: u32) -> u32 {
    unsafe {
        /// Board id passed to the schema query in ECX.
        const BOARD_ID: u32 = 0x16a;
        /// Word offset of the column array pointer within the query out-block.
        const OUT_ARR: usize = 5;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }

        let mut out = [0u32; 6];
        let ok: u32 =
            lf_checker_rt::callee_fastcall!(1, u32, BOARD_ID, out.as_mut_ptr() as u32);
        if ok & 0xFF == 0 {
            return 0;
        }
        let cell = rd32(out[OUT_ARR].wrapping_add(index.wrapping_mul(4)));
        let kind: u32 = lf_checker_rt::callee_thiscall!(2, u32, cell);
        if kind == 0xFFFF_FFFF {
            return 0;
        }
        match kind.wrapping_sub(1) {
            0 => 4,
            1 | 2 => 8,
            4 => 4,
            _ => 0,
        }
    }
});
