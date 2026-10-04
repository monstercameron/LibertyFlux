// original: 0x00a01de0 SET_OBJECT_LIGHTS
/// Script native `SET_OBJECT_LIGHTS` (hash 0x45D71590).
///
/// Forwards an object handle and a boolean flag to the engine. The flag is
/// coerced with `arg != 0` through the stack-slot quirk (see
/// `ALLOW_THIS_SCRIPT_TO_BE_PAUSED`): the pushed word's high bytes repeat
/// the context pointer and are reproduced here for bit-exact matching. No
/// return slot is written.
export!(cdecl, rw_00a01de0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let flag = u32::from(*args.add(1) != 0);
        let quirked = (ctx as u32 & 0xFFFF_FF00) | flag;
        callee_cdecl!(1, u32, *args, quirked)
    }
});
