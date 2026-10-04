// original: 0x00562620 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_75, player_schema::LeaderboardInfo, 10>::vf2

/// Leaderboard key gate: checks the object's schema id, then publishes one key.
///
/// `this` is the leaderboard-info object whose vtable slot 1 (at `+0x04`)
/// answers the schema id. `expected` is the id the caller wants; `out` is an
/// optional one-word sink (may be null).
///
/// Algorithm: call the slot-1 getter with `this`; if its answer differs from
/// `expected`, return 0. If `out` is null, return 0. Otherwise store
/// `LEADERBOARD_KEY` through `out` and return `out` itself.
///
/// Edge cases: a null `out` with a matching id returns 0 without storing;
/// a mismatch returns 0 without touching `out`.
///
/// Original: 0x00562620 (thiscall, `this` in ecx, two stack words).
lf_checker_rt::export!(thiscall, rw_00562620(this: u32, out: u32, expected: u32) -> u32 {
    unsafe {
        const VTABLE_SLOT_GET_ID: u32 = 0x04;
        const LEADERBOARD_KEY: u32 = 0x00fd202c;
        let vtable = (this as *const u32).read_unaligned();
        let get_id: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(
            ((vtable + VTABLE_SLOT_GET_ID) as *const u32).read_unaligned() as usize,
        );
        if get_id(this) != expected {
            return 0;
        }
        if out == 0 {
            return 0;
        }
        (out as *mut u32).write_unaligned(LEADERBOARD_KEY);
        out
    }
});
