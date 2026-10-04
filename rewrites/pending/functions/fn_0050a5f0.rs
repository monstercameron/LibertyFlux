// original: 0x0050A5F0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Race18NoHolds,player_schema::LeaderboardInfo,10>::LeaderboardInfo,10>
/// Constructor for one leaderboard-info specialization (network subsystem).
///
/// Calls the shared base-class constructor, then installs this
/// specialization's virtual table, initializes the embedded member object at
/// `+0x4a0` (its own virtual table, a `-1` tag word and zeroed fields) and
/// sets the initialized flag bit at `+0x5a4`. Returns the object pointer.
lf_checker_rt::export!(thiscall, rw_0050a5f0(this_ptr: u32) -> u32 {
    // Base-class constructor (intercepted and scripted by the checker).
    lf_checker_rt::callee_thiscall!(2, u32, this_ptr);
    unsafe {
        let obj = this_ptr as *mut u8;
        // Virtual table of this specialization.
        (obj as *mut u32).write(lf_checker_rt::relocated(0x00FD9894));
        // Embedded member: its virtual table, -1 tag, zeroed trailing words.
        (obj.add(0x4A0) as *mut u32).write(lf_checker_rt::relocated(0x00FDE68C));
        // Initialized flag.
        *obj.add(0x5A4) |= 1;
        (obj.add(0x4A4) as *mut u32).write(0xFFFF_FFFF);
        (obj.add(0x4A8) as *mut u32).write(0);
        (obj.add(0x4AC) as *mut u32).write(0);
        (obj.add(0x4B0) as *mut u32).write(0);
        (obj.add(0x4B4) as *mut u32).write(0);
        (obj.add(0x4B8) as *mut u32).write(0);
    }
    this_ptr
});
