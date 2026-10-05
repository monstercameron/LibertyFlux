// original: 0x00bc7bf0 SET_RANDOM_CAR_DENSITY_MULTIPLIER
/// Script native `SET_RANDOM_CAR_DENSITY_MULTIPLIER` (hash 0x073505E0).
///
/// Forwards one float script argument (the density multiplier) to the
/// engine as raw bits, so the forward is bit-exact. No return slot is
/// written.
/// (The original cleans its one pushed argument with `(an instruction of the original)`; the effect
/// on the stack pointer is identical to the plain cdecl return here.)
export!(cdecl, rw_00bc7bf0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
