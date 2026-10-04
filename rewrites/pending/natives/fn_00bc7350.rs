// original: 0x00bc7350 SET_CAR_ALWAYS_CREATE_SKIDS
/// Script native `SET_CAR_ALWAYS_CREATE_SKIDS` (hash 0x0B9F0356).
///
/// Forwards two script arguments (a vehicle handle and a boolean toggle) to the engine.
///
/// The toggle is coerced with `arg != 0`.
///
/// Quirk (observed): the handler coerces the toggle into the low byte of its own incoming stack slot and pushes the whole dword, so the pushed word's high bytes repeat the context pointer. The engine reads only the low byte (Inferred); the full dword is reproduced here for bit-exact outgoing-call matching. No return slot is written.
export!(cdecl, rw_00bc7350(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let flag = u32::from(*args.add(1) != 0);
        let quirked = (ctx as u32 & 0xFFFF_FF00) | flag;
        callee_cdecl!(1, u32, *args, quirked)
    }
});
