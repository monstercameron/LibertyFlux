// original: 0x00bba510 TASK_SEEK_COVER_FROM_POS
/// Script native `TASK_SEEK_COVER_FROM_POS` (hash 0x2BDF7B7E).
///
/// Forwards five script arguments (character handle, three float coordinates as raw bits, flags) to the engine task routine. Writes no return slot.
export!(cdecl, rw_00bba510(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args.add(0), *args.add(1), *args.add(2), *args.add(3), *args.add(4))
    }
});
