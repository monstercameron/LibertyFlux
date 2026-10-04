// original: 0x00575ca0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_146, player_schema::LeaderboardInfo, 10>::vf12

/// Find the position of one leaderboard row id within the board's row list.
///
/// `index` selects a row id from the id array; the function returns the
/// position of that id in the row array, or `NOT_FOUND`. The schema query
/// (fastcall callee: ECX = board id, EDX = out-block) provides the row count
/// at word `OUT_COUNT`, the row array at `OUT_ROWS` and the id array at
/// `OUT_IDS`. A failed query, an id of `NOT_FOUND`, an empty list, or a
/// linear scan that finds nothing all yield `NOT_FOUND`.
///
/// The incoming ECX is ignored and one stack word is popped (stdcall). The
/// scan bound is unsigned and there is no bounds check on `index`.
lf_checker_rt::export!(stdcall, rw_00575ca0(index: u32) -> u32 {
    unsafe {
        /// Board id passed to the schema query in ECX.
        const BOARD_ID: u32 = 0x16a;
        /// Word offsets within the query out-block: row count, row array,
        /// id array.
        const OUT_COUNT: usize = 1;
        const OUT_ROWS: usize = 2;
        const OUT_IDS: usize = 5;
        /// Returned when the row id cannot be placed.
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
        let needle = rd32(out[OUT_IDS].wrapping_add(index.wrapping_mul(4)));
        if needle == NOT_FOUND {
            return NOT_FOUND;
        }
        let count = out[OUT_COUNT];
        if count == 0 {
            return NOT_FOUND;
        }
        let rows = out[OUT_ROWS];
        let mut i = 0u32;
        loop {
            if rd32(rows.wrapping_add(i.wrapping_mul(4))) == needle {
                return i;
            }
            i = i.wrapping_add(1);
            if i >= count {
                return NOT_FOUND;
            }
        }
    }
});
