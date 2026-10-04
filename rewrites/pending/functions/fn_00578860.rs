// original: 0x00578860 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_156, player_schema::LeaderboardInfo, 10>::vf12
/// Find where an indexed leaderboard key sits in the id table, or -1.
///
/// Fetches both leaderboard tables, reads the key at `index` from the key table, then linearly scans the id table for it and returns the first matching slot. Returns -1 on helper failure, an empty id table, a -1 key, or no match.
lf_checker_rt::export!(stdcall, rw_00578860(index: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x174;
        // Helper out-slots: id count, id-table pointer, key-table pointer.
        let mut out = [0u32; 6];
        let ok: u32 = lf_checker_rt::callee_fastcall!(0, u32, LEADERBOARD_ID, out.as_mut_ptr() as u32);
        if ok & 0xFF == 0 {
            return 0xFFFF_FFFF;
        }
        let keys = out[5];
        let at = keys.wrapping_add(index.wrapping_mul(4));
        let key: u32 = (at as *const u32).read();
        if key == 0xFFFF_FFFF {
            return 0xFFFF_FFFF;
        }
        let count = out[1];
        if count == 0 {
            return 0xFFFF_FFFF;
        }
        let ids = out[2];
        let mut slot = 0u32;
        while slot < count {
            let at = ids.wrapping_add(slot.wrapping_mul(4));
            if (at as *const u32).read() == key {
                return slot;
            }
            slot += 1;
        }
        0xFFFF_FFFF
    }
});
