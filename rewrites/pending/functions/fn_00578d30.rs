// original: 0x00578d30 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_157, player_schema::LeaderboardInfo, 10>::vf13
/// Look up a leaderboard key's value, or -1.
///
/// Fetches this leaderboard's key and value tables, scans the key table for `needle`, and on a hit returns the value at the same slot in the value table. Returns -1 on helper failure, an empty table, or a miss.
lf_checker_rt::export!(stdcall, rw_00578d30(needle: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x175;
        // Helper out-slots: entry count, key table, value table.
        let mut out = [0u32; 6];
        let ok: u32 = lf_checker_rt::callee_fastcall!(0, u32, LEADERBOARD_ID, out.as_mut_ptr() as u32);
        if ok & 0xFF == 0 {
            return 0xFFFF_FFFF;
        }
        let count = out[3] as i32;
        if count <= 0 {
            return 0xFFFF_FFFF;
        }
        let keys = out[4];
        let mut slot = 0u32;
        let mut found = 0u32;
        let mut hit = false;
        while slot < count as u32 {
            let at = keys.wrapping_add(slot.wrapping_mul(4));
            if (at as *const u32).read() == needle {
                found = slot;
                hit = true;
                break;
            }
            slot += 1;
        }
        if !hit {
            return 0xFFFF_FFFF;
        }
        let vals = out[5];
        let at = vals.wrapping_add(found.wrapping_mul(4));
        (at as *const u32).read()
    }
});
