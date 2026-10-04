// original: 0x00579030 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_157, player_schema::LeaderboardInfo, 10>::vf9
/// Map the key in a leaderboard slot to its kind code.
///
/// Fetches this leaderboard's key table, reads the key at `index`, and maps the classifier's codes 1 to 5 onto 0, 1, 3, -1 and 2. Returns -1 when the helper reports failure or the code falls outside 1 to 5.
lf_checker_rt::export!(stdcall, rw_00579030(index: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x175;
        // Helper out-slot: key-table pointer.
        let mut out = [0u32; 6];
        let ok: u32 = lf_checker_rt::callee_fastcall!(0, u32, LEADERBOARD_ID, out.as_mut_ptr() as u32);
        if ok & 0xFF == 0 {
            return 0xFFFF_FFFF;
        }
        let table = out[5];
        let at = table.wrapping_add(index.wrapping_mul(4));
        let key: u32 = (at as *const u32).read();
        let code: u32 = lf_checker_rt::callee_thiscall!(1, u32, key);
        match code {
            1 => 0,
            2 => 1,
            3 => 3,
            4 => 0xFFFF_FFFF,
            5 => 2,
            _ => 0xFFFF_FFFF,
        }
    }
});
