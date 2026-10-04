// original: 0x00b9ac60 LOAD_PATH_NODES_IN_AREA
/// Script native `LOAD_PATH_NODES_IN_AREA` (hash 0x44640C28).
///
/// Forwards four float bit-patterns (two opposite corners of a rectangle)
/// to the engine. No return slot is written; the engine answer is the exit
/// value.
export!(cdecl, rw_00b9ac60(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2), *args.add(3))
    }
});
