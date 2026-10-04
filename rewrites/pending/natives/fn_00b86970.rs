// original: 0x00b86970 DESTROY_ALL_SCRIPT_VIEWPORTS
/// Native handler `DESTROY_ALL_SCRIPT_VIEWPORTS` (script context in, engine call out).
///
/// Destroys all script viewports: current thread id plus constant kind through the cam manager.
/// `ctx.ret` points at the return slot, `ctx.args` at the argument array.
export!(cdecl, rw_00b86970(ctx: *mut NativeCtx) -> u32 {
    let _ = ctx;
    unsafe {
        let mgr = *global::<u32>(0x12BD0C4);
        let thread = callee_cdecl!(1, u32,);
        callee_thiscall!(2, u32, mgr, 0x16, thread)
    }
});
