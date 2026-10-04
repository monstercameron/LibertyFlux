// original: 0x00578700 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_155, player_schema::LeaderboardInfo, 10>::vf8
/// Classify the key in a leaderboard slot into a size bucket.
///
/// Fetches this leaderboard's key table, reads the key at `index`, and maps the classifier's codes 1 to 5 onto 4, 8, 8, 0 and 4. Returns 0 when the helper reports failure or the code falls outside 1 to 5.
lf_checker_rt::export!(stdcall, rw_00578700(index: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x173;
        // Helper out-slot: key-table pointer.
        let mut out = [0u32; 6];
        let ok: u32 = lf_checker_rt::callee_fastcall!(0, u32, LEADERBOARD_ID, out.as_mut_ptr() as u32);
        if ok & 0xFF == 0 {
            return 0;
        }
        let table = out[5];
        let at = table.wrapping_add(index.wrapping_mul(4));
        let key: u32 = (at as *const u32).read();
        let code: u32 = lf_checker_rt::callee_thiscall!(1, u32, key);
        match code {
            1 | 5 => 4,
            2 | 3 => 8,
            _ => 0,
        }
    }
});
