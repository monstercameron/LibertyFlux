// original: 0x00578c90 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_157, player_schema::LeaderboardInfo, 10>::vf2
/// Query a leaderboard object for a known tag.
///
/// Asks the object, through its second vtable slot, for its tag id. When the answer equals `want` and `sink` is non-null, stores this instance's tag address there and returns `sink`. Returns null otherwise.
lf_checker_rt::export!(thiscall, rw_00578c90(this_obj: u32, sink: u32, want: u32) -> u32 {
    unsafe {
        // Tag record this instance publishes (file VA; relocated at load).
        const TAG: u32 = 0x00FDF92C;
        let vtable = (this_obj as *const u32).read();
        let slot = (vtable.wrapping_add(4) as *const u32).read();
        let ask: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(slot as usize);
        if ask(this_obj) != want {
            return 0;
        }
        if sink == 0 {
            return 0;
        }
        (sink as *mut u32).write(lf_checker_rt::relocated(TAG));
        sink
    }
});
