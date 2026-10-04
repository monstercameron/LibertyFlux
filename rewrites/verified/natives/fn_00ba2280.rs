// original: 0x00ba2280 SET_NM_MESSAGE_INT
/// Script native `SET_NM_MESSAGE_INT` (hash 0x49105005).
///
/// Forwards two script arguments (a message id and an integer value) to the engine. No return slot is written.
export!(cdecl, rw_00ba2280(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
