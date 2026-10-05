// original: 0x009cbd50 HIGH_FALL_SCREAM
/// Script native `HIGH_FALL_SCREAM` (hash 0x478976DB).
///
/// Forwards one script argument (a character handle) to the engine. No return slot is written. (The original cleans its one pushed argument with `(an instruction of the original)`; the effect on the stack pointer is identical to the plain cdecl return here.)
export!(cdecl, rw_009cbd50(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
