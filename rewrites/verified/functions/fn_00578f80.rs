// original: 0x00578f80 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_157, player_schema::LeaderboardInfo, 10>::vf7
/// Read one leaderboard key by slot index, or -1.
///
/// Fetches this leaderboard's key table through the shared data helper and returns the key stored at `index`. Returns -1 when the helper reports failure. The index is used unchecked, exactly like the original.
lf_checker_rt::export!(stdcall, rw_00578f80(index: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x175;
        // Helper out-slot: key-table pointer.
        let mut out = [0u32; 5];
        let ok: u32 = lf_checker_rt::callee_fastcall!(0, u32, LEADERBOARD_ID, out.as_mut_ptr() as u32);
        if ok & 0xFF == 0 {
            return 0xFFFF_FFFF;
        }
        let table = out[4];
        let at = table.wrapping_add(index.wrapping_mul(4));
        (at as *const u32).read()
    }
});
