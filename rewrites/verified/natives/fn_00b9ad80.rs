// original: 0x00b9ad80 SET_IGNORE_NO_GPS_FLAG
/// Script native `SET_IGNORE_NO_GPS_FLAG` (hash 0x1FC06A1B).
///
/// Forwards one boolean script argument to the engine, coerced with
/// `arg != 0` through the stack-slot quirk (see
/// `ALLOW_THIS_SCRIPT_TO_BE_PAUSED`): the pushed word's high bytes repeat
/// the context pointer and are reproduced here for bit-exact matching. No
/// return slot is written.
export!(cdecl, rw_00b9ad80(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let flag = u32::from(*args != 0);
        let quirked = (ctx as u32 & 0xFFFF_FF00) | flag;
        callee_cdecl!(1, u32, quirked)
    }
});
