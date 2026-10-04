// original: 0x00ba1500 SET_CHAR_FIRE_DAMAGE_MULTIPLIER
/// Native handler `SET_CHAR_FIRE_DAMAGE_MULTIPLIER`: forwards script args [arg0 (dword), arg1 (float bits)]
/// to its engine function and returns nothing (void).
export!(cdecl, rw_00ba1500(ctx: *const u32) -> u32 {
    unsafe {
        let args = *(ctx.add(2) as *const *const u32);
        callee_cdecl!(1, u32, *args.add(0), *args.add(1));
        0
    }
});
