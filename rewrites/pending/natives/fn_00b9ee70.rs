// original: 0x00b9ee70 GET_CHAR_HEIGHT_ABOVE_GROUND
/// Script native handler `GET_CHAR_HEIGHT_ABOVE_GROUND`.
///
/// Returns the char's distance from the ground (via out-pointer).
///
/// Forwards 2 script arguments to the engine function; no return slot.
/// handler function: `0x00b9ee70`, engine call site: `0x00b9ee7c`.
export!(cdecl, rw_b9ee70(ctx: *const NativeCtx) -> u32 {
    unsafe {
        let args = (*ctx).args_ptr;
        let ped = *args.add(0);
        let height_out = *args.add(1);
        callee_cdecl!(1, u32, ped, height_out)
    }
});
