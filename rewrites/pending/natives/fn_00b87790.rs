// original: 0x00b87790 SET_CAR_FOV_MAX
/// Script native `SET_CAR_FOV_MAX` (hash 0x3FBF13BD).
///
/// Forwards one float script argument to the engine as raw bits, so the forward is bit-exact.
export!(cdecl, rw_00b87790(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
