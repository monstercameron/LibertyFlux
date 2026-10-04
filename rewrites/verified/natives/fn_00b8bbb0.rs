// original: 0x00b8bbb0 ADD_SIMPLE_BLIP_FOR_PICKUP
/// Script native `ADD_SIMPLE_BLIP_FOR_PICKUP` (hash 0x44B30452).
///
/// Forwards one script argument to the engine. No return slot is written.
export!(cdecl, rw_00b8bbb0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
