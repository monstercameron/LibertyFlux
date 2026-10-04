// original: 0x00bd9350 SET_NETWORK_ID_STOP_CLONING_FOR_ENEMIES
/// Stop cloning a network id for enemies: pass the id plus the second
/// script argument coerced to 0/1 (low byte of a context-pointer-based
/// dword, as in the original). No return value.
export!(cdecl, rw_00bd9350(ctx: *const u32) -> u32 {
    unsafe {
        let args = *ctx.add(2) as *const u32;
        let flag = u32::from(*args.add(1) != 0);
        callee_cdecl!(1, u32, *args, (ctx as u32 & 0xFFFFFF00) | flag);
        0
    }
});
