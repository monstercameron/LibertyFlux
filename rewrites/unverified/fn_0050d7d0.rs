// original: 0x0050D7D0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Standard_CoopDrugFactory_BG_TIME,player_schema::LeaderboardInfo,10>::LeaderboardInfo,10>

/// Constructor of the concrete leaderboard-info class for Leaderboard_Standard_CoopDrugFactory_BG_TIME.
///
/// Calls the shared base constructor (its return value is ignored),
/// installs this class's primary vtable at offset 0, then initialises
/// the trailing descriptor block: the secondary vtable at +0x4A0, a
/// -1 sentinel word at +0x4A4, five zeroed words at +0x4A8..+0x4B8,
/// and sets the initialised flag (bit 0) of the byte at +0x5A4,
/// leaving its other bits unchanged. Returns the object pointer.
///
/// thiscall: the object pointer arrives in ECX; no stack arguments.
lf_checker_rt::export!(thiscall, rw_0050d7d0(this: u32) -> u32 {
    const PRIMARY_VTABLE: u32 = 0x00FD4510;
    const DESCRIPTOR_VTABLE: u32 = 0x00FD14E4;
    const DESCRIPTOR_AT: u32 = 0x4A0;
    const SENTINEL_AT: u32 = 0x4A4;
    const ZERO_WORDS_AT: [u32; 5] = [0x4A8, 0x4AC, 0x4B0, 0x4B4, 0x4B8];
    const FLAG_BYTE_AT: u32 = 0x5A4;
    const BASE_CTOR: u32 = 1;

    #[inline(always)]
    unsafe fn write_u32(address: u32, value: u32) {
        unsafe { (address as *mut u32).write_unaligned(value) }
    }

    unsafe {
        let _: u32 = lf_checker_rt::callee_thiscall!(BASE_CTOR, u32, this);
        write_u32(this, lf_checker_rt::relocated(PRIMARY_VTABLE));
        write_u32(this.wrapping_add(DESCRIPTOR_AT), lf_checker_rt::relocated(DESCRIPTOR_VTABLE));
        let flag = (this.wrapping_add(FLAG_BYTE_AT)) as *mut u8;
        flag.write_unaligned(flag.read_unaligned() | 1);
        write_u32(this.wrapping_add(SENTINEL_AT), 0xFFFF_FFFF);
        for at in ZERO_WORDS_AT {
            write_u32(this.wrapping_add(at), 0);
        }
        this
    }
});
