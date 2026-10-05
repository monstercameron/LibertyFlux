// original: 0x00b94fc0 SET_RANDOM_SEED
/// Script native `SET_RANDOM_SEED` (hash 0x1BA8350B).
///
/// Forwards one script argument (the seed) to the engine. No return slot
/// is written.
/// (The original cleans its one pushed argument with `(an instruction of the original)`; the effect
/// on the stack pointer is identical to the plain cdecl return here.)
export!(cdecl, rw_00b94fc0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
