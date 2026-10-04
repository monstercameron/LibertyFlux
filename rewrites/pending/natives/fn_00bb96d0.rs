// original: 0x00bb96d0 TASK_ENTER_CAR_AS_DRIVER
/// Script native `TASK_ENTER_CAR_AS_DRIVER`.
///
/// Forwards three script arguments (a character handle, a vehicle handle
/// and a timeout) to the engine. No return slot is written.
export!(cdecl, rw_00bb96d0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2))
    }
});
