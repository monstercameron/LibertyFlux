// original: 0x00551b60 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_14, player_schema::LeaderboardInfo, 10>::vf2

/// Publish this board's row id when the probed id matches.
///
/// Calls the second virtual slot of the object in `this` (an indirect call
/// through the object's table, intercepted by planting) and compares the
/// answer with `want`. When they match and `out` is non-null, stores this
/// board's row id there and returns `out`; otherwise returns null (a null
/// `out` with a matching id also returns null without storing).
///
/// Original: 0x00551b60 (thiscall: object in ecx, two stack words).
lf_checker_rt::export!(thiscall, rw_00551b60(this: u32, out: u32, want: u32) -> u32 {
    unsafe {
        const ROW_ID: u32 = 0xFDDA54;
        const PROBE_SLOT: usize = 1;
        let vtable = (this as *const u32).read_unaligned();
        let probe: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(
            ((vtable as *const u32).add(PROBE_SLOT)).read_unaligned() as usize);
        if probe(this) != want {
            return 0;
        }
        if out == 0 {
            return 0;
        }
        (out as *mut u32).write_unaligned(ROW_ID);
        out
    }
});
