// original: 0x00bba990 TASK_SMART_FLEE_CHAR
/// Script native `TASK_SMART_FLEE_CHAR` (hash 0x1880639C).
///
/// Forwards four script arguments to the engine: two character
/// handles, one float bit-pattern (the safe distance) and one integer.
/// The float is copied as raw bits, so the forward is bit-exact. No
/// return slot is written.
export!(cdecl, rw_00bba990(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2), *args.add(3))
    }
});
