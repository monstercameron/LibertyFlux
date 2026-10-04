// original: 0x00b982a0 GET_KEYBOARD_MOVE_INPUT
/// Script native `GET_KEYBOARD_MOVE_INPUT` (hash 0x4AF73456).
///
/// Forwards two script arguments (out-pointers for the movement axes) to the
/// engine. No return slot is written by the handler itself.
export!(cdecl, rw_00b982a0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
