// original: 0x00b9f1d0 GET_CURRENT_COP_MODEL
/// Script native `GET_CURRENT_COP_MODEL` (hash 0x018B2055).
///
/// Forwards one script argument to the engine. No return slot is written by
/// the handler itself.
/// (The original cleans its one pushed argument with `(an instruction of the original)`; the effect
/// on the stack pointer is identical to the plain cdecl return here.)
export!(cdecl, rw_00b9f1d0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
