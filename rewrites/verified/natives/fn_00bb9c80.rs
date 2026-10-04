// original: 0x00bb9c80 TASK_GUARD_ASSIGNED_DEFENSIVE_AREA
/// Script native `TASK_GUARD_ASSIGNED_DEFENSIVE_AREA` (hash 0x07E21C28).
///
/// Forwards seven script arguments to the engine: a character handle, five float bit-patterns (a position, a heading and a radius) and an integer. The original shuffles the words through vector registers while building the call frame, but the net effect is a plain in-order forward; floats are copied as raw bits, so the forward is bit-exact.
/// No return slot is written.
export!(cdecl, rw_00bb9c80(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2), *args.add(3), *args.add(4), *args.add(5), *args.add(6),)
    }
});
