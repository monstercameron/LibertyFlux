// original: 0x00516ff0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Race2NoHolds, player_schema::LeaderboardInfo, 10>::vf2
/// Leaderboard QueryInterface-style probe: tag `out` when the id matches.
///
/// Loads the object's function table from `[this]` and calls slot `+4` with
/// `this` in ECX. When that answer differs from `expected`, or when `out` is
/// null, returns 0. Otherwise stores the per-board tag pointer (file address
/// 0xfd9adc, relocated at load like the original's own immediate) at `out`
/// and returns `out`. Thiscall with two stack arguments.
lf_checker_rt::export!(thiscall, rw_00516ff0(this: u32, out: u32, expected: u32) -> u32 {
    unsafe {
        const SLOT_OFF: u32 = 4;
        const TAG_FILE_VA: u32 = 0xfd9adc;
        let vt = (this as *const u32).read_unaligned();
        let slot = ((vt.wrapping_add(SLOT_OFF)) as *const u32).read_unaligned();
        let f: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(slot as usize);
        let got = f(this);
        if got != expected {
            return 0;
        }
        if out == 0 {
            return 0;
        }
        (out as *mut u32).write_unaligned(lf_checker_rt::relocated(TAG_FILE_VA));
        out
    }
});
