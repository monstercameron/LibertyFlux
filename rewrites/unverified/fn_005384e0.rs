// original: 0x005384e0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_CompDeathmatch_BG, player_schema::LeaderboardInfo, 10>::vf2

/// Ranked-leaderboard slot check with id stamp (vf2).
///
/// Calls virtual slot 1 of `this` and compares the result with `want`. When
/// they match and `out` is non-null, writes the leaderboard stamp constant
/// 0xfd8194 to `out` and returns `out`; otherwise returns 0 (mismatch,
/// null `out`).
/// Original: thiscall with two stack words.
lf_checker_rt::export!(thiscall, rw_005384e0(this: u32, out: u32, want: u32) -> u32 {
    unsafe {
        const STAMP: u32 = 0xfd8194;
        const VTABLE_SLOT1: u32 = 4;
        const RANK_CALLEE: u32 = 1;
        let _ = RANK_CALLEE;
        let vtable = (this as *const u32).read_unaligned();
        let slot: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(
            ((vtable.wrapping_add(VTABLE_SLOT1)) as *const u32).read_unaligned() as usize);
        if slot(this) != want {
            return 0;
        }
        if out == 0 {
            return 0;
        }
        (out as *mut u32).write_unaligned(STAMP);
        out
    }
});
