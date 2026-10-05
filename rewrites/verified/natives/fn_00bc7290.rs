// original: 0x00bc7290 SET_AMBIENT_PLANES_SPEED_MULTIPLIER
/// Script native `SET_AMBIENT_PLANES_SPEED_MULTIPLIER` (hash 0x4B470947).
///
/// Forwards one script argument (the speed multiplier as a float
/// bit-pattern) to the engine. The float is copied as raw bits, so the
/// forward is bit-exact. No return slot is written.
/// (The original cleans its one pushed argument with `(an instruction of the original)`; the effect
/// on the stack pointer is identical to the plain cdecl return here.)
export!(cdecl, rw_00bc7290(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
