// original: 0x00555700 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_27, player_schema::LeaderboardInfo, 10>::vf7

/// Fetch one leaderboard entry by position.
///
/// `index` selects a row of the entry table. The info callee (id 1, called with
/// `LEADERBOARD_ID` in ecx and a pointer to a five-word out-struct in edx)
/// fills `TABLE` (struct word 4); the word is zeroed before the call. When the
/// callee reports failure (al == 0) the result is `NOT_FOUND`, otherwise it is
/// the table word at `index`.
///
/// Original: 0x00555700 (stdcall, one stack word; ecx is ignored).
lf_checker_rt::export!(stdcall, rw_00555700(index: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0xed;
        const INFO_CALLEE: u32 = 1;
        const NOT_FOUND: u32 = 0xFFFF_FFFF;
        let mut info = [0u32; 5];
        info[4] = 0; // TABLE
        let ok: u32 = lf_checker_rt::callee_fastcall!(INFO_CALLEE, u32,
            LEADERBOARD_ID, info.as_mut_ptr() as u32);
        if ok as u8 == 0 {
            return NOT_FOUND;
        }
        (info[4].wrapping_add(index.wrapping_mul(4)) as *const u32).read_unaligned()
    }
});
