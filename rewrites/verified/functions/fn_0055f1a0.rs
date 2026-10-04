// original: 0x0055f1a0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_63, player_schema::LeaderboardInfo, 10>::vf2

/// Identify a leaderboard object and publish this class's vtable.
///
/// `this` points at a leaderboard object whose first word points at its
/// identity table; slot 1 of that table (byte offset 4) is called with `this`
/// and its answer compared against `want`. When they differ, or when `out` is
/// null, the result is 0. Otherwise this class's vtable address (0xfdba5c as a
/// file address) is stored through `out` and `out` itself is returned.
///
/// Original: 0x0055f1a0 (thiscall, two stack words).
lf_checker_rt::export!(thiscall, rw_0055f1a0(this: u32, out: u32, want: u32) -> u32 {
    unsafe {
        const IDENTIFY_SLOT_OFF: u32 = 4;
        const CLASS_VTABLE: u32 = 0xfdba5c;
        const FAILED: u32 = 0;
        let table = (this as *const u32).read();
        let slot = table.wrapping_add(IDENTIFY_SLOT_OFF) as *const u32;
        let identify: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(slot.read() as usize);
        if identify(this) != want {
            return FAILED;
        }
        if out == 0 {
            return FAILED;
        }
        (out as *mut u32).write(lf_checker_rt::relocated(CLASS_VTABLE));
        out
    }
});
