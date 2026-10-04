// original: 0x00528A30 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Race6Standard, player_schema::LeaderboardInfo, 10>::vf6

/// Index of a leaderboard row id in the race-6 id array, or -1.
///
/// `key` is the row id to find. `this` (ECX) is ignored: the schema is
/// selected by the constant id SCHEMA_ID handed to the lookup. The lookup
/// (callee 1) answers ok/failed in AL and, when ok, writes the row count at
/// scratch-block offset COUNT_OFF and a pointer to the id array at IDS_OFF.
/// A failed lookup, or a count that is zero or negative (compared signed),
/// yields NOT_FOUND. Otherwise the array is searched front to back and the
/// first index holding `key` is returned, or NOT_FOUND when none matches.
/// The search bound is signed: a very large unsigned count that is negative
/// as i32 still yields NOT_FOUND without reading the array.
///
/// Original: 0x00528A30 (thiscall, one stack word; ECX ignored).
lf_checker_rt::export!(thiscall, rw_00528a30(_this: u32, key: u32) -> u32 {
    unsafe {
        /// Schema id selecting the race-6 leaderboard.
        const SCHEMA_ID: u32 = 0x54;
        /// Byte offset of the row count in the lookup out-block.
        const COUNT_OFF: u32 = 12;
        /// Byte offset of the id-array pointer in the lookup out-block.
        const IDS_OFF: u32 = 16;
        const NOT_FOUND: u32 = 0xFFFF_FFFF;
        let mut block = [0u32; 5];
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
        loop {
            let cur = (ids.wrapping_add(i.wrapping_mul(4)) as *const u32).read_unaligned();
            if cur == key {
                return i;
            }
            i = i.wrapping_add(1);
            if (i as i32) >= count {
                return NOT_FOUND;
            }
        }
    }
});
