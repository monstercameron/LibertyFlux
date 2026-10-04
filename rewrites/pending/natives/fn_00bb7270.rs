// original: 0x00bb7270 ADD_NEEDED_AT_POSN
/// Script native `ADD_NEEDED_AT_POSN` (hash 0x2E831921).
///
/// Marks a map position as needed for streaming. Forwards three float
/// bit-patterns (the x, y, z coordinates) to the engine; they are copied as
/// raw bits, so the forward is bit-exact. No return slot is written.
export!(cdecl, rw_00bb7270(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2))
    }
});
