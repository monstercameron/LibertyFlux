// original: 0x00592ab0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_251, player_schema::LeaderboardInfo, 10>::vf7

/// Return row `index` of the leaderboard array, or `NOT_FOUND` on failure.
///
/// Calls the leaderboard-info callee for board `LEADERBOARD_ID` with a six-word
/// out struct; word `+4` (row array) is read afterwards. There is no bounds
/// check: a wild index reads or faults exactly like the original (address
/// arithmetic wraps mod 2^32).
/// Original: stdcall, one stack word, the callee pops 4 bytes.
lf_checker_rt::export!(stdcall, rw_00592ab0(index: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x1d3;
        const INFO_CALLEE: u32 = 1;
        const NOT_FOUND: u32 = 0xffff_ffff;
        let mut info = [0u32; 6];
        let ok: u32 = lf_checker_rt::callee_fastcall!(INFO_CALLEE, u32, LEADERBOARD_ID, info.as_mut_ptr() as u32);
        if ok & 0xff == 0 {
            return NOT_FOUND;
        }
        let rows = info[4];
        (rows.wrapping_add(index.wrapping_mul(4)) as *const u32).read_unaligned()
    }
});
