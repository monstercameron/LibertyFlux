// original: 0x00528CA0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Race7Standard, player_schema::LeaderboardInfo, 10>::vf13

/// Leaderboard value for a row id, looked up through the race-7 tables.
///
/// `key` is the row id to find. `this` (ECX) is ignored: the schema is
/// selected by the constant id SCHEMA_ID handed to the lookup. The lookup
/// (callee 1) answers ok/failed in AL and, when ok, writes the row count at
/// scratch-block offset COUNT_OFF, the id-array pointer at IDS_OFF and the
/// value-array pointer at VALS_OFF. A failed lookup, or a count that is zero
/// or negative (compared signed), yields NOT_FOUND. Otherwise the id array
/// is searched front to back; when `key` is found, the value at the same
/// index of the value array is returned, else NOT_FOUND. (The original then
/// compares the found index against -1, which can never match: the index
/// counts up from zero.)
///
/// Original: 0x00528CA0 (thiscall, one stack word; ECX ignored).
lf_checker_rt::export!(thiscall, rw_00528ca0(_this: u32, key: u32) -> u32 {
    unsafe {
        /// Schema id selecting the race-7 leaderboard.
        const SCHEMA_ID: u32 = 0x55;
        /// Byte offset of the row count in the lookup out-block.
        const COUNT_OFF: u32 = 12;
        /// Byte offset of the id-array pointer in the lookup out-block.
        const IDS_OFF: u32 = 16;
        /// Byte offset of the value-array pointer in the lookup out-block.
        const VALS_OFF: u32 = 20;
        const NOT_FOUND: u32 = 0xFFFF_FFFF;
        let mut block = [0u32; 6];
        let ok: u32 =
            lf_checker_rt::callee_fastcall!(1, u32, SCHEMA_ID, block.as_mut_ptr() as u32);
        // Only AL carries the answer; the upper bytes are entry leftovers.
        if (ok & 0xFF) == 0 {
            return NOT_FOUND;
        }
        let count = block[(COUNT_OFF / 4) as usize] as i32;
        if count <= 0 {
            return NOT_FOUND;
        }
        let ids = block[(IDS_OFF / 4) as usize];
        let mut i: u32 = 0;
        let found: u32 = loop {
            let cur = (ids.wrapping_add(i.wrapping_mul(4)) as *const u32).read_unaligned();
            if cur == key {
                break i;
            }
            i = i.wrapping_add(1);
            if (i as i32) >= count {
                return NOT_FOUND;
            }
        };
        let vals = block[(VALS_OFF / 4) as usize];
        (vals.wrapping_add(found.wrapping_mul(4)) as *const u32).read_unaligned()
    }
});
