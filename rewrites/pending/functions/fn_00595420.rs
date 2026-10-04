// original: 0x00595420 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_261, player_schema::LeaderboardInfo, 10>::vf13
/// Ranked-leaderboard key-to-slot lookup (vf13 slot, race 261).
///
/// Finds `key` in the leaderboard-0x1d9 candidate list and returns the
/// value from the parallel slot table at the same position, or `u32::MAX`
/// when the helper fails, the list is empty, or the key is absent. (The
/// original rechecks the found position against -1 afterwards; that check
/// cannot fire since positions are non-negative, so it is omitted here.)
export!(stdcall, rw_00595420(key: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x1d9;
        let mut out = [0u32; 6];
        let ok: u32 = callee_fastcall!(1, u32, LEADERBOARD_ID, out.as_mut_ptr() as u32);
        if ok & 0xff == 0 {
            return u32::MAX;
        }
        let count = out[3] as i32;
        let candidates = out[4];
        let slots = out[5];
        if count <= 0 {
            return u32::MAX;
        }
        let mut i = 0u32;
        while i < count as u32 {
            let cell = core::ptr::read(candidates.wrapping_add(i.wrapping_mul(4)) as *const u32);
            if cell == key {
                return core::ptr::read(slots.wrapping_add(i.wrapping_mul(4)) as *const u32);
            }
            i = i.wrapping_add(1);
        }
        u32::MAX
    }
});
