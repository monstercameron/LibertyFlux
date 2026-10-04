// original: 0x00bb2a10 SET_PLAYER_CAN_USE_COVER
/// Native handler `SET_PLAYER_CAN_USE_COVER` (script context in, engine call out).
///
/// Toggles player cover use; forwards player index and coerced flag.
/// `ctx.ret` points at the return slot, `ctx.args` at the argument array.
export!(cdecl, rw_00bb2a10(ctx: *mut NativeCtx) -> u32 {
    unsafe {
        let a = (*ctx).args;
        callee_cdecl!(1, u32, *a.add(0), u32::from(*a.add(1) != 0))
    }
});
