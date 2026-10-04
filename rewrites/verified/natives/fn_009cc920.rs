// original: 0x009cc920 TRIGGER_VIGILANTE_CRIME
/// Script native `TRIGGER_VIGILANTE_CRIME` (hash 0x195D582E).
///
/// Forwards four script arguments to the engine: one integer (a crime id)
/// and three float bit-patterns (a position). Floats are copied as raw bits,
/// so the forward is bit-exact. No return slot is written.
export!(cdecl, rw_009cc920(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2), *args.add(3))
    }
});
