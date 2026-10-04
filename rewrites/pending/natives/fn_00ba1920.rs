// original: 0x00ba1920 SET_CHAR_PROP_INDEX
/// Native handler `SET_CHAR_PROP_INDEX`.
///
/// Set a character prop index.
/// Forwards 3 argument(s) to the engine and returns nothing.
export!(cdecl, rw_00ba1920(ctx: *const u32) -> u32 {
    unsafe {
        let args = *ctx.add(2) as *const u32;
        let arg0 = *args.add(0);
        let arg1 = *args.add(1);
        let arg2 = *args.add(2);
        let _ = arg0;
        callee_cdecl!(1, u32, arg0, arg1, arg2);
        0
    }
});
