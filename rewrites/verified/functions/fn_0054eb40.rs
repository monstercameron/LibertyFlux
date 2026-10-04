// original: 0x0054eb40 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_3,player_schema::LeaderboardInfo,10>::LeaderboardInfo,10>_2
// Guarded tag store, third leaderboard instantiation.
//
export!(thiscall, rs252_0054eb40(this_: u32, out: u32, want: u32) -> u32 {
    let vtable = unsafe { (this_ as *const u32).read() };
    let slot = unsafe { ((vtable.wrapping_add(4)) as *const u32).read() };
    let f: extern "thiscall" fn(u32) -> u32 =
        unsafe { core::mem::transmute(slot as usize) };
    let got = f(this_);
    if got != want {
        return 0;
    }
    if out == 0 {
        return 0;
    }
    unsafe { (out as *mut u32).write(relocated(0xfd0cf4)) };
    out
});
