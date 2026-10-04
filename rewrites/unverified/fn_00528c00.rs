// original: 0x00528C00 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Race7Standard, player_schema::LeaderboardInfo, 10>::vf2

/// Type tag of this race-7 leaderboard object, when its probe matches.
///
/// `this` points to the object. Its virtual slot TYPE_SLOT is called with
/// the same object and the answer is compared against `expected`; a mismatch
/// yields 0. A null `out` also yields 0. Otherwise the relocated tag is
/// stored through `out` and `out` itself is returned.
///
/// Original: 0x00528C00 (thiscall, two stack words).
lf_checker_rt::export!(thiscall, rw_00528c00(this: u32, out: u32, expected: u32) -> u32 {
    unsafe {
        /// Virtual slot of the probe, in bytes from the table base.
        const TYPE_SLOT: u32 = 4;
        /// Tag stored on a match, as a file VA; distinct per race schema.
        /// It is an image address (the worker relocates the original's
        /// immediate), so it must go through `relocated`, not in as a literal.
        const TYPE_TAG_FILEVA: u32 = 0x00FDDEEC;
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
        (out as *mut u32).write_unaligned(lf_checker_rt::relocated(TYPE_TAG_FILEVA));
        out
    }
});
