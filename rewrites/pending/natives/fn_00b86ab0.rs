// original: 0x00b86ab0 FORCE_GAME_TELESCOPE_CAM
/// Script native `FORCE_GAME_TELESCOPE_CAM` (hash 0x01C51E90).
///
/// Coerces script argument 0 to a 0/1 flag and forwards it to the engine telescope-camera switch.
/// No return slot is written.
/// The original pushes the flag byte through its own stack slot; the contract masks the call argument to the low byte.
export!(cdecl, rw_00b86ab0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let flag = u32::from(*args != 0);
        callee_cdecl!(1, u32, flag)
    }
});
