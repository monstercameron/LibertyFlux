// original: 0x0050c570 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Race42Standard,player_schema::LeaderboardInfo,10>::LeaderboardInfo,10>
/// Constructor for `rage::rlConcreteLeaderboardInfo<Leaderboard_Ranked_Race42Standard, ...>`.
///
/// Calls the shared base constructor, then installs this instantiation's
/// primary vtable at +0x0 and its member subobject's vtable at +0x4A0, sets
/// the initialised flag bit at +0x5A4, writes the member state sentinel
/// (0xFFFFFFFF) at +0x4A4 and zeroes the member state words at +0x4A8..+0x4BC.
/// Returns `this`.
export!(thiscall, rw_0050c570(this: *mut u8) -> u32 {
    unsafe {
        const PRIMARY_VTBL: u32 = 0x00fd09dc;
        const MEMBER_VTBL: u32 = 0x00fd2a0c;
        const MEMBER_OFF: usize = 0x4a0;
        const FLAG_OFF: usize = 0x5a4;
        callee_thiscall!(1, u32, this as u32);
        *(this as *mut u32) = relocated(PRIMARY_VTBL);
        *((this.add(MEMBER_OFF)) as *mut u32) = relocated(MEMBER_VTBL);
        *this.add(FLAG_OFF) |= 1;
        *((this.add(MEMBER_OFF + 4)) as *mut u32) = 0xffff_ffff;
        *((this.add(MEMBER_OFF + 8)) as *mut u32) = 0;
        *((this.add(MEMBER_OFF + 12)) as *mut u32) = 0;
        *((this.add(MEMBER_OFF + 16)) as *mut u32) = 0;
        *((this.add(MEMBER_OFF + 20)) as *mut u32) = 0;
        *((this.add(MEMBER_OFF + 24)) as *mut u32) = 0;
        this as u32
    }
});
