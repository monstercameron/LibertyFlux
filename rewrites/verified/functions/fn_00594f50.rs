// original: 0x00594F50 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_260, player_schema::LeaderboardInfo, 10>::vf12
/// Ranked-leaderboard indexed-key search (vf12 slot, race 260).
///
/// Takes the key from slot `index` of the leaderboard-0x1d8 key table and
/// returns the position of its first occurrence in the candidate list, or
/// `u32::MAX` when the helper fails, the key is all-ones, the list is empty,
/// or the key is absent.
export!(stdcall, rw_00594f50(index: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x1d8;
        let mut out = [0u32; 6];
        let ok: u32 = callee_fastcall!(1, u32, LEADERBOARD_ID, out.as_mut_ptr() as u32);
        if ok & 0xff == 0 {
            return u32::MAX;
        }
        let count = out[1];
        let candidates = out[2];
        let keys = out[5];
        // Volatile: the original loads the key before testing the count, so
        // a wild index faults even when the list is empty; a plain read lets
        // the compiler sink the load under the count check and miss the fault.
        let key = core::ptr::read_volatile(keys.wrapping_add(index.wrapping_mul(4)) as *const u32);
        if key == u32::MAX || count == 0 {
            return u32::MAX;
        }
        let mut i = 0u32;
        while i < count {
            let cell = core::ptr::read(candidates.wrapping_add(i.wrapping_mul(4)) as *const u32);
            if cell == key {
                return i;
            }
            i = i.wrapping_add(1);
        }
        u32::MAX
    }
});
