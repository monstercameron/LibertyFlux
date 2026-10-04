// original: 0x005251b0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Race53NoHolds, player_schema::LeaderboardInfo, 10>::vf7

/// Reads one entry of this leaderboard's column list.
///
/// Asks the registry for id LEADERBOARD_ID's column list (array
/// pointer at +16) and returns entry `index`, or -1 when the
/// lookup fails. stdcall, one stack word.
lf_checker_rt::export!(stdcall, rw_005251b0(index: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x39;
        const LOOKUP: u32 = 1;
        const ARRAY: usize = 4;
        const NOT_FOUND: u32 = 0xFFFF_FFFF;
        let mut buf = [0u32; 8];
        let ok: u32 = lf_checker_rt::callee_fastcall!(LOOKUP, u32, LEADERBOARD_ID, buf.as_mut_ptr() as u32);
        if ok as u8 == 0 {
            return NOT_FOUND;
        }
        let arr = buf[ARRAY] as u32;
        (arr.wrapping_add(index.wrapping_mul(4)) as *const u32).read_unaligned()
    }
});
