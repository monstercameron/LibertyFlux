// original: 0x00b8d990 SET_TEXT_JUSTIFY
/// Native handler `SET_TEXT_JUSTIFY`.
///
/// Sets whether drawn text is justified to fill an even rectangle.
///
/// Handler mechanics: takes the native call context,
/// Forwards the coerced flag to the engine.
/// The flag dword carries the original's argument-slot quirk (see body).
lf_rn21_rt::export!(cdecl, rw_00b8d990(ctx: u32) -> () {
    let args = unsafe { *((ctx.wrapping_add(8)) as *const u32) } as *const u32;
    let flag = u32::from(unsafe { *args } != 0);
    // Quirk, reproduced exactly: the original coerces the boolean in place in
    // its own argument slot and forwards the whole slot dword, so the high
    // bytes of the forwarded value are the low bytes of the context pointer.
    // The call comparison verifies the forwarded dword bit for bit.
    let coerced = (ctx & 0xFFFF_FF00) | flag;
    lf_rn21_rt::callee_cdecl!(1, u32, coerced);
});
