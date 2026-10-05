// original: 0x009cc280 REMOVE_CLOSE_MIC_PED
/// Script native `REMOVE_CLOSE_MIC_PED` (hash 0x72B73FBA).
///
/// Forwards one script argument (a character handle) to the engine.
/// No return slot is written.
/// (The original cleans its one pushed argument with `(an instruction of the original)`; the effect
/// on the stack pointer is identical to the plain cdecl return here.)
export!(cdecl, rw_009cc280(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
