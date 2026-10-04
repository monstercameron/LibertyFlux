// original: 0x00bba000 TASK_LOOK_AT_VEHICLE
/// Script native `TASK_LOOK_AT_VEHICLE` (hash 0x4A2C5544).
///
/// Forwards four script arguments (a character handle, a vehicle handle and
/// task parameters) to the engine. No return slot is written.
export!(cdecl, rw_00bba000(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2), *args.add(3))
    }
});
