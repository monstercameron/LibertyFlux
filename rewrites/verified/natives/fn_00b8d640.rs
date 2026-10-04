// original: 0x00b8d640 SET_HELP_MESSAGE_BOX_SIZE_F
/// Script native `SET_HELP_MESSAGE_BOX_SIZE_F` (hash 0x7A521650).
///
/// Forwards one float script argument (copied as raw bits) to the engine. No return slot is written.
export!(cdecl, rw_00b8d640(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
