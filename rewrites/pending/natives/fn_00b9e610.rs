// original: 0x00b9e610 COPY_COMBAT_DECISION_MAKER

/// Native handler `COPY_COMBAT_DECISION_MAKER`.
///
/// Copy a combat decision maker into another slot.
/// Forwards 2 argument(s) to the engine and returns nothing.
export!(cdecl, rw_00b9e610(ctx: *const u32) -> u32 {
    unsafe {
        let args = *ctx.add(2) as *const u32;
        let arg0 = *args.add(0);
        let arg1 = *args.add(1);
        let _ = arg0;
        callee_cdecl!(1, u32, arg0, arg1);
        0
    }
});
