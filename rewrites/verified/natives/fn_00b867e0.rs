// original: 0x00b867e0 CAM_SEQUENCE_START
/// Script native `CAM_SEQUENCE_START` (hash 0x26335EE7).
///
/// Starts a camera sequence. Forwards one script argument (the sequence id)
/// to the engine. No return slot is written.
/// (The original cleans its one pushed argument with `(an instruction of the original)`; the effect
/// on the stack pointer is identical to the plain cdecl return here.)
export!(cdecl, rw_00b867e0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
