// original: 0x00bba690 TASK_SHAKE_FIST
/// Script native `TASK_SHAKE_FIST` (hash 0x0F7F3837).
///
/// Ped handle.
/// No return slot is written.
///
/// The handler returns the engine's answer in EAX (the register
/// is untouched after the call); the rewrite does the same.
export!(cdecl, rw_00bba690(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
