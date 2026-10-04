// original: 0x00bbae30 TASK_WANDER_STANDARD
/// Script native `TASK_WANDER_STANDARD` (hash 0x43F5151F).
///
/// Forwards one script argument to the engine.
/// No return slot is written.
export!(cdecl, rw_00bbae30(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
