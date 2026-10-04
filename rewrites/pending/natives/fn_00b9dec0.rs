// original: 0x00b9dec0 ALLOW_AUTO_CONVERSATION_LOOKATS
/// Native handler `ALLOW_AUTO_CONVERSATION_LOOKATS`.
///
/// Lets a character automatically look at whoever is talking in a conversation.
///
/// Handler mechanics: takes the native call context,
/// Forwards the character handle and the coerced allow flag to the engine.
/// The flag dword carries the original's argument-slot quirk (see body).
lf_rn21_rt::export!(cdecl, rw_00b9dec0(ctx: u32) -> () {
    let args = unsafe { *((ctx.wrapping_add(8)) as *const u32) } as *const u32;
    let ped = unsafe { *args };
    let flag = u32::from(unsafe { *args.add(1) } != 0);
    // Quirk, reproduced exactly: the original coerces the boolean in place in
    // its own argument slot and forwards the whole slot dword, so the high
    // bytes of the forwarded value are the low bytes of the context pointer.
    // The call comparison verifies the forwarded dword bit for bit.
    let coerced = (ctx & 0xFFFF_FF00) | flag;
    lf_rn21_rt::callee_cdecl!(1, u32, ped, coerced);
});
