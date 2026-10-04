// original: 0x00529350 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Race8Standard, player_schema::LeaderboardInfo, 10>::vf7

/// Row id at an index of the race-8 id array, or -1 on lookup failure.
///
/// `index` counts array slots from zero. `this` (ECX) is ignored: the schema
/// is selected by the constant id SCHEMA_ID handed to the lookup. The lookup
/// (callee 1) answers ok/failed in AL and, when ok, writes a pointer to the
/// id array at scratch-block offset IDS_OFF. A failed lookup yields MISSING;
/// otherwise the slot is read with no bounds check, so an out-of-range index
/// reads whatever follows the array.
///
/// Original: 0x00529350 (thiscall, one stack word; ECX ignored).
lf_checker_rt::export!(thiscall, rw_00529350(_this: u32, index: u32) -> u32 {
    unsafe {
        /// Schema id selecting the race-8 leaderboard.
        const SCHEMA_ID: u32 = 0x56;
        /// Byte offset of the id-array pointer in the lookup out-block.
        const IDS_OFF: u32 = 16;
        const MISSING: u32 = 0xFFFF_FFFF;
        let mut block = [0u32; 5];
        let ok: u32 =
            lf_checker_rt::callee_fastcall!(1, u32, SCHEMA_ID, block.as_mut_ptr() as u32);
        // Only AL carries the answer; the upper bytes are entry leftovers.
        if (ok & 0xFF) == 0 {
            return MISSING;
        }
        let ids = block[(IDS_OFF / 4) as usize];
        (ids.wrapping_add(index.wrapping_mul(4)) as *const u32).read_unaligned()
    }
});
