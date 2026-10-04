// original: 0x00537390 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Race60Standard, player_schema::LeaderboardInfo, 10>::vf2

/// Ranked-leaderboard slot check with id stamp (vf2).
///
/// Calls virtual slot 1 of `this` and compares the result with `want`. When
/// they match and `out` is non-null, writes the leaderboard stamp pointer
/// (file VA 0xfcea5c, relocated to the mapped image) to `out` and returns
/// `out`; otherwise returns 0 (mismatch, null `out`). The stamp is a
/// relocated address, not a plain constant: the immediate carries a base
/// relocation, so the rewrite derives it with `relocated`.
/// Original: thiscall with two stack words.
lf_checker_rt::export!(thiscall, rw_00537390(this: u32, out: u32, want: u32) -> u32 {
    unsafe {
        const STAMP_FILE_VA: u32 = 0xfcea5c;
        const VTABLE_SLOT1: u32 = 4;
        let vtable = (this as *const u32).read_unaligned();
        let slot: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(
            ((vtable.wrapping_add(VTABLE_SLOT1)) as *const u32).read_unaligned() as usize);
        if slot(this) != want {
            return 0;
        }
        if out == 0 {
            return 0;
        }
        (out as *mut u32).write_unaligned(lf_checker_rt::relocated(STAMP_FILE_VA));
        out
    }
});
