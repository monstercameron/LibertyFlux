// original: 0x00b93db0 ADD_STRING_TO_NEWS_SCROLLBAR

/// Native handler `ADD_STRING_TO_NEWS_SCROLLBAR`.
///
/// Pass the string argument to the news scrollbar engine routine.
/// Forwards 1 argument(s) to the engine and returns nothing.
export!(cdecl, rw_00b93db0(ctx: *const u32) -> u32 {
    unsafe {
        let args = *ctx.add(2) as *const u32;
        let arg0 = *args.add(0);
        let _ = arg0;
        callee_cdecl!(1, u32, arg0);
        0
    }
});
