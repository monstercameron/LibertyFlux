// original: 0x00bba0c0 TASK_PERFORM_SEQUENCE_FROM_PROGRESS
/// Script native `TASK_PERFORM_SEQUENCE_FROM_PROGRESS` (hash 0x62701AF8).
///
/// Forwards four script arguments (a character handle, a sequence id and two progress values) to the engine. No return slot is written.
export!(cdecl, rw_00bba0c0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args.add(0), *args.add(1), *args.add(2), *args.add(3))
    }
});
