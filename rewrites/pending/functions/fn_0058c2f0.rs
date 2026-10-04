// original: 0x0058C2F0 rage::rlConcreteLeaderboard<player_schema::Leaderboard_Ranked_Episodic_Race_228,player_schema::LeaderboardInfo>::LeaderboardInfo>
/// Leaderboard info constructor: initialise the object at THIS.
///
/// Arguments: THIS in ECX (thiscall, no stack words). Zeroes offsets +0x04,
/// +0x08 and +0x0c, installs vtable 0x00FD29E4 at +0x00 and the info vtable
/// 0x00FDD154 at +0x10 (both relocated image addresses), and returns THIS.
/// Original: 0x0058C2F0 (thiscall, no stack words).

lf_checker_rt::export!(thiscall, rw_0058C2F0(this: u32) -> u32 {
    unsafe {
        const VTABLE: u32 = 0x00FD29E4;
        const INFO_VTABLE: u32 = 0x00FDD154;
        (this.wrapping_add(4) as *mut u32).write_unaligned(0);
        (this.wrapping_add(8) as *mut u32).write_unaligned(0);
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE));
        (this.wrapping_add(0x0c) as *mut u32).write_unaligned(0);
        (this.wrapping_add(0x10) as *mut u32).write_unaligned(lf_checker_rt::relocated(INFO_VTABLE));
        this
    }
});
