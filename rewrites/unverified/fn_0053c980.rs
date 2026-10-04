// original: 0x0053C980 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_TeamDeathmatch_BG, player_schema::LeaderboardInfo, 10>::vf8

/// Classify a leaderboard row's kind into a field width.
///
/// Queries table TABLE_ID for the row-key array (descriptor word 5), reads
/// the key at the argument position, and passes it to the kind classifier.
/// Classifier result minus one selects among five cases: 1 and 5 give width
/// 4, 2 and 3 give width 8, 4 gives 0. Anything else, including a failed
/// query or classifier answer -1, gives 0.
///
/// Original: thiscall with one stack word; ECX (this) is unused. The case
/// selection is the original's five-entry jump table written as a match.

lf_checker_rt::export!(thiscall, rw_0053c980(this: u32, index: u32) -> u32 {
    unsafe {
        const TABLE_ID: u32 = 0x34;
        const NOT_FOUND: u32 = 0xFFFF_FFFF;
        let mut info = [0u32; 6];
        let ok: u32 = lf_checker_rt::callee_fastcall!(1, u32, TABLE_ID, info.as_mut_ptr() as u32);
        if ok as u8 == 0 {
            return 0;
        }
        let items = info[5] as *const u32;
        let elem = items.wrapping_add(index as usize).read_unaligned();
        let r: u32 = lf_checker_rt::callee_thiscall!(2, u32, elem);
        if r == NOT_FOUND {
            return 0;
        }
        let d = r.wrapping_sub(1);
        if d > 4 {
            return 0;
        }
        match d {
            0 => 4,
            1 => 8,
            2 => 8,
            3 => 0,
            _ => 4,
        }
    }
});
