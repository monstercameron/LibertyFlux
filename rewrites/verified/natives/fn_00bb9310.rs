// original: 0x00bb9310 TASK_CAR_TEMP_ACTION
/// Script native `TASK_CAR_TEMP_ACTION` (hash 0x11612815).
///
/// Forwards four script arguments (a character handle, a vehicle handle and
/// two integers) to the engine. No return slot is written.
export!(cdecl, rw_00bb9310(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2), *args.add(3))
    }
});
