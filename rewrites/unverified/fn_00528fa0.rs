// original: 0x00528FA0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Race7Standard, player_schema::LeaderboardInfo, 10>::vf9

/// Rank class of a race-7 leaderboard row, or -1 when unknown.
///
/// `index` counts array slots from zero. `this` (ECX) is ignored: the schema
/// is selected by the constant id SCHEMA_ID handed to the lookup. The lookup
/// (callee 1) answers ok/failed in AL and, when ok, writes a pointer to the
/// row array at scratch-block offset ROWS_OFF. A failed lookup yields
/// UNKNOWN. Otherwise the slot is read with no bounds check and classified
/// by callee 2; its tag minus one selects the class (1->0, 2->1, 3->3,
/// 4->-1, 5->2) and anything else, including a tag of -1 or 0, yields
/// UNKNOWN (-1).
///
/// Original: 0x00528FA0 (thiscall, one stack word; ECX ignored).
lf_checker_rt::export!(thiscall, rw_00528fa0(_this: u32, index: u32) -> u32 {
    unsafe {
        /// Schema id selecting the race-7 leaderboard.
        const SCHEMA_ID: u32 = 0x55;
        /// Byte offset of the row-array pointer in the lookup out-block.
        const ROWS_OFF: u32 = 20;
        const UNKNOWN: u32 = 0xFFFF_FFFF;
        let mut block = [0u32; 6];
        let ok: u32 =
            lf_checker_rt::callee_fastcall!(1, u32, SCHEMA_ID, block.as_mut_ptr() as u32);
        // Only AL carries the answer; the upper bytes are entry leftovers.
        if (ok & 0xFF) == 0 {
            return UNKNOWN;
        }
        let rows = block[(ROWS_OFF / 4) as usize];
        let row = (rows.wrapping_add(index.wrapping_mul(4)) as *const u32).read_unaligned();
        let tag: u32 = lf_checker_rt::callee_thiscall!(2, u32, row);
        match tag.wrapping_sub(1) {
0 => 0,
            1 => 1,
            2 => 3,
            4 => 2,
            _ => UNKNOWN,
        }
    }
});
