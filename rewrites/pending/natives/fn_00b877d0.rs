// original: 0x00b877d0 SET_CAR_FOV_RATE
/// Script native `SET_CAR_FOV_RATE` (hash 0x536B4F4A).
///
/// Forwards one script argument (a float bit-pattern, the field-of-view
/// rate) to the engine. Copied as raw bits, so the forward is bit-exact.
/// No return slot is written.
export!(cdecl, rw_00b877d0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
