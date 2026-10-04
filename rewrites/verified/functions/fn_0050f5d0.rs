// original: 0x0050f5d0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_28,player_schema::LeaderboardInfo,10>::LeaderboardInfo,10>
/// Constructor for the ranked episodic race 28 leaderboard update object.
///
/// Runs the shared base-class constructor, then installs this template
/// instantiation's vtable pair and initializes the embedded leaderboard-info
/// sub-object: an invalid-slot marker, zeroed counters and a set flag byte.
/// Returns the object pointer.
lf_checker_rt::export!(thiscall, rw_0050f5d0(this: u32) -> u32 {
    const UPDATE_VTABLE: u32 = 0x00FD1EAC;
    const INFO_VTABLE: u32 = 0x00FDFA3C;
    const INFO_OFF: usize = 0x4A0;
    const SLOT_OFF: usize = 0x4A4;
    const FLAG_OFF: usize = 0x5A4;
    const ZERO_WORDS: [usize; 5] = [0x4A8, 0x4AC, 0x4B0, 0x4B4, 0x4B8];
    lf_checker_rt::callee_thiscall!(1, u32, this);
    unsafe {
        let base = this as *mut u8;
        (base as *mut u32).write(lf_checker_rt::relocated(UPDATE_VTABLE));
        (base.add(INFO_OFF) as *mut u32).write(lf_checker_rt::relocated(INFO_VTABLE));
        *base.add(FLAG_OFF) |= 1;
        (base.add(SLOT_OFF) as *mut u32).write(u32::MAX);
        for off in ZERO_WORDS {
            (base.add(off) as *mut u32).write(0);
        }
    }
    this
});
