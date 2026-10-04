// original: 0x0050da10 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Standard_TeamVip,player_schema::LeaderboardInfo,10>::LeaderboardInfo,10>
/// Construct a `TeamVip` leaderboard-info object (thiscall constructor).
///
/// Runs the shared base-class constructor, installs this class's primary and
/// secondary vtable pointers, sets the initialised flag, marks the entry
/// index empty and zeroes the remaining header words. Returns `this`.
export!(thiscall, rw_0050da10(this: *mut u8) -> u32 {
    unsafe {
        // Field offsets within the leaderboard-info object.
        const SECONDARY_VTABLE_OFF: usize = 0x4a0;
        const INDEX_OFF: usize = 0x4a4;
        const ZEROED_FIRST_OFF: usize = 0x4a8;
        const ZEROED_WORDS: usize = 5;
        const FLAG_OFF: usize = 0x5a4;
        // Entry-index value meaning "no entry assigned".
        const INDEX_EMPTY: u32 = 0xffff_ffff;

        // Shared base-class constructor (intercepted by the checker).
        callee_thiscall!(1, u32, this as u32);

        let words = this as *mut u32;
        // Primary vtable pointer.
        *words = relocated(0xfd25b0);
        // Secondary vtable pointer.
        *words.add(SECONDARY_VTABLE_OFF / 4) = relocated(0xfdd7c4);
        // Initialised flag.
        *this.add(FLAG_OFF) |= 1;
        // Entry index: empty.
        *words.add(INDEX_OFF / 4) = INDEX_EMPTY;
        // Remaining header words start zeroed.
        for i in 0..ZEROED_WORDS {
            *words.add(ZEROED_FIRST_OFF / 4 + i) = 0;
        }
        this as u32
    }
});
