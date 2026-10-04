// original: 0x00bb9ad0 TASK_GO_TO_CHAR
/// Script native `TASK_GO_TO_CHAR` (hash 0x664D06FF).
///
/// Forwards four script arguments to the engine: three integers (two
/// character handles and a time limit) and one float bit-pattern (a target
/// radius). The float is copied as raw bits, so the forward is bit-exact.
/// No return slot is written by the handler itself.
export!(cdecl, rw_00bb9ad0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2), *args.add(3))
    }
});
