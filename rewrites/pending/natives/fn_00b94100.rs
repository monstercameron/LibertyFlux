// original: 0x00b94100 CLEAR_AREA_OF_OBJECTS
/// Script native `CLEAR_AREA_OF_OBJECTS` (hash 0x118A67C9).
///
/// Forwards four float bit-patterns (an area origin) to the engine. No return slot is written.
export!(cdecl, rw_00b94100(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2), *args.add(3))
    }
});
