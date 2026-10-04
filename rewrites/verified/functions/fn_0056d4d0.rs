// original: 0x0056d4d0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_115, player_schema::LeaderboardInfo, 10>::vf2
/// Publish this leaderboard's table address on validation match.
///
/// Calls slot 1 of the object's vtable (thiscall, `this` forwarded in ECX)
/// and compares the answer with `want`. On mismatch, or when `out` is null,
/// returns 0 and stores nothing. Otherwise stores PUBLISHED (this
/// instantiation's table address, a relocated image constant: the original
/// stores it from an immediate that carries a relocation entry, and the
/// worker maps the image away from its preferred base) through `out` and
/// returns `out`.
///
/// thiscall with two stack arguments (`out`, `want`); the only heap write is
/// the four bytes at `out`.
lf_checker_rt::export!(thiscall, rw_0056d4d0(this: u32, out: u32, want: u32) -> u32 {
    unsafe {
        const SLOT_ONE_BYTES: u32 = 4;
        const TABLE_FILE_VA: u32 = 0xfd40d4;
        let published: u32 = lf_checker_rt::relocated(TABLE_FILE_VA);
        let vtable = (this as *const u32).read();
        let slot = (vtable.wrapping_add(SLOT_ONE_BYTES) as *const u32).read();
        let validate: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(slot as usize);
        if validate(this) != want {
            return 0;
        }
        if out == 0 {
            return 0;
        }
        (out as *mut u32).write(published);
        out
    }
});
