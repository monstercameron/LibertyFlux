// original: 0x005383b0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_CompDeathmatch, player_schema::LeaderboardInfo, 10>::vf8

/// Classified size rank for one ranked leaderboard (vf8).
///
/// Fetches the table through the fetch callee (fastcall: ECX = leaderboard
/// id, EDX = out-frame), reads the entry at `index`, and classifies it
/// through a second callee (thiscall of the entry value). The class maps to
/// a rank by jump table: 1 -> 4, 2 -> 8, 3 -> 8, 4 -> 0, 5 -> 4; a fetch
/// failure, class -1, or any other class yields 0. Leaderboard id: 0x7.
/// Original: stdcall of one stack word; incoming ECX is overwritten, not read.
lf_checker_rt::export!(stdcall, rw_005383b0(index: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x7;
        const FETCH_CALLEE: u32 = 1;
        const CLASSIFY_CALLEE: u32 = 2;
        const FAILED: u32 = 0xFFFF_FFFF;
        let mut frame = [0u32; 6];
        let answer: u32 = lf_checker_rt::callee_fastcall!(
            FETCH_CALLEE, u32, LEADERBOARD_ID, frame.as_mut_ptr() as u32);
        if answer & 0xFF == 0 {
            return 0;
        }
        let base = frame[5];
        let entry = (base.wrapping_add(index.wrapping_mul(4)) as *const u32).read_unaligned();
        let class: u32 = lf_checker_rt::callee_thiscall!(CLASSIFY_CALLEE, u32, entry);
        if class == FAILED {
            return 0;
        }
        match class.wrapping_sub(1) {
            0 => 4,
            1 | 2 => 8,
            3 => 0,
            4 => 4,
            _ => 0,
        }
    }
});
