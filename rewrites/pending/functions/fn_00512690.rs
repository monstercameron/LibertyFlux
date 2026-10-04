// original: 0x00512690 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_158,player_schema::LeaderboardInfo,10>::LeaderboardInfo,10>
/// Constructor for the ranked episodic-race-158 leaderboard info object.
///
/// Runs the shared base-class constructor on `this` (answered by the checker
/// stub), then installs this class's primary vtable and its secondary vtable
/// at +0x4a0, sets the initialised flag bit, writes the empty-slot tag (-1)
/// and zeroes the five trailing counter words. Returns `this`.
export!(thiscall, rw_00512690(this: *mut u8) -> u32 {
    // File VAs of this class's vtables; relocated to the worker mapping.
    const PRIMARY_VTBL: u32 = 0x00fcfd6c;
    const SECONDARY_VTBL: u32 = 0x00fdffc4;
    // Field offsets within the object.
    const SECONDARY_VTBL_OFF: usize = 0x4a0;
    const SLOT_TAG_OFF: usize = 0x4a4;
    const COUNTERS_OFF: usize = 0x4a8;
    const COUNTER_WORDS: usize = 5;
    const FLAGS_OFF: usize = 0x5a4;
    unsafe {
        // Shared base-class constructor; the checker stub answers it.
        let _base: u32 = callee_thiscall!(1, u32, this as u32);
        *(this as *mut u32) = relocated(PRIMARY_VTBL);
        *(this.add(SECONDARY_VTBL_OFF) as *mut u32) = relocated(SECONDARY_VTBL);
        *this.add(FLAGS_OFF) |= 1;
        *(this.add(SLOT_TAG_OFF) as *mut u32) = 0xffff_ffff;
        core::ptr::write_bytes(this.add(COUNTERS_OFF), 0, COUNTER_WORDS * 4);
        this as u32
    }
});
