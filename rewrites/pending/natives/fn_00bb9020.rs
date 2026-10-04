// original: 0x00bb9020 TASK_ACHIEVE_HEADING
/// Script native `TASK_ACHIEVE_HEADING` (hash 0x6D6A1261).
///
/// Forwards two script arguments (a character handle and a float bit-pattern for a heading) to the engine. No return slot is written.
export!(cdecl, rw_00bb9020(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
