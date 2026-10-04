// original: 0x0054f400 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_5, player_schema::LeaderboardInfo, 10>::vf2
/// Look up this object's leaderboard id through its own virtual slot 1 and, when
/// it equals the wanted id, publish this variant's leaderboard tag through the
/// caller's out pointer.
///
/// `this` points to an object whose first word is its virtual table; slot 1
/// (`+4`) is called with `this` and returns the object's leaderboard id.
/// `want` is the expected id, `out` the caller's out pointer (may be null).
/// When the ids differ, or `out` is null, nothing is stored and 0 is returned.
/// Otherwise `LEADERBOARD_TAG` is stored to `out` and `out` is returned.
///
/// Original: thiscall, `this` in `ecx`, two stack words, callee pops 8.
lf_checker_rt::export!(thiscall, rw_0054f400(this: u32, out: u32, want: u32) -> u32 {
    unsafe {
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        const VTABLE_ID_SLOT: u32 = 0x04;
        const LEADERBOARD_TAG: u32 = 0xFD9FA4;
        const ID_CALLEE: u32 = 1;
        let vtable = rd32(this);
        let id_of: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(rd32(vtable.wrapping_add(VTABLE_ID_SLOT)) as usize);
        let got = id_of(this);
        if got != want {
            return 0;
        }
        if out == 0 {
            return 0;
        }
        wr32(out, LEADERBOARD_TAG);
        out
    }
});
