// original: 0x00bd90f0 SAVE_SCRIPT_ARRAY_IN_SCRATCHPAD
/// Script native `SAVE_SCRIPT_ARRAY_IN_SCRATCHPAD` (hash 0x331F7E6F).
///
/// Forwards four script arguments (array bounds and scratchpad slot) to the
/// engine. No return slot is written.
export!(cdecl, rw_00bd90f0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(
            1,
            u32,
            *args,
            *args.add(1),
            *args.add(2),
            *args.add(3),
        )
    }
});
