// original: 0x00bbacf0 TASK_TURN_CHAR_TO_FACE_COORD
/// Script native `TASK_TURN_CHAR_TO_FACE_COORD` (hash 0x51517B11).
///
/// Forwards four script arguments to the engine: a character handle and
/// three float bit-patterns (the coordinates to face). Floats are copied
/// as raw bits, so the forward is bit-exact. No return slot is written.
export!(cdecl, rw_00bbacf0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2), *args.add(3))
    }
});
