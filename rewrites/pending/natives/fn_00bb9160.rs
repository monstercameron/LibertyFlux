// original: 0x00bb9160 TASK_CAR_DRIVE_WANDER
/// Script native `TASK_CAR_DRIVE_WANDER` (hash 0x1E9635A9).
///
/// Forwards four script arguments to the engine: a character handle, an
/// integer, a float bit-pattern (speed), and an integer flag. The float is
/// copied as raw bits, so the forward is bit-exact. No return slot is
/// written.
export!(cdecl, rw_00bb9160(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2), *args.add(3))
    }
});
