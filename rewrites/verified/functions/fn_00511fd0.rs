// original: 0x00511fd0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_140,player_schema::LeaderboardInfo,10>::LeaderboardInfo,10>
// Constructor for one ranked-leaderboard info object (merged name
// below is low-confidence, from the class survey). Calls the shared
// base constructor, installs this class's primary vtable at +0x0 and
// the secondary vtable of the embedded base at +0x4a0, sets the
// initialised flag bit at +0x5a4, writes the -1 sentinel at +0x4a4
// and zeroes the five trailing words +0x4a8..+0x4b8. Returns `this`.
// Merged name: rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_140,player_schema::LeaderboardInfo,10>::LeaderboardInfo,10>.
lf_checker_rt::export!(thiscall, rw_00511fd0(this: *mut u8) -> u32 {
    unsafe {
        // Base-class constructor; intercepted and scripted by the checker.
        let _: u32 = lf_checker_rt::callee_thiscall!(1, u32, this as u32);
        // Primary vtable of this leaderboard class.
        *(this as *mut u32) = lf_checker_rt::relocated(0x00fdde5c);
        // Secondary vtable of the embedded base at +0x4a0.
        *((this.add(0x4a0)) as *mut u32) = lf_checker_rt::relocated(0x00fdb9a4);
        // Initialised flag bit.
        *this.add(0x5a4) |= 1;
        // Sentinel -1, then five zeroed words.
        *((this.add(0x4a4)) as *mut u32) = 0xffff_ffff;
        *((this.add(0x4a8)) as *mut u32) = 0;
        *((this.add(0x4ac)) as *mut u32) = 0;
        *((this.add(0x4b0)) as *mut u32) = 0;
        *((this.add(0x4b4)) as *mut u32) = 0;
        *((this.add(0x4b8)) as *mut u32) = 0;
        this as u32
    }
});
