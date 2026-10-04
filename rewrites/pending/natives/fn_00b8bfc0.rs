// original: 0x00b8bfc0 DISABLE_PAUSE_MENU
/// Native handler `DISABLE_PAUSE_MENU` (script context in, engine call out).
///
/// Enables or disables the pause menu; coerces the flag to 0/1 in the incoming stack slot.
/// `ctx.ret` points at the return slot, `ctx.args` at the argument array.
export!(cdecl, rw_00b8bfc0(ctx: *mut NativeCtx) -> u32 {
    unsafe {
        let a = (*ctx).args;
        callee_cdecl!(1, u32, u32::from(*a.add(0) != 0))
    }
});
