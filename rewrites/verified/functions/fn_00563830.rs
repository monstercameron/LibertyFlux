// original: 0x00563830 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_79, player_schema::LeaderboardInfo, 10>::vf13

/// Value mapped from a key through this leaderboard's tables, or -1.
///
/// `value` is searched for in the leaderboard's key list; on a hit the
/// entry at the same position in the value table is returned. `_this` is
/// ignored (the leaderboard id is the constant `LEADERBOARD_ID`). The
/// schema lookup callee fills a six-word frame: word 3 is the key count,
/// word 4 points at the key array, word 5 points at the value array.
///
/// Algorithm: run the lookup; if it reports failure, return -1. If the
/// count is not positive, return -1. Scan `keys[0..count]` for `value`;
/// on a miss return -1. On a hit at position `i` (the original redundantly
/// re-checks `i == -1`, mirrored here), return `values[i]`.
///
/// Edge cases: lookup failure, a non-positive count and a miss all
/// return -1.
///
/// Original: 0x00563830 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00563830(_this: u32, value: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x11a;
        const INFO_COUNT: usize = 3;
        const INFO_KEYS: usize = 4;
        const INFO_VALUES: usize = 5;
        const SCHEMA_LOOKUP: u32 = 1;
        const MISSING: u32 = 0xffff_ffff;
        let mut info = [0u32; 6];
        let ok: u32 = lf_checker_rt::callee_fastcall!(
            SCHEMA_LOOKUP, u32, LEADERBOARD_ID, info.as_mut_ptr() as u32
        );
        if ok & 0xff == 0 {
            return MISSING;
        }
        let count = info[INFO_COUNT] as i32;
        if count <= 0 {
            return MISSING;
        }
        let keys = info[INFO_KEYS] as *const u32;
        let mut i = 0u32;
        while (i as i32) < count {
            if keys.add(i as usize).read_unaligned() == value {
                if i == MISSING {
                    return MISSING;
                }
                let values = info[INFO_VALUES] as *const u32;
                return values.add(i as usize).read_unaligned();
            }
            i += 1;
        }
        MISSING
    }
});
