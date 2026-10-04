// original: 0x00bbad50 TASK_USE_MOBILE_PHONE_TIMED
/// Script native `TASK_USE_MOBILE_PHONE_TIMED` (hash 0x0BAD1A62).
///
/// Forwards two script arguments (a character handle and an integer
/// duration) to the engine. No return slot is written.
export!(cdecl, rw_00bbad50(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
