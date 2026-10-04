// original: 0x00510D70 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_91,player_schema::LeaderboardInfo,10>::LeaderboardInfo,10>
/// Constructor for the `Leaderboard_Ranked_Episodic_Race_91` leaderboard info object.
///
/// Runs the shared base constructor on the object, then installs this
/// class's primary and secondary virtual tables, sets the initialised flag
/// byte, stamps the generation word with -1, and zeroes the five trailing
/// state words. Returns the object pointer.
lf_checker_rt::export!(thiscall, rw_00510d70(this: u32) -> u32 {
    const PRIMARY_VTABLE: u32 = 0x00FDF24C;
    const SECOND_VTABLE: u32 = 0x00FD0BAC;
    const SECOND_VTABLE_OFF: usize = 0x4a0;
    const GENERATION_OFF: usize = 0x4a4;
    const STATE_OFF: usize = 0x4a8;
    const STATE_WORDS: usize = 5;
    const INITIALISED_FLAG_OFF: usize = 0x5a4;
    unsafe {
        let _ = lf_checker_rt::callee_thiscall!(0, u32, this);
        let obj = this as *mut u8;
        core::ptr::write_unaligned(
            obj as *mut u32,
            lf_checker_rt::relocated(PRIMARY_VTABLE),
        );
        core::ptr::write_unaligned(
            obj.add(SECOND_VTABLE_OFF) as *mut u32,
            lf_checker_rt::relocated(SECOND_VTABLE),
        );
        let flag = obj.add(INITIALISED_FLAG_OFF);
        core::ptr::write_unaligned(flag, core::ptr::read_unaligned(flag) | 1);
        core::ptr::write_unaligned(
            obj.add(GENERATION_OFF) as *mut u32,
            0xFFFF_FFFF,
        );
        for k in 0..STATE_WORDS {
            core::ptr::write_unaligned(
                (obj.add(STATE_OFF) as *mut u32).add(k),
                0,
            );
        }
    }
    this
});
