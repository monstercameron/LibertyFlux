// original: 0x00b87e90 UNATTACH_CAM
/// Native handler `UNATTACH_CAM` (script context in, engine call out).
///
/// Detaches an attached camera; forwards the camera handle.
/// `ctx.ret` points at the return slot, `ctx.args` at the argument array.
export!(cdecl, rw_00b87e90(ctx: *mut NativeCtx) -> u32 {
    unsafe {
        let a = (*ctx).args;
        callee_cdecl!(1, u32, *a.add(0))
    }
});
