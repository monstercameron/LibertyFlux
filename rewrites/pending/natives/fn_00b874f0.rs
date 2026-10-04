// original: 0x00b874f0 SET_CAM_NEAR_DOF
/// Script native `SET_CAM_NEAR_DOF` (hash 0x60AD2FE0).
///
/// Forwards two script arguments to the engine: a camera handle and
/// /// one float bit-pattern (the near depth-of-field distance).
/// /// The float is copied as raw bits, so the forward is bit-exact.
/// /// No return slot is written.
export!(cdecl, rw_00b874f0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
