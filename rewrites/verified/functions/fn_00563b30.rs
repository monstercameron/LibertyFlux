// original: 0x00563b30 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_79, player_schema::LeaderboardInfo, 10>::vf9

/// Kind of one leaderboard entry: 0, 1, 2, 3 or -1.
///
/// `index` selects an entry of the leaderboard's value table; the entry's
/// classifier answer decides the kind. `_this` is ignored (the leaderboard
/// id is the constant `LEADERBOARD_ID`). The schema lookup callee fills a
/// six-word frame whose word 5 points at the value array.
///
/// Algorithm: run the lookup; on failure return -1. Classify
/// `values[index]`; a -1 answer returns -1. Otherwise subtract 1 and map
/// the result: 0->0, 1->1, 2->3, 3->-1, 4->2, anything else returns -1.
///
/// Edge cases: lookup failure, a -1 classification and an out-of-range
/// class all return -1; class 4 maps to -1 as well.
///
/// Original: 0x00563b30 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00563b30(_this: u32, index: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x11a;
        const INFO_VALUES: usize = 5;
        const SCHEMA_LOOKUP: u32 = 1;
        const CLASSIFY: u32 = 2;
        let mut info = [0u32; 6];
        let ok: u32 = lf_checker_rt::callee_fastcall!(
            SCHEMA_LOOKUP, u32, LEADERBOARD_ID, info.as_mut_ptr() as u32
        );
        if ok & 0xff == 0 {
            return 0xffff_ffff;
        }
        let values = info[INFO_VALUES] as *const u32;
        let entry = values.add(index as usize).read_unaligned();
        let class: u32 = lf_checker_rt::callee_thiscall!(CLASSIFY, u32, entry);
        if class == 0xffff_ffff {
            return 0xffff_ffff;
        }
        match class.wrapping_sub(1) {
            0 => 0,
            1 => 1,
            2 => 3,
            3 => 0xffff_ffff,
            4 => 2,

            _ => 0xffff_ffff,
        }
    }
});
