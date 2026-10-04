// original: 0x00537730 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Race60Standard, player_schema::LeaderboardInfo, 10>::vf9

/// Classified kind rank for one ranked leaderboard (vf9).
///
/// Same shape as vf8: fetch the table (fastcall fetch callee with
/// leaderboard id 0x9d), read the entry at `index`, classify it through
/// the second callee. The class maps by jump table: 1 -> 0, 2 -> 1, 3 -> 3,
/// 4 -> -1, 5 -> 2; a fetch failure, class -1, or any other class yields -1.
/// Original: stdcall of one stack word; incoming ECX is overwritten, not read.
lf_checker_rt::export!(stdcall, rw_00537730(index: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x9d;
        const FETCH_CALLEE: u32 = 1;
        const CLASSIFY_CALLEE: u32 = 2;
        const FAILED: u32 = 0xFFFF_FFFF;
        let mut frame = [0u32; 6];
        let answer: u32 = lf_checker_rt::callee_fastcall!(
            FETCH_CALLEE, u32, LEADERBOARD_ID, frame.as_mut_ptr() as u32);
        if answer & 0xFF == 0 {
            return FAILED;
        }
        let base = frame[5];
        let entry = (base.wrapping_add(index.wrapping_mul(4)) as *const u32).read_unaligned();
        let class: u32 = lf_checker_rt::callee_thiscall!(CLASSIFY_CALLEE, u32, entry);
        if class == FAILED {
            return FAILED;
        }
        match class.wrapping_sub(1) {
            0 => 0,
            1 => 1,
            2 => 3,
            3 => FAILED,
            4 => 2,
            _ => FAILED,
        }
    }
});
