// original: 0x00b8d630 SET_HELP_MESSAGE_BOX_SIZE
/// Script native `SET_HELP_MESSAGE_BOX_SIZE` (hash 0x4FB069ED).
///
/// Forwards one script argument to the engine. No return slot is written.
export!(cdecl, rw_00b8d630(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
