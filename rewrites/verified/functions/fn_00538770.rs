// original: 0x00538770 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_CompDeathmatch_BG, player_schema::LeaderboardInfo, 10>::vf6

/// Index lookup by id for one ranked leaderboard (vf6).
///
/// Fetch the leaderboard tables through the fetch callee (fastcall: ECX =
/// leaderboard id, EDX = out-frame), then scan the id array. The callee answers
/// in AL (nonzero = ok) and fills count and table pointers into the frame.
/// `id` is scanned for linearly over `count` entries (signed bound); the
/// matching index is returned, or -1 when the fetch fails, the count is not
/// positive, or the id is absent. Leaderboard id: 0x17.
/// Original: stdcall of one stack word; incoming ECX is overwritten, not read.
lf_checker_rt::export!(stdcall, rw_00538770(id: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x17;
        const FETCH_CALLEE: u32 = 1;
        const NOT_FOUND: u32 = 0xFFFF_FFFF;
        let mut frame = [0u32; 6];
        let answer: u32 = lf_checker_rt::callee_fastcall!(
            FETCH_CALLEE, u32, LEADERBOARD_ID, frame.as_mut_ptr() as u32);
        if answer & 0xFF == 0 {
            return NOT_FOUND;
        }
        let count = frame[3] as i32;
        if count <= 0 {
            return NOT_FOUND;
        }
        let ids = frame[4] as *const u32;
        let mut i = 0i32;
        loop {
            if i >= count {
                return NOT_FOUND;
            }
            if ids.add(i as usize).read_unaligned() == id {
                return i as u32;
            }
            i = i.wrapping_add(1);
        }
    }
});
