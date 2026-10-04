// original: 0x00bb96f0 TASK_ENTER_CAR_AS_PASSENGER
/// Script native `TASK_ENTER_CAR_AS_PASSENGER` (hash 0x0A2C70AF).
///
/// Forwards four script arguments (a character handle, a vehicle handle, a seat index and flags) to the engine task assigner. No return slot is written.
export!(cdecl, rw_00bb96f0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2), *args.add(3))
    }
});
