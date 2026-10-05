// original: 0x00b8cf30 PRINT_HELP_FOREVER
/// Script native `PRINT_HELP_FOREVER` (hash 0x43F7517D).
///
/// Forwards one script argument (a text label) to the engine text queue.
/// No return slot is written.
/// (The original cleans its one pushed argument with `(an instruction of the original)`; the effect
/// on the stack pointer is identical to the plain cdecl return here.)
export!(cdecl, rw_00b8cf30(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
