// original: 0x0052A640 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Race13Standard, player_schema::LeaderboardInfo, 10>::vf2

/// Resolve this leaderboard's interface tag by its numeric id.
///
/// `this` points to the leaderboard-info object; its virtual slot at `+4`
/// answers with this instance's id. `want` is the requested id and `out`,
/// when non-null, receives the per-race interface tag on success. Returns
/// `out` when the probed id equals `want` and `out` is non-null, else null.
/// The stored tag is an absolute image address; the worker maps the image
/// rebased, so the rewrite derives it with `relocated` like the original's
/// rebased immediate.
///
/// Original: 0x0052A640 (thiscall, two stack words).
lf_checker_rt::export!(thiscall, rw_0052A640(this: u32, out: u32, want: u32) -> u32 {
    unsafe {
        const VTABLE_ID_SLOT: u32 = 4;
        const TYPE_TAG: u32 = 0x00FDD96C;
        let vtable = (this as *const u32).read_unaligned();
        let probe: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(
            (vtable.wrapping_add(VTABLE_ID_SLOT) as *const u32).read_unaligned() as usize,
        );
        if probe(this) != want {
            return 0;
        }
        if out == 0 {
            return 0;
        }
        (out as *mut u32).write_unaligned(lf_checker_rt::relocated(TYPE_TAG));
        out
    }
});
