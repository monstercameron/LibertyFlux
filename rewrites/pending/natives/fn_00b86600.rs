// original: 0x00b86600 ACTIVATE_VIEWPORT
/// Script native `ACTIVATE_VIEWPORT` (hash 0x4D7D105A).
///
/// Handle + bool flag.
///
/// Quirk (observed): the handler coerces the boolean argument
/// into the low byte of its own incoming stack slot and pushes
/// the whole dword, so the pushed word's high bytes repeat the
/// context pointer. The engine reads only the low byte (Inferred);
/// the full dword is reproduced here for bit-exact outgoing-call
/// matching.
/// No return slot is written.
///
/// The handler returns the engine's answer in EAX (the register
/// is untouched after the call); the rewrite does the same.
export!(cdecl, rw_00b86600(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let flag = u32::from(*args.add(1) != 0);
        let quirked = (ctx as u32 & 0xFFFF_FF00) | flag;
        callee_cdecl!(1, u32, *args, quirked)
    }
});
