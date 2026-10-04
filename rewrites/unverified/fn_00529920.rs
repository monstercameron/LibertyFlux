// original: 0x00529920 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Race10Standard, player_schema::LeaderboardInfo, 10>::vf2

/// Type tag of this race-10 leaderboard object, when its probe matches.
///
/// `this` points to the object. Its virtual slot TYPE_SLOT is called with
/// the same object and the answer is compared against `expected`; a mismatch
/// yields 0. A null `out` also yields 0. Otherwise TYPE_TAG is stored through
/// `out` and `out` itself is returned.
///
/// Original: 0x00529920 (thiscall, two stack words).
lf_checker_rt::export!(thiscall, rw_00529920(this: u32, out: u32, expected: u32) -> u32 {
    unsafe {
        /// Virtual slot of the probe, in bytes from the table base.
        const TYPE_SLOT: u32 = 4;
        /// Tag stored on a match; distinct per race schema.
        const TYPE_TAG: u32 = 0x00FDD8DC;
        let vtable = (this as *const u32).read_unaligned();
        let probe: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(
            ((vtable.wrapping_add(TYPE_SLOT)) as *const u32).read_unaligned() as usize,
        );
        if probe(this) != expected {
            return 0;
        }
        if out == 0 {
            return 0;
        }
        (out as *mut u32).write_unaligned(TYPE_TAG);
        out
    }
});
