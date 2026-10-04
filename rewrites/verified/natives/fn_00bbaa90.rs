// original: 0x00bbaa90 TASK_SPACE_SHIP_GO_TO_COORD
/// Script native `TASK_SPACE_SHIP_GO_TO_COORD` (hash 0x2960330A).
///
/// Forwards a character handle, a ship handle and three coordinate words
/// (copied bit-for-bit through vector registers, floating point in
/// meaning) to the engine. No return slot is written.
export!(cdecl, rw_00bbaa90(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2), *args.add(3), *args.add(4))
    }
});
