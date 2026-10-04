// original: 0x00bc7e20 SET_VEHICLE_DEFORMATION_MULT
/// Script native `SET_VEHICLE_DEFORMATION_MULT` (hash 0x7B65266B).
///
/// Forwards two script arguments to the engine: a vehicle handle and
/// one float bit-pattern (the deformation multiplier). The float is
/// copied as raw bits, so the forward is bit-exact. No return slot is
/// written.
export!(cdecl, rw_00bc7e20(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
