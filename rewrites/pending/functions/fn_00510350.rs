// original: 0x00510350 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_64,player_schema::LeaderboardInfo,10>::LeaderboardInfo,10>
/// Construct a ranked episodic-race leaderboard info object (race 64).
///
/// Calls the shared base constructor (stubbed thiscall/0) on `this`, then
/// installs this class's two vtables, sets the initialised flag, resets the
/// current index slot to none (-1) and clears the five trailing slots.
/// Returns `this`. Vtable addresses are relocated image addresses.
export!(thiscall, rw_00510350(this_ptr: u32) -> u32 {
    unsafe {
        const VTABLE_MAIN: u32 = 0x00FD9BEC;
        const VTABLE_SECONDARY: u32 = 0x00FD6EB4;
        const OFF_SECONDARY: usize = 0x4A0;
        const OFF_INDEX: usize = 0x4A4;
        const OFF_FLAG: usize = 0x5A4;
        let _: u32 = callee_thiscall!(2, u32, this_ptr);
        let base = this_ptr as *mut u8;
        core::ptr::write(base.add(0) as *mut u32, relocated(VTABLE_MAIN));
        core::ptr::write(base.add(OFF_SECONDARY) as *mut u32, relocated(VTABLE_SECONDARY));
        core::ptr::write(base.add(OFF_FLAG), core::ptr::read(base.add(OFF_FLAG)) | 1);
        core::ptr::write(base.add(OFF_INDEX) as *mut u32, 0xFFFF_FFFF);
        for slot in 0..5usize {
            core::ptr::write(base.add(OFF_INDEX + 4 + slot * 4) as *mut u32, 0);
        }
        this_ptr
    }
});
