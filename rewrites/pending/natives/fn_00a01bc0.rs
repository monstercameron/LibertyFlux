// original: 0x00a01bc0 SET_OBJECT_ANIM_PLAYING_FLAG
/// Script native `SET_OBJECT_ANIM_PLAYING_FLAG` (hash 0x6A7236C9).
///
/// Object + anim names + bool flag.
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
export!(cdecl, rw_00a01bc0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let flag = u32::from(*args.add(3) != 0);
        let quirked = (ctx as u32 & 0xFFFF_FF00) | flag;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2), quirked)
    }
});
