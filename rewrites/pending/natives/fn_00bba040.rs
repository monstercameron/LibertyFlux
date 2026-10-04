// original: 0x00bba040 TASK_OPEN_DRIVER_DOOR
/// Native handler `TASK_OPEN_DRIVER_DOOR`.
///
/// Makes a character open a vehicle's driver door.
///
/// Handler mechanics: takes the native call context,
/// Forwards character, vehicle and duration to the task engine.
lf_rn21_rt::export!(cdecl, rw_00bba040(ctx: u32) -> () {
    let args = unsafe { *((ctx.wrapping_add(8)) as *const u32) } as *const u32;
    let a0 = unsafe { *args };
    let a1 = unsafe { *args.add(1) };
    let a2 = unsafe { *args.add(2) };
    lf_rn21_rt::callee_cdecl!(1, u32, a0, a1, a2);
});
