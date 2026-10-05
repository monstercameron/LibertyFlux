// original: 0x00bd9630 SET_RICH_PRESENCE_TEMPLATESP2
/// Script native `SET_RICH_PRESENCE_TEMPLATESP2` (hash 0x09766174).
///
/// Forwards one script argument (a template string) to the engine. No return
/// slot is written.
/// (The original cleans its one pushed argument with `(an instruction of the original)`; the effect
/// on the stack pointer is identical to the plain cdecl return here.)
export!(cdecl, rw_00bd9630(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
