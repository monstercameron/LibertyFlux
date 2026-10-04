// original: 0x00ba0ea0 SET_CHAR_ACCURACY
/// Script native `SET_CHAR_ACCURACY` (hash 0x1958471A).
///
/// Forwards two script arguments (a character handle and an accuracy
/// value) to the engine. No return slot is written.
export!(cdecl, rw_00ba0ea0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
