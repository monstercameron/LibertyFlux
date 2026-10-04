// original: 0x0051c2f0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Race21NoHolds, player_schema::LeaderboardInfo, 10>::vf2

/// Publish this leaderboard descriptor address when the caller names its id.
///
/// Calls the second virtual slot of `this` (checker callee 1, thiscall
/// with `this` in ECX) to get the board id; returns 0 unless it equals
/// `want`. On a match with a non-null `out`, stores the relocated descriptor address and returns
/// `out`; on a match with a null `out`, returns 0. thiscall.
lf_checker_rt::export!(thiscall, rw_0051c2f0(this: u32, out: u32, want: u32) -> u32 {
    unsafe {
        const TAG_FILE_VA: u32 = 0x00FDC43C;
        const VTABLE_SLOT: u32 = 4;
        let vtbl = (this as *const u32).read_unaligned();
        let slot =
            (vtbl.wrapping_add(VTABLE_SLOT) as *const u32).read_unaligned();
        let f: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(slot as usize);
        let got = f(this);
        if got != want {
            return 0;
        }
        if out == 0 {
            return 0;
        }
        (out as *mut u32).write_unaligned(lf_checker_rt::relocated(TAG_FILE_VA));
        out
    }
});
