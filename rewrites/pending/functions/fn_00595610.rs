// original: 0x00595610 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_261, player_schema::LeaderboardInfo, 10>::vf6
/// Ranked-leaderboard key position search (vf6 slot, race 261).
///
/// Returns the position of the first occurrence of `key` in the
/// leaderboard-0x1d9 candidate list, or `u32::MAX` when the helper fails,
/// the list is empty, or the key is absent.
export!(stdcall, rw_00595610(key: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x1d9;
        let mut out = [0u32; 5];
        let ok: u32 = callee_fastcall!(1, u32, LEADERBOARD_ID, out.as_mut_ptr() as u32);
        if ok & 0xff == 0 {
            return u32::MAX;
        }
        let count = out[3] as i32;
        let candidates = out[4];
        if count <= 0 {
            return u32::MAX;
        }
        let mut i = 0u32;
        while i < count as u32 {
            let cell = core::ptr::read(candidates.wrapping_add(i.wrapping_mul(4)) as *const u32);
            if cell == key {
                return i;
            }
            i = i.wrapping_add(1);
        }
        u32::MAX
    }
});
