// original: 0x0050bc70 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Race18Standard,player_schema::LeaderboardInfo,10>::LeaderboardInfo,10>
/// Constructor for the `Race18Standard` ranked-leaderboard info object.
///
/// Runs the shared base constructor, then installs this board's primary and
/// secondary tables, sets the initialised flag, resets the state word to -1
/// and clears the five trailing info words. Returns the object pointer.
export!(thiscall, rw_0050bc70(this: u32) -> u32 {
    unsafe {
        // File VAs of this board's tables (relocated at load; both have
        // HIGHLOW fixups in the executable).
        const PRIMARY_TABLE: u32 = 0x00fcfac4;
        const SECONDARY_TABLE: u32 = 0x00fd802c;
        // Field offsets within the object (base-class layout undocumented).
        const SECONDARY_SLOT: u32 = 0x4a0;
        const STATE_WORD: u32 = 0x4a4;
        const INFO_WORDS: u32 = 0x4a8;
        const INFO_COUNT: u32 = 5;
        const FLAGS_BYTE: u32 = 0x5a4;
        callee_thiscall!(1, u32, this);
        let obj = this as *mut u8;
        (obj as *mut u32).write(relocated(PRIMARY_TABLE));
        (obj.add(SECONDARY_SLOT as usize) as *mut u32).write(relocated(SECONDARY_TABLE));
        let flags = obj.add(FLAGS_BYTE as usize).read();
        obj.add(FLAGS_BYTE as usize).write(flags | 1);
        (obj.add(STATE_WORD as usize) as *mut u32).write(0xffff_ffff);
        let mut slot = obj.add(INFO_WORDS as usize) as *mut u32;
        let mut left = INFO_COUNT;
        while left > 0 {
            slot.write(0);
            slot = slot.add(1);
            left -= 1;
        }
        this
    }
});
