// original: 0x00bb1e20 GET_MAX_WANTED_LEVEL
/// Script native `GET_MAX_WANTED_LEVEL` (hash 0x71755E9B).
///
/// Forwards one script argument to the engine.
/// (The original cleans its one pushed argument with `(an instruction of the original)`; the effect
/// on the stack pointer is identical to the plain cdecl return here.)
/// No return slot is written.
export!(cdecl, rw_00bb1e20(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
