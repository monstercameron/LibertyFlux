// original: 0x0053C590 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_TeamDeathmatch, player_schema::LeaderboardInfo, 10>::vf9

/// Classify a leaderboard row's kind into a small rank.
///
/// Same shape as its sibling width query: table TABLE_ID, row key at the
/// argument position through descriptor word 5, kind classifier, then a
/// five-way selection on the classifier result minus one, giving 0, 1, 3,
/// -1, 2 for results 1 through 5. Any other outcome gives -1.
///
/// Original: thiscall with one stack word; ECX (this) is unused.

lf_checker_rt::export!(thiscall, rw_0053c590(this: u32, index: u32) -> u32 {
    unsafe {
        const TABLE_ID: u32 = 0x4;
        const NOT_FOUND: u32 = 0xFFFF_FFFF;
        let mut info = [0u32; 6];
        let ok: u32 = lf_checker_rt::callee_fastcall!(1, u32, TABLE_ID, info.as_mut_ptr() as u32);
        if ok as u8 == 0 {
            return NOT_FOUND;
        }
        let items = info[5] as *const u32;
        let elem = items.wrapping_add(index as usize).read_unaligned();
        let r: u32 = lf_checker_rt::callee_thiscall!(2, u32, elem);
        if r == NOT_FOUND {
            return NOT_FOUND;
        }
        let d = r.wrapping_sub(1);
        if d > 4 {
            return NOT_FOUND;
        }
        match d {
            0 => 0,
            1 => 1,
            2 => 3,
            3 => NOT_FOUND,
            _ => 2,
        }
    }
});
