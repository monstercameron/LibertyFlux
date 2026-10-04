// original: 0x0050fd50 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_48,player_schema::LeaderboardInfo,10>::LeaderboardInfo,10>
/// Constructor for the network object with primary vtable `0x00FDF0EC`.
///
/// Calls the shared base constructor on `this`, installs this class's two
/// vtables (offsets 0 and 0x4A0), sets the initialized flag bit, resets the
/// generation counter to -1 and clears the trailing state words.
/// Returns `this`.
export!(thiscall, rw_0050fd50(this_ptr: u32) -> u32 {
    callee_thiscall!(1, u32, this_ptr);
    unsafe {
        let base = this_ptr as *mut u32;
        // Primary vtable then secondary vtable (both relocated image addresses).
        base.write(relocated(0x00FDF0EC));
        base.byte_add(0x4A0).cast::<u32>().write(relocated(0x00FD8B54));
        // Initialized flag bit.
        let flag = (this_ptr as *mut u8).byte_add(0x5A4);
        flag.write(flag.read() | 1);
        // Generation counter reset, then five cleared state words.
        base.byte_add(0x4A4).cast::<u32>().write(0xFFFF_FFFF);
        base.byte_add(0x4A8).cast::<u32>().write(0);
        base.byte_add(0x4AC).cast::<u32>().write(0);
        base.byte_add(0x4B0).cast::<u32>().write(0);
        base.byte_add(0x4B4).cast::<u32>().write(0);
        base.byte_add(0x4B8).cast::<u32>().write(0);
    }
    this_ptr
});
