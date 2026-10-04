// original: 0x00580fc0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_187, player_schema::LeaderboardInfo, 10>::vf2

/// Leaderboard slot check: calls virtual slot 1 of the object in `this`
/// and compares the result with `expected`.
///
/// On a match with a non-null `out`, writes the per-race mark (file VA 0xfd0494,
/// relocated by the loader) to `*out` and returns `out`; otherwise returns 0
/// (mismatch, or match with null).
///
/// Original: thiscall, object in ECX plus two stack words, callee pops 8.
lf_checker_rt::export!(thiscall, rw_00580fc0(this: u32, out: u32, expected: u32) -> u32 {
    unsafe {
        const MARK_FILE_VA: u32 = 0xfd0494;
        const VTABLE_SLOT: u32 = 4;
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        let vtable = rd32(this);
        let target: extern "thiscall" fn(u32) -> u32 =
            unsafe { core::mem::transmute(rd32(vtable.wrapping_add(VTABLE_SLOT)) as usize) };
        let got = target(this);
        if got != expected {
            return 0;
        }
        if out == 0 {
            return 0;
        }
        // The stored mark is an image address: the original's immediate is
        // relocated by the loader, so derive it from the file VA.
        (out as *mut u32).write_unaligned(lf_checker_rt::relocated(MARK_FILE_VA));
        out
    }
});
