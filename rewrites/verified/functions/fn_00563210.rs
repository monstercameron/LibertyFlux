// original: 0x00563210 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_77, player_schema::LeaderboardInfo, 10>::vf8

/// Size class of one leaderboard entry: 4, 8 or 0.
///
/// `index` selects an entry of the leaderboard's value table; the entry's
/// classifier answer decides the size. `_this` is ignored (the leaderboard
/// id is the constant `LEADERBOARD_ID`). The schema lookup callee fills a
/// six-word frame whose word 5 points at the value array.
///
/// Algorithm: run the lookup; on failure return 0. Classify
/// `values[index]`; a -1 answer returns 0. Otherwise subtract 1 and map
/// the result: 0->4, 1->8, 2->8, 3->0, 4->4, anything else returns 0.
///
/// Edge cases: lookup failure, a -1 classification and an out-of-range
/// class all return 0.
///
/// Original: 0x00563210 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00563210(_this: u32, index: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x118;
        const INFO_VALUES: usize = 5;
        const SCHEMA_LOOKUP: u32 = 1;
        const CLASSIFY: u32 = 2;
        let mut info = [0u32; 6];
        let ok: u32 = lf_checker_rt::callee_fastcall!(
            SCHEMA_LOOKUP, u32, LEADERBOARD_ID, info.as_mut_ptr() as u32
        );
        if ok & 0xff == 0 {
            return 0;
        }
        let values = info[INFO_VALUES] as *const u32;
        let entry = values.add(index as usize).read_unaligned();
        let class: u32 = lf_checker_rt::callee_thiscall!(CLASSIFY, u32, entry);
        if class == 0xffff_ffff {
            return 0;
        }
        match class.wrapping_sub(1) {
            0 => 4,
            1 => 8,
            2 => 8,
            3 => 0,
            4 => 4,

            _ => 0,
        }
    }
});
