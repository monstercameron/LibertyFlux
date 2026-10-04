// original: 0x00b9e380 BLOCK_CHAR_HEAD_IK
/// Script native `BLOCK_CHAR_HEAD_IK` (hash 0x3EFA66E8).
///
/// Forwards two script arguments (a character handle and a boolean toggle) to the engine.
///
/// The toggle is coerced with `arg != 0`.
///
/// Quirk (observed): the handler coerces the toggle into the low byte of its own incoming stack slot and pushes the whole dword, so the pushed word's high bytes repeat the context pointer. The engine reads only the low byte (Inferred); the full dword is reproduced here for bit-exact outgoing-call matching. No return slot is written.
export!(cdecl, rw_00b9e380(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let flag = u32::from(*args.add(1) != 0);
        let quirked = (ctx as u32 & 0xFFFF_FF00) | flag;
        callee_cdecl!(1, u32, *args, quirked)
    }
});
