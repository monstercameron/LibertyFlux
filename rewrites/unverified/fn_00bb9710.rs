// original: 0x00bb9710 TASK_EVERYONE_LEAVE_CAR
/// Script native `TASK_EVERYONE_LEAVE_CAR` (hash 0x41E45BE5).
///
/// Forwards one script argument (a vehicle handle) to the engine.
/// No return slot is written.
export!(cdecl, rw_00bb9710(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
