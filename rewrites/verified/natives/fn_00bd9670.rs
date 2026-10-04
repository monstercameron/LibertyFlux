// original: 0x00bd9670 SET_SYNC_WEATHER_AND_GAME_TIME
/// Script native `SET_SYNC_WEATHER_AND_GAME_TIME` (hash 0x51112E95).
///
/// Coerces script argument 0 to a 0/1 flag and forwards it to the engine weather/time sync switch.
/// No return slot is written.
/// The original pushes the flag byte through its own stack slot; the contract masks the call argument to the low byte.
export!(cdecl, rw_00bd9670(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let flag = u32::from(*args != 0);
        callee_cdecl!(1, u32, flag)
    }
});
