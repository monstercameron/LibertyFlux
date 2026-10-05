// original: 0x00b8cfc0 PRINT_HELP_OVER_FRONTEND
/// Script native `PRINT_HELP_OVER_FRONTEND` (hash 0x1C334022).
///
/// Forwards one script argument (a text label) to the engine. No return slot
/// is written.
/// (The original cleans its one pushed argument with `(an instruction of the original)`; the effect
/// on the stack pointer is identical to the plain cdecl return here.)
export!(cdecl, rw_00b8cfc0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
