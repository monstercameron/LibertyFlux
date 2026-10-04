// original: 0x0050ED90 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_6,player_schema::LeaderboardInfo,10>::LeaderboardInfo,10>
/// Constructor of one concrete leaderboard-info class (Race_6).
///
/// Runs the shared base constructor, installs this class's primary vtable,
/// then initializes the trailing descriptor block: the secondary vtable, an
/// all-ones sentinel word, five zeroed words, and sets the initialized flag
/// bit. Returns the object pointer.
export!(thiscall, rw_0050ed90(this: *mut u8) -> u32 {
    unsafe {
        // Shared base constructor (intercepted by the checker); it leaves
        // the object pointer in place and its return value is not used.
        let _: u32 = callee_thiscall!(1, u32, this as u32);
        // Primary vtable of this concrete class.
        *(this as *mut u32) = relocated(0x00FDF63C);
        // Descriptor block vtable.
        *((this.add(0x4A0)) as *mut u32) = relocated(0x00FD874C);
        // Initialized flag.
        *(this.add(0x5A4)) |= 1;
        // Sentinel word, then zeroed words.
        *((this.add(0x4A4)) as *mut u32) = 0xFFFF_FFFF;
        *((this.add(0x4A8)) as *mut u32) = 0;
        *((this.add(0x4AC)) as *mut u32) = 0;
        *((this.add(0x4B0)) as *mut u32) = 0;
        *((this.add(0x4B4)) as *mut u32) = 0;
        *((this.add(0x4B8)) as *mut u32) = 0;
        this as u32
    }
});
