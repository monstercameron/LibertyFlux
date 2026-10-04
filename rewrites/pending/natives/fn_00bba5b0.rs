// original: 0x00bba5b0 TASK_SEEK_COVER_TO_COVER_POINT
/// Script native `TASK_SEEK_COVER_TO_COVER_POINT` (hash 0x143358D3).
///
/// Forwards six script words (handles and three coordinate float bit
/// patterns) to the engine. Floats are copied as raw bits, so the forward
/// is bit-exact. No return slot is written.
export!(cdecl, rw_00bba5b0(ctx: *const u8) -> u32 {
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
            *args.add(5)
        )
    }
});
