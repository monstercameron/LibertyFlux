// original: 0x00b877b0 SET_CAR_FOV_MIN
/// Script native `SET_CAR_FOV_MIN` (hash 0x068F59E3).
///
/// Forwards one script argument (a field-of-view value) to the engine as a
/// raw float bit-pattern, so the forward is bit-exact. No return slot is
/// written.
export!(cdecl, rw_00b877b0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
