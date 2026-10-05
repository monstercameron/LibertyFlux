// original: 0x00a018d0 RELEASE_ENTITY_FROM_ROPE_FOR_OBJECT
/// Script native `RELEASE_ENTITY_FROM_ROPE_FOR_OBJECT` (hash 0x7A575AC9).
///
/// Forwards one script argument (an object handle) to the engine. No return
/// slot is written.
/// (The original cleans its one pushed argument with `(an instruction of the original)`; the effect
/// on the stack pointer is identical to the plain cdecl return here.)
export!(cdecl, rw_00a018d0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
