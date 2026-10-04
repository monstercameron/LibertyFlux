// original: 0x00536fd0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Race59Standard, player_schema::LeaderboardInfo, 10>::vf13

/// Value lookup by id for one ranked leaderboard (vf13).
///
/// Fetch the leaderboard tables through the fetch callee (fastcall: ECX =
/// leaderboard id, EDX = out-frame), then scan the id array. The callee answers
/// in AL (nonzero = ok) and fills count and table pointers into the frame.
/// `id` is scanned for linearly over `count` entries (signed bound: a count
/// at or below zero finds nothing); on a match the parallel value at the
/// same index is returned, otherwise -1. The original re-checks the found
/// index against -1 after the loop, which is unreachable (the index counts
/// up from 0) and is not repeated here. Leaderboard id: 0x96.
/// Original: stdcall of one stack word; incoming ECX is overwritten, not read.
lf_checker_rt::export!(stdcall, rw_00536fd0(id: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x96;
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
        let values = frame[5];
        let mut i = 0i32;
        loop {
            if i >= count {
                return NOT_FOUND;
            }
            if ids.add(i as usize).read_unaligned() == id {
                break;
            }
            i = i.wrapping_add(1);
        }
        (values.wrapping_add((i as u32).wrapping_mul(4)) as *const u32).read_unaligned()
    }
});
