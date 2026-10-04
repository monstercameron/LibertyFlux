// original: 0x0050BF70 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Race26Standard,player_schema::LeaderboardInfo,10>::LeaderboardInfo,10>
/// Network-subsystem object constructor 05 of 20 siblings.
///
/// Runs the shared base-class constructor (thiscall/0, intercepted by the
/// checker), installs this class's two virtual tables, sets the initialised
/// flag bit, resets the state word to -1 and clears five trailing words.
/// Returns the object pointer it was given.
export!(thiscall, rw_0050bf70(this_ptr: u32) -> u32 {
    unsafe {
        const VTABLE: u32 = 0x00FD56CC;
        const SECONDARY_VTABLE: u32 = 0x00FD8AC4;
        const SECONDARY_OFF: u32 = 0x4a0;
        const STATE_OFF: u32 = 0x4a4;
        const FLAG_OFF: u32 = 0x5a4;
        const ZERO_WORDS: [u32; 5] = [0x4a8, 0x4ac, 0x4b0, 0x4b4, 0x4b8];
        let _base_answer: u32 = callee_thiscall!(1, u32, this_ptr);
        (this_ptr as *mut u32).write(relocated(VTABLE));
        (this_ptr.wrapping_add(SECONDARY_OFF) as *mut u32).write(relocated(SECONDARY_VTABLE));
        let flag = this_ptr.wrapping_add(FLAG_OFF) as *mut u8;
        flag.write(flag.read() | 1);
        (this_ptr.wrapping_add(STATE_OFF) as *mut u32).write(u32::MAX);
        for off in ZERO_WORDS {
            (this_ptr.wrapping_add(off) as *mut u32).write(0);
        }
        this_ptr
    }
});
