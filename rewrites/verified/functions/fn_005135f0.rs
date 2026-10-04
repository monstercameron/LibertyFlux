// original: 0x005135F0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_199,player_schema::LeaderboardInfo,10>::LeaderboardInfo,10>
/// Constructor for the ranked episodic race 199 leaderboard-info object.
///
/// Calls the shared base constructor, installs this class' two vtables,
/// sets the initialised flag bit, writes the -1 sentinel and clears the
/// five trailing state words, then returns the object pointer.
export!(thiscall, rw_005135f0(this: *mut u8) -> u32 {
    unsafe {
        /// Primary vtable installed at offset 0 (file VA; relocated at load).
        const VTABLE: u32 = 0x00FD04DC;
        /// Secondary vtable installed at offset 0x4A0 (file VA; relocated at load).
        const VTABLE2: u32 = 0x00FDA214;
        // Base-class constructor (intercepted by the checker on both sides).
        let _: u32 = callee_thiscall!(1, u32, this as u32);
        *(this as *mut u32) = relocated(VTABLE);
        *(this.add(0x4A0) as *mut u32) = relocated(VTABLE2);
        // Flag byte: set bit 0, keep the other bits.
        *this.add(0x5A4) |= 1;
        // Sentinel word, then five cleared words.
        *(this.add(0x4A4) as *mut u32) = 0xFFFF_FFFF;
        core::ptr::write_bytes(this.add(0x4A8), 0, 20);
        this as u32
    }
});
