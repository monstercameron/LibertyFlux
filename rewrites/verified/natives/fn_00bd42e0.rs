// original: 0x00bd42e0 SET_PED_FIRE_FX_LOD_SCALER
/// Script native `SET_PED_FIRE_FX_LOD_SCALER` (hash 0x679C4276).
///
/// Forwards 1 script argument(s) to the engine: 1 float bit-pattern(s).
/// Floats are copied as raw bits, so the forward is bit-exact.
/// No return slot is written.
export!(cdecl, rw_00bd42e0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
