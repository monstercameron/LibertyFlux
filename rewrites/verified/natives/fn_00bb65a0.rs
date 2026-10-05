// original: 0x00bb65a0 REGISTER_TRACK_NUMBER
/// Script native `REGISTER_TRACK_NUMBER` (hash 0x519B3104).
///
/// Forwards one script argument (the track number) to the engine. No return
/// slot is written.
/// (The original cleans its one pushed argument with `(an instruction of the original)`; the effect
/// on the stack pointer is identical to the plain cdecl return here.)
export!(cdecl, rw_00bb65a0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
