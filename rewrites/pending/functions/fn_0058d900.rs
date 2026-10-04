// original: 0x0058d900 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_233,player_schema::LeaderboardInfo,10>::LeaderboardInfo,10>_2
/// Leaderboard key check: ask the object's second vtable slot for its key
/// and, when it matches `key` and `out` is non-null, store this
/// leaderboard's schema descriptor there. Returns `out`, or null on mismatch.
export!(thiscall, rw_0058d900(this_ptr: u32, out: u32, key: u32) -> u32 {
    unsafe {
        /// Schema descriptor stored on a key match (file VA; the worker
        /// relocates the original's immediate, so this must relocate too).
        const SCHEMA_DESC: u32 = 0xfcf5bc;
        let vtable = *(this_ptr as *const u32);
        let slot: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(*((vtable.wrapping_add(4)) as *const u32));
        if slot(this_ptr) != key {
            return 0;
        }
        if out == 0 {
            return 0;
        }
        *(out as *mut u32) = relocated(SCHEMA_DESC);
        out
    }
});
