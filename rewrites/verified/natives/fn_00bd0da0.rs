// original: 0x00bd0da0 ENABLE_MAX_AMMO_CAP
/// Script native `ENABLE_MAX_AMMO_CAP` (hash 0x7E657B56).
///
/// Coerces script argument 0 to a 0/1 flag and forwards it to the engine ammo-cap switch.
/// No return slot is written.
/// The original pushes the flag byte through its own stack slot, leaving caller-address bytes in the upper dword; the contract masks the call argument to the low byte.
export!(cdecl, rw_00bd0da0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let flag = u32::from(*args != 0);
        callee_cdecl!(1, u32, flag)
    }
});
