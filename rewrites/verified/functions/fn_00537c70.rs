// original: 0x00537c70 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_CompCarSteal_BG, player_schema::LeaderboardInfo, 10>::vf12

/// Reverse index lookup for one ranked leaderboard (vf12).
///
/// Fetches the tables through the fetch callee (fastcall: ECX = leaderboard
/// id, EDX = out-frame; AL answers nonzero on success), reads the key at
/// `index` from the key table, and scans the id array for it, returning the
/// matching position. Returns -1 when the fetch fails, the key is -1, the
/// count is zero, or the key is absent. Note the loop bound is unsigned
/// (`jb`): only a zero count skips the scan. Leaderboard id: 0x20.
/// Original: stdcall of one stack word; incoming ECX is overwritten, not read.
lf_checker_rt::export!(stdcall, rw_00537c70(index: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x20;
        const FETCH_CALLEE: u32 = 1;
        const NOT_FOUND: u32 = 0xFFFF_FFFF;
        let mut frame = [0u32; 6];
        let answer: u32 = lf_checker_rt::callee_fastcall!(
            FETCH_CALLEE, u32, LEADERBOARD_ID, frame.as_mut_ptr() as u32);
        if answer & 0xFF == 0 {
            return NOT_FOUND;
        }
        let keys = frame[5];
        let key = (keys.wrapping_add(index.wrapping_mul(4)) as *const u32).read_unaligned();
        if key == NOT_FOUND {
            return NOT_FOUND;
        }
        let count = frame[1];
        if count == 0 {
            return NOT_FOUND;
        }
        let ids = frame[2] as *const u32;
        let mut i = 0u32;
        loop {
            if ids.add(i as usize).read_unaligned() == key {
                return i;
            }
            i = i.wrapping_add(1);
            if i >= count {
                return NOT_FOUND;
            }
        }
    }
});
