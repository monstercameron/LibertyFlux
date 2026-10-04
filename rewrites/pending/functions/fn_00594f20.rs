// original: 0x00594F20 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_260, player_schema::LeaderboardInfo, 10>::vf2
/// Ranked-leaderboard info probe (vf2 slot, race 260).
///
/// Probes the object through the second slot of its own virtual table and,
/// when the probe answer equals `expected` and `out` is non-null, stores the
/// leaderboard info pointer there and returns `out`; otherwise returns 0.
export!(thiscall, rw_00594f20(this: u32, out: u32, expected: u32) -> u32 {
    unsafe {
        const INFO_PTR: u32 = 0x00FD135C;
        let vtable = core::ptr::read(this as *const u32);
        let probe: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(
            core::ptr::read(vtable.wrapping_add(4) as *const u32) as usize,
        );
        if probe(this) != expected {
            return 0;
        }
        if out == 0 {
            return 0;
        }
        core::ptr::write(out as *mut u32, relocated(INFO_PTR));
        out
    }
});
