// original: 0x00bb7500 REMOVE_ANIMS
/// Script native `REMOVE_ANIMS` (hash 0x55E00E7E).
///
/// Forwards one script argument (an animation-set name) to the engine.
/// (The original cleans its one pushed argument with `(an instruction of the original)`; the effect
/// on the stack pointer is identical to the plain cdecl return here.)
export!(cdecl, rw_00bb7500(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
