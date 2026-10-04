// original: 0x00bc7af0 SET_PARKED_CAR_DENSITY_MULTIPLIER
/// Script native `SET_PARKED_CAR_DENSITY_MULTIPLIER` (hash 0x010C7044).
///
/// Float multiplier bits.
///
/// Float script arguments are forwarded as raw bit patterns,
/// so the forward is bit-exact.
/// No return slot is written.
///
/// The handler returns the engine's answer in EAX (the register
/// is untouched after the call); the rewrite does the same.
export!(cdecl, rw_00bc7af0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
