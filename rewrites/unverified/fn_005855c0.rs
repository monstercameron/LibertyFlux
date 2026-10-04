// original: 0x005855c0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_203, player_schema::LeaderboardInfo, 10>::vf2

/// Stamp the row marker when the oracle value matches the wanted one.
///
/// `this` points to an object whose first word is a function table; the slot
/// at `+4` is called with `this` and returns an oracle value. When it differs
/// from `want`, or when `out` is null, the result is 0. Otherwise the
/// instantiation's marker constant is stored to `*out` and `out` is returned.
///
/// Original: 0x005855c0 (thiscall: object in ECX, two stack words).
lf_checker_rt::export!(thiscall, rw_005855c0(this: u32, out: u32, want: u32) -> u32 {
    unsafe {
        const VTABLE_SLOT: u32 = 4;
        const ROW_MARKER: u32 = 0xfd250c;
        const NO_MATCH: u32 = 0;
        let vtable = (this as *const u32).read_unaligned();
        let slot = (vtable.wrapping_add(VTABLE_SLOT) as *const u32).read_unaligned();
        let oracle: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(slot as usize);
        let seen = oracle(this);
        if seen != want {
            return NO_MATCH;
        }
        if out == 0 {
            return NO_MATCH;
        }
        (out as *mut u32).write_unaligned(ROW_MARKER);
        out
    }
});
