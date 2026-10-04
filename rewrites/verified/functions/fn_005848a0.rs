// original: 0x005848a0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_200, player_schema::LeaderboardInfo, 10>::vf2

/// Stamp the row marker when the oracle value matches the wanted one.
///
/// `this` points to an object whose first word is a function table; the slot
/// at `+4` is called with `this` and returns an oracle value. When it differs
/// from `want`, or when `out` is null, the result is 0. Otherwise a pointer
/// to this instantiation's row descriptor (an image address, relocated like
/// the original's) is stored to `*out` and `out` is returned.
///
/// Original: 0x005848a0 (thiscall: object in ECX, two stack words).
lf_checker_rt::export!(thiscall, rw_005848a0(this: u32, out: u32, want: u32) -> u32 {
    unsafe {
        const VTABLE_SLOT: u32 = 4;
        const ROW_DESCRIPTOR_FILE_VA: u32 = 0xfcebbc;
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
        (out as *mut u32).write_unaligned(lf_checker_rt::relocated(ROW_DESCRIPTOR_FILE_VA));
        out
    }
});
