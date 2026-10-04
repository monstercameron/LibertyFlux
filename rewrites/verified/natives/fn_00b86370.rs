// original: 0x00b86370 REGISTER_WORLD_POINT_SCRIPT_BRAIN
/// Native handler `REGISTER_WORLD_POINT_SCRIPT_BRAIN` (script context in, engine call out).
///
/// Registers a world-point script brain; forwards a handle and a radius float.
/// `ctx.ret` points at the return slot, `ctx.args` at the argument array.
export!(cdecl, rw_00b86370(ctx: *mut NativeCtx) -> u32 {
    unsafe {
        let a = (*ctx).args;
        callee_cdecl!(1, u32, *a.add(0), *a.add(1))
    }
});
