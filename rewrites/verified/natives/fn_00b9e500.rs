// original: 0x00b9e500 CLEAR_CHAR_PROP
/// Script native `CLEAR_CHAR_PROP` (hash 0x51546112).
///
/// Forwards two script arguments (a character handle and a prop slot) to
/// the engine. No return slot is written.
export!(cdecl, rw_00b9e500(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
