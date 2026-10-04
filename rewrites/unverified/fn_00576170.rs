// original: 0x00576170 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_147, player_schema::LeaderboardInfo, 10>::vf13

/// Look up the leaderboard value stored for one row id.
///
/// `want` is the row id. The schema query (fastcall callee: ECX = board id,
/// EDX = out-block) provides the row count at word `OUT_COUNT`, the row id
/// array at `OUT_ROWS` and the value array at `OUT_VALS`. The function scans
/// the ids and returns the value at the matching position, or `NOT_FOUND`
/// when the query fails, the count is not positive, or no id matches.
///
/// The count is compared signed, matching the original's conditional jumps.
/// After a match the original compares the found index against -1, which can
/// never hold (the scan starts at 0), so that check has no effect and is not
/// reproduced. The incoming ECX is ignored; one stack word is popped
/// (stdcall).
lf_checker_rt::export!(stdcall, rw_00576170(want: u32) -> u32 {
    unsafe {
        /// Board id passed to the schema query in ECX.
        const BOARD_ID: u32 = 0x16b;
        /// Word offsets within the query out-block: row count, row id
        /// array, value array.
        const OUT_COUNT: usize = 3;
        const OUT_ROWS: usize = 4;
        const OUT_VALS: usize = 5;
        /// Returned when the row id has no stored value.
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
        let count = out[OUT_COUNT] as i32;
        if count <= 0 {
            return NOT_FOUND;
        }
        let rows = out[OUT_ROWS];
        let vals = out[OUT_VALS];
        let mut i = 0u32;
        loop {
            if rd32(rows.wrapping_add(i.wrapping_mul(4))) == want {
                return rd32(vals.wrapping_add(i.wrapping_mul(4)));
            }
            i = i.wrapping_add(1);
            if i as i32 >= count {
                return NOT_FOUND;
            }
        }
    }
});
