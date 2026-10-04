// original: 0x00bc70d0 PLANE_STARTS_IN_AIR
/// Mark that a plane starts in the air: forward the script argument to
/// the engine. No return value.
export!(cdecl, rw_00bc70d0(ctx: *const u32) -> u32 {
    unsafe {
        let args = *ctx.add(2) as *const u32;
        callee_cdecl!(1, u32, *args);
        0
    }
});
