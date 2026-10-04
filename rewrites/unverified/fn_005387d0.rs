// original: 0x005387d0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_CompDeathmatch_BG, player_schema::LeaderboardInfo, 10>::vf7

/// Table element read by index for one ranked leaderboard (vf7).
///
/// Fetches the leaderboard table through the fetch callee (fastcall: ECX =
/// leaderboard id, EDX = out-frame; AL answers nonzero on success) and
/// returns the word at `index` with no bounds check. Returns -1 when the
/// fetch fails; a wild index reads (or faults on) whatever the address
/// reaches, exactly like the original. Leaderboard id: 0x17.
/// Original: stdcall of one stack word; incoming ECX is overwritten, not read.
lf_checker_rt::export!(stdcall, rw_005387d0(index: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x17;
        const FETCH_CALLEE: u32 = 1;
        const NOT_FOUND: u32 = 0xFFFF_FFFF;
        let mut frame = [0u32; 5];
        let answer: u32 = lf_checker_rt::callee_fastcall!(
            FETCH_CALLEE, u32, LEADERBOARD_ID, frame.as_mut_ptr() as u32);
        if answer & 0xFF == 0 {
            return NOT_FOUND;
        }
        let base = frame[4];
        (base.wrapping_add(index.wrapping_mul(4)) as *const u32).read_unaligned()
    }
});
