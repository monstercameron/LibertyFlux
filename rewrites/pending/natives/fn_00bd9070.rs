// original: 0x00bd9070 RESTORE_SCRIPT_ARRAY_FROM_SCRATCHPAD
/// Script native `RESTORE_SCRIPT_ARRAY_FROM_SCRATCHPAD` (hash 0x522B182B).
///
/// Forwards four script arguments to the engine. No return slot is written.
export!(cdecl, rw_00bd9070(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2), *args.add(3))
    }
});
