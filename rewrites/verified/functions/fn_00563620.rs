// original: 0x00563620 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_78, player_schema::LeaderboardInfo, 10>::vf7

/// One entry of this leaderboard's value table, or -1.
///
/// `index` selects an entry of the leaderboard's value table. `_this` is
/// ignored (the leaderboard id is the constant `LEADERBOARD_ID`). The
/// schema lookup callee fills a five-word frame whose word 4 points at
/// the value array.
///
/// Algorithm: run the lookup; if it reports failure, return -1.
/// Otherwise return `values[index]`.
///
/// Edge cases: only the lookup failure returns -1; every entry value,
/// including -1, is passed through.
///
/// Original: 0x00563620 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00563620(_this: u32, index: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x119;
        const INFO_VALUES: usize = 4;
        const SCHEMA_LOOKUP: u32 = 1;
        const MISSING: u32 = 0xffff_ffff;
        let mut info = [0u32; 5];
        let ok: u32 = lf_checker_rt::callee_fastcall!(
            SCHEMA_LOOKUP, u32, LEADERBOARD_ID, info.as_mut_ptr() as u32
        );
        if ok & 0xff == 0 {
            return MISSING;
        }
        let values = info[INFO_VALUES] as *const u32;
        values.add(index as usize).read_unaligned()
    }
});
