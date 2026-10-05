// original: 0x00bd8fd0 REGISTER_MODEL_FOR_RANK_POINTS
/// Script native `REGISTER_MODEL_FOR_RANK_POINTS` (hash 0x39E61536).
///
/// Forwards one script argument (a model id) to the engine worker.
/// (The original drops its pushed word with `(an instruction of the original)`; the effect on the
/// stack pointer is identical to the plain cdecl return here.)
/// No return slot is written.
export!(cdecl, rw_00bd8fd0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
