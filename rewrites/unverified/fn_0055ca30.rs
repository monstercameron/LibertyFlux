// original: 0x0055ca30 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_54, player_schema::LeaderboardInfo, 10>::vf2
/// Resolve this leaderboard's id through its owner and publish it on match.
///
/// `this` is the leaderboard-info object; its virtual slot at `+0x04` returns
/// the owner's current id. When that id equals `want` and `out` is non-null,
/// this leaderboard's constant address `LEADERBOARD_ID` (a file VA, relocated
/// at load like the original's) is stored to `*out` and `out` is returned;
/// otherwise null is returned and nothing is stored.
///
/// Original: 0x0055ca30 (thiscall, two stack words).
lf_checker_rt::export!(thiscall, rw_0055ca30(this: u32, out: u32, want: u32) -> u32 {
    unsafe {
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        const SLOT_GET_ID: u32 = 0x04;
        const LEADERBOARD_ID: u32 = 0xfdd10c;
        let vtable = rd32(this);
        let get_id: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(rd32(vtable + SLOT_GET_ID) as usize);
        if get_id(this) != want {
            return 0;
        }
        if out == 0 {
            return 0;
        }
        (out as *mut u32).write_unaligned(lf_checker_rt::relocated(LEADERBOARD_ID));
        out
    }
});
