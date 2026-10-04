// original: 0x00b8ce50 LOAD_ADDITIONAL_TEXT

/// Native handler `LOAD_ADDITIONAL_TEXT`.
///
/// Load an additional text table by name and slot.
/// Forwards 2 argument(s) to the engine and returns nothing.
export!(cdecl, rw_00b8ce50(ctx: *const u32) -> u32 {
    unsafe {
        let args = *ctx.add(2) as *const u32;
        let arg0 = *args.add(0);
        let arg1 = *args.add(1);
        let _ = arg0;
        callee_cdecl!(1, u32, arg0, arg1);
        0
    }
});
