// original: 0x00b8d6c0 SET_MENU_COLUMN_ORIENTATION
/// Script native handler `SET_MENU_COLUMN_ORIENTATION`.
///
/// Forwards 3 script arguments to the engine function; no return slot.
/// handler function: `0x00b8d6c0`, engine call site: `0x00b8d6cf`.
export!(cdecl, rw_b8d6c0(ctx: *const NativeCtx03) -> u32 {
    unsafe {
        let args = (*ctx).args_ptr;
        let a0 = *args.add(0);
        let a1 = *args.add(1);
        let a2 = *args.add(2);
        callee_cdecl!(1, u32, a0, a1, a2)
    }
});
