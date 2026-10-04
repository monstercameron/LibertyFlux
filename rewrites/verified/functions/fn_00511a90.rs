// original: 0x00511A90 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_126,player_schema::LeaderboardInfo,10>::LeaderboardInfo,10>
/// Constructor for the ranked episodic race 126 leaderboard info object.
///
/// Runs the shared base constructor on `this`, then installs this class's
/// two vtables, sets the initialised flag bit, stores the empty-slot marker
/// (-1) and clears the five trailing state words. Returns `this`.
export!(thiscall, rw_00511a90(this_ptr: u32) -> u32 {
    unsafe {
        // Base-class constructor (thiscall/0), intercepted by the checker.
        callee_thiscall!(1, u32, this_ptr);
        const VTABLE: u32 = 0x00fd8aa4;
        const VTABLE_INNER: u32 = 0x00fd26d4;
        const EMPTY_SLOT: u32 = 0xffff_ffff;
        (this_ptr as *mut u32).write(relocated(VTABLE));
        (this_ptr.wrapping_add(0x4a0) as *mut u32).write(relocated(VTABLE_INNER));
        let flags = this_ptr.wrapping_add(0x5a4) as *mut u8;
        flags.write(flags.read() | 1);
        (this_ptr.wrapping_add(0x4a4) as *mut u32).write(EMPTY_SLOT);
        (this_ptr.wrapping_add(0x4a8) as *mut u32).write(0);
        (this_ptr.wrapping_add(0x4ac) as *mut u32).write(0);
        (this_ptr.wrapping_add(0x4b0) as *mut u32).write(0);
        (this_ptr.wrapping_add(0x4b4) as *mut u32).write(0);
        (this_ptr.wrapping_add(0x4b8) as *mut u32).write(0);
        this_ptr
    }
});
