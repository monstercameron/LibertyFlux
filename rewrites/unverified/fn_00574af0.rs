// original: 0x00574af0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_142, player_schema::LeaderboardInfo, 10>::vf2

/// Ranked-race leaderboard tag publication: ask this board's owner
/// object for its id (second vtable slot) and, when it matches
/// `expected` and `out` is non-null, store this board's tag word there.
///
/// Returns `out` on success, 0 when the id differs or `out` is null.
///
/// Original: 0x00574af0 (thiscall, two stack words).
lf_checker_rt::export!(thiscall, rw_00574af0(this: u32, out: u32, expected: u32) -> u32 {
    unsafe {
        const VTABLE_SLOT: usize = 1;
        const LEADERBOARD_TAG: u32 = 0xfd9a7c;
        let vtable = (this as *const u32).read_unaligned();
        let slot = (vtable as *const u32).add(VTABLE_SLOT).read_unaligned();
        let get_id: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(slot as usize);
        let id = get_id(this);
        if id != expected {
            return 0;
        }
        if out == 0 {
            return 0;
        }
        (out as *mut u32).write_unaligned(LEADERBOARD_TAG);
        out
    }
});
