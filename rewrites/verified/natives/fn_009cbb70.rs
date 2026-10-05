// original: 0x009cbb70 FORCE_FULL_VOICE
/// Script native `FORCE_FULL_VOICE` (hash 0x62285CAD).
///
/// Forwards one script argument (a boolean-ish flag) to the engine. No
/// return slot is written.
/// (The original cleans its one pushed argument with `(an instruction of the original)`; the effect
/// on the stack pointer is identical to the plain cdecl return here.)
export!(cdecl, rw_009cbb70(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
