// original: 0x00513c50 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_216,player_schema::LeaderboardInfo,10>::LeaderboardInfo,10>
/// Constructor for one ranked episodic-race leaderboard description object.
///
/// Runs the shared base constructor on `this`, then installs this class's two
/// virtual tables, sets the ready flag, resets the selection index to "none",
/// clears the trailing counter block, and returns `this`.
export!(thiscall, rw_00513c50(this_ptr: u32) -> u32 {
    unsafe {
        /// Offset of the main virtual-table pointer from `this`.
        const MAIN_VTABLE_SLOT: u32 = 0x000;
        /// Offset of the inner virtual-table pointer from `this`.
        const INNER_VTABLE_SLOT: u32 = 0x4A0;
        /// Offset of the selection index ("none" when all bits set).
        const SELECTION_SLOT: u32 = 0x4A4;
        /// First word of the cleared counter block.
        const COUNTERS_SLOT: u32 = 0x4A8;
        /// How many words the counter block holds.
        const COUNTER_WORDS: u32 = 5;
        /// Offset of the ready-flag byte.
        const READY_FLAG: u32 = 0x5A4;

        // Base-class construction first; its effects are scripted by the
        // checker on both sides, so only the call itself is compared here.
        callee_thiscall!(1, u32, this_ptr);

        let this = this_ptr as *mut u32;
        // Install this class's virtual tables (file addresses, relocated).
        this.add((MAIN_VTABLE_SLOT / 4) as usize).write(relocated(0x00FCF640));
        this.add((INNER_VTABLE_SLOT / 4) as usize).write(relocated(0x00FDBE5C));
        // Mark ready, keeping any other flag bits already set.
        *((this_ptr + READY_FLAG) as *mut u8) |= 1;
        // No selection yet.
        this.add((SELECTION_SLOT / 4) as usize).write(0xFFFF_FFFF);
        // Clear the counter block.
        for i in 0..COUNTER_WORDS {
            this.add(((COUNTERS_SLOT / 4) + i) as usize).write(0);
        }
        this_ptr
    }
});
