// original: 0x00bb9b00 TASK_GO_TO_COORD_ANY_MEANS
/// Script native `TASK_GO_TO_COORD_ANY_MEANS` (hash 0x04F72E4C).
///
/// Forwards six script arguments to the engine: a character handle, three
/// float bit-patterns (target coordinates, moved through SSE registers),
/// and two integers. Floats are copied as raw bits, so the forward is
/// bit-exact. No return slot is written.
export!(cdecl, rw_00bb9b00(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(
            1,
            u32,
            *args,
            *args.add(1),
            *args.add(2),
            *args.add(3),
            *args.add(4),
            *args.add(5),
        )
    }
});
