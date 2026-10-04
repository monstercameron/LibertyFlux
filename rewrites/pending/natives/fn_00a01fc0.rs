// original: 0x00a01fc0 SET_PICKUP_COLLECTABLE_BY_CAR
/// Set whether a pickup is collectable by car: pass the pickup handle
/// plus the second script argument coerced to 0/1 (low byte of a
/// context-pointer-based dword, as in the original). No return value.
export!(cdecl, rw_00a01fc0(ctx: *const u32) -> u32 {
    unsafe {
        let args = *ctx.add(2) as *const u32;
        let flag = u32::from(*args.add(1) != 0);
        callee_cdecl!(1, u32, *args, (ctx as u32 & 0xFFFFFF00) | flag);
        0
    }
});
