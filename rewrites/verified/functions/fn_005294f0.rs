// original: 0x005294F0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Race9Standard, player_schema::LeaderboardInfo, 10>::vf12

/// Index, in the race-9 id array, of the row id held by another row.
///
/// `index` counts slots of the row array from zero. `this` (ECX) is ignored:
/// the schema is selected by the constant id SCHEMA_ID handed to the lookup.
/// The lookup (callee 1) answers ok/failed in AL and, when ok, writes the id
/// count at scratch-block offset COUNT_OFF, the id-array pointer at IDS_OFF
/// and the row-array pointer at ROWS_OFF. A failed lookup yields NOT_FOUND.
/// The row slot is read with no bounds check; a row id of -1 (empty slot)
/// yields NOT_FOUND, as does a zero id count. Otherwise the id array is
/// searched front to back (the bound is compared unsigned) and the first
/// index holding the row id is returned, or NOT_FOUND when none matches.
///
/// Original: 0x005294F0 (thiscall, one stack word; ECX ignored).
lf_checker_rt::export!(thiscall, rw_005294f0(_this: u32, index: u32) -> u32 {
    unsafe {
        /// Schema id selecting the race-9 leaderboard.
        const SCHEMA_ID: u32 = 0x57;
        /// Byte offset of the id count in the lookup out-block.
        const COUNT_OFF: u32 = 4;
        /// Byte offset of the id-array pointer in the lookup out-block.
        const IDS_OFF: u32 = 8;
        /// Byte offset of the row-array pointer in the lookup out-block.
        const ROWS_OFF: u32 = 20;
        const NOT_FOUND: u32 = 0xFFFF_FFFF;
        let mut block = [0u32; 6];
        let ok: u32 =
            lf_checker_rt::callee_fastcall!(1, u32, SCHEMA_ID, block.as_mut_ptr() as u32);
        // Only AL carries the answer; the upper bytes are entry leftovers.
        if (ok & 0xFF) == 0 {
            return NOT_FOUND;
        }
        let rows = block[(ROWS_OFF / 4) as usize];
        let want =
            (rows.wrapping_add(index.wrapping_mul(4)) as *const u32).read_unaligned();
        if want == NOT_FOUND {
            return NOT_FOUND;
        }
        let count = block[(COUNT_OFF / 4) as usize];
        if count == 0 {
            return NOT_FOUND;
        }
        let ids = block[(IDS_OFF / 4) as usize];
        let mut i: u32 = 0;
        loop {
            let cur = (ids.wrapping_add(i.wrapping_mul(4)) as *const u32).read_unaligned();
            if cur == want {
                return i;
            }
            i = i.wrapping_add(1);
            if i >= count {
                return NOT_FOUND;
            }
        }
    }
});
