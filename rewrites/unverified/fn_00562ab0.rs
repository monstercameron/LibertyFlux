// original: 0x00562ab0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_76, player_schema::LeaderboardInfo, 10>::vf12

/// Index of a looked-up key inside this leaderboard's key list, or -1.
///
/// `index` selects one entry of the leaderboard's value table; the value
/// found there is searched for in the key list. `_this` is ignored (the
/// leaderboard id is the constant `LEADERBOARD_ID`). The schema lookup
/// callee fills a six-word frame: word 1 is the key count, word 2 points at
/// the key array, word 5 points at the value array.
///
/// Algorithm: run the lookup; if it reports failure, return -1. Read
/// `values[index]`; if it is -1, return -1. If the count is 0, return -1.
/// Otherwise scan `keys[0..count]` for the value and return its position,
/// or -1 when absent.
///
/// Edge cases: lookup failure, a -1 value, an empty list and a miss all
/// return -1; an empty list is checked before the scan.
///
/// Original: 0x00562ab0 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00562ab0(_this: u32, index: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x117;
        const INFO_COUNT: usize = 1;
        const INFO_KEYS: usize = 2;
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
        let values = info[INFO_VALUES] as *const u32;
        let want = values.add(index as usize).read_unaligned();
        if want == MISSING {
            return MISSING;
        }
        let count = info[INFO_COUNT];
        if count == 0 {
            return MISSING;
        }
        let keys = info[INFO_KEYS] as *const u32;
        let mut i = 0u32;
        while i < count {
            if keys.add(i as usize).read_unaligned() == want {
                return i;
            }
            i += 1;
        }
        MISSING
    }
});
