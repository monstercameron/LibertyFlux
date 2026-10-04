// original: 0x00b8d390 REMOVE_BLIP
/// Script native `REMOVE_BLIP` (hash 0x7BBF3625).
///
/// Forwards one script argument (a blip handle) to the engine.
/// No return slot is written.
export!(cdecl, rw_00b8d390(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
