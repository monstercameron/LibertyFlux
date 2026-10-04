// original: 0x00583720 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_196,player_schema::LeaderboardInfo,10>::LeaderboardInfo,10>_2
/// rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_196,player_schema::LeaderboardInfo,10>::LeaderboardInfo,10>_2
///
/// Slot check: asks the object for its slot through its function table
/// and, when the answer equals `want` and `out` is not null, stores the
/// match tag there. Returns `out` on a stored match, else 0.
lf_checker_rt::export!(thiscall, rw_00583720(this_ptr: u32, out: u32, want: u32) -> u32 {
    // The stored tag is a link-time VA with a relocation entry: derive it
    // for the worker's mapping instead of using the file constant.
    let match_tag: u32 = lf_checker_rt::relocated(0xfdd354);
    let vtable = unsafe { (this_ptr as *const u32).read() };
    let slot: extern "thiscall" fn(u32) -> u32 =
        unsafe { core::mem::transmute((vtable.wrapping_add(4) as *const u32).read() as usize) };
    let got = slot(this_ptr);
    if got == want && out != 0 {
        unsafe { (out as *mut u32).write(match_tag) };
        return out;
    }
    0
});
