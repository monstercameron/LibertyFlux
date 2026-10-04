// original: 0x0052F4C0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Race31Standard, player_schema::LeaderboardInfo, 10>::vf2
/// Publish this board's info vtable when the probe matches (race 31).
/// 
/// Thiscall: object in ecx, two stack words (`out`, `want`). Calls
/// the object's virtual slot 1 with the object pointer, and when the
/// answer equals `want` and `out` is non-null, writes this board's
/// info-table address into `*out`. Returns `out` on that path and 0
/// otherwise. The indirect call runs through the same fabricated
/// vtable on both sides, so the checker observes it.
lf_checker_rt::export!(thiscall, rw_0052f4c0(this: u32, out: u32, want: u32) -> u32 {
    unsafe {
        const VTABLE_SLOT: u32 = 4;
        const INFO_VTABLE: u32 = 0xfd0b1c;
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }

        let vtable = rd32(this);
        let slot = rd32(vtable.wrapping_add(VTABLE_SLOT));
        let probe: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(slot as usize);
        let got = probe(this);
        if got != want {
            return 0;
        }
        if out == 0 {
            return 0;
        }
        (out as *mut u32).write_unaligned(lf_checker_rt::relocated(INFO_VTABLE));
        out
    }
});
