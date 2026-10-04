// original: 0x00b8bcc0 CHANGE_BLIP_DISPLAY
/// Script native `CHANGE_BLIP_DISPLAY` (hash 0x3ACC1794).
///
/// Forwards two script arguments (a blip handle and a display mode) to the engine.
///
/// No return slot is written.
export!(cdecl, rw_00b8bcc0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
