// original: 0x00578f20 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_157, player_schema::LeaderboardInfo, 10>::vf6
/// Find a leaderboard key's position, or -1.
///
/// Fetches this leaderboard's key table through the shared data helper, then linearly scans it for `needle` and returns the first matching slot index. Returns -1 (all bits set) when the helper reports failure, the table is empty, or no slot holds the key.
lf_checker_rt::export!(stdcall, rw_00578f20(needle: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x175;
        // Helper out-slots: entry count, then key-table pointer.
        let mut out = [0u32; 6];
        let ok: u32 = lf_checker_rt::callee_fastcall!(0, u32, LEADERBOARD_ID, out.as_mut_ptr() as u32);
        if ok & 0xFF == 0 {
            return 0xFFFF_FFFF;
        }
        let count = out[3] as i32;
        let table = out[4];
        if count <= 0 {
            return 0xFFFF_FFFF;
        }
        let mut slot = 0u32;
        while slot < count as u32 {
            let at = table.wrapping_add(slot.wrapping_mul(4));
            if (at as *const u32).read() == needle {
                return slot;
            }
            slot += 1;
        }
        0xFFFF_FFFF
    }
});
