// original: 0x0058FC00 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_241, player_schema::LeaderboardInfo, 10>::vf2
/// Leaderboard token check: stamp `out` when the live token matches.
/// Calls the token source through this object's virtual slot at `+4`,
/// passing this object. When the returned token differs from `want`, or
/// `out` is null, returns 0. Otherwise writes the race table address
/// (file VA 0xfda6ec, relocated at load) to `out` and returns `out`.
/// Original: 0x0058FC00 (thiscall, two stack words).
lf_checker_rt::export!(thiscall, rw_0058fc00(this: u32, out: u32, want: u32) -> u32 {
    unsafe {
        const VTABLE_SLOT: u32 = 0x04;
        const RACE_TABLE_FILE_VA: u32 = 0xfda6ec;
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }

        let vtab = rd32(this);
        let token_of: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(rd32(vtab + VTABLE_SLOT) as usize);
        if token_of(this) != want {
            return 0;
        }
        if out == 0 {
            return 0;
        }
        (out as *mut u32).write_unaligned(lf_checker_rt::relocated(RACE_TABLE_FILE_VA));
        out
    }
});
