// original: 0x0050aa10 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Race29NoHolds,player_schema::LeaderboardInfo,10>::LeaderboardInfo,10>
/// Constructor for the ranked-race-29 leaderboard info object (network).
///
/// Runs the shared base-class constructor through the intercepted callee,
/// then installs this class's two vtables, sets the active flag bit, marks
/// the slot index empty (-1) and clears the five trailing info words.
/// Returns `this`.
export!(thiscall, rw_0050aa10(this: u32) -> u32 {
    unsafe {
        // Primary vtable for this leaderboard class (file VA; relocated at load).
        const VTABLE: u32 = 0x00FDBCB4;
        // Secondary vtable installed at +0x4a0.
        const VTABLE2: u32 = 0x00FD1264;
        callee_thiscall!(1, u32, this);
        let obj = this as *mut u8;
        *(obj as *mut u32) = relocated(VTABLE);
        *(obj.add(0x4a0) as *mut u32) = relocated(VTABLE2);
        *obj.add(0x5a4) |= 1;
        *(obj.add(0x4a4) as *mut u32) = 0xFFFF_FFFF;
        *(obj.add(0x4a8) as *mut u32) = 0;
        *(obj.add(0x4ac) as *mut u32) = 0;
        *(obj.add(0x4b0) as *mut u32) = 0;
        *(obj.add(0x4b4) as *mut u32) = 0;
        *(obj.add(0x4b8) as *mut u32) = 0;
        this
    }
});
