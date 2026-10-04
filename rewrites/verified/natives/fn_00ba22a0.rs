// original: 0x00ba22a0 SET_NM_MESSAGE_STRING
/// Script native `SET_NM_MESSAGE_STRING` (hash 0x3F296F78).
///
/// Forwards two script arguments (a message id and a string pointer) to
/// the engine. No return slot is written.
export!(cdecl, rw_00ba22a0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
