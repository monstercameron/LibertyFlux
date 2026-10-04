// original: 0x0057ab30 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_164, player_schema::LeaderboardInfo, 10>::vf2
/// Publish this race's info constant when the live leaderboard value matches.
///
/// `this` is the leaderboard-info object: a table pointer is loaded from it
/// and the function pointer four bytes into that table is called with `this`,
/// yielding the current value for this race. When that value equals
/// `expected` and `out` is non-null, SLOT_VALUE (this race's info constant)
/// is stored through `out` and `out` is returned. Otherwise null is returned
/// and nothing is stored.
///
/// Edge cases: null `out` with a matching value returns null without storing;
/// a mismatching value returns null without reading `out`.
///
/// Original: thiscall, object in ECX, two stack words, callee cleans 8.
lf_checker_rt::export!(thiscall, rw_0057ab30(this: u32, out: u32, expected: u32) -> u32 {
    unsafe {
        const SLOT_VALUE: u32 = 0xfcff64;
        const VTABLE_SLOT_OFF: u32 = 4;

        #[inline(always)]
        unsafe fn rd32(addr: u32) -> u32 {
            unsafe { (addr as *const u32).read_unaligned() }
        }

        let table = rd32(this);
        let fetch: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(rd32(table.wrapping_add(VTABLE_SLOT_OFF)) as usize);
        if fetch(this) != expected {
            return 0;
        }
        if out == 0 {
            return 0;
        }
        (out as *mut u32).write_unaligned(SLOT_VALUE);
        out
    }
});
