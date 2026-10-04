// original: 0x005264a0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Race58NoHolds, player_schema::LeaderboardInfo, 10>::vf2

/// Tags a leaderboard handle after checking its generation.
///
/// Calls the wrapped object's generation slot (second vtable entry)
/// with `this`; when it equals `expect` and `out` is non-null, writes
/// this leaderboard's descriptor pointer (TAG, a file VA relocated
/// to the running image) and returns `out`, else returns 0.
/// thiscall: object in ecx, two stack words.
lf_checker_rt::export!(thiscall, rw_005264a0(this: u32, out: u32, expect: u32) -> u32 {
    unsafe {
        const VTBL_SLOT: u32 = 4;
        const TAG: u32 = 0xfde984;
        let vtbl = (this as *const u32).read_unaligned();
        let slot = (vtbl.wrapping_add(VTBL_SLOT) as *const u32).read_unaligned();
        let f: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(slot as usize);
        let got = f(this);
        if got != expect {
            return 0;
        }
        if out == 0 {
            return 0;
        }
        (out as *mut u32).write_unaligned(lf_checker_rt::relocated(TAG));
        out
    }
});
