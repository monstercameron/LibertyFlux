// original: 0x0051cbb0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Race23NoHolds, player_schema::LeaderboardInfo, 10>::vf2

/// Publish this leaderboard descriptor address when the caller names its id.
///
/// Same shape as rw_0051c2f0 with descriptor file VA 0x00FCF03C. thiscall.
lf_checker_rt::export!(thiscall, rw_0051cbb0(this: u32, out: u32, want: u32) -> u32 {
    unsafe {
        const TAG_FILE_VA: u32 = 0x00FCF03C;
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
