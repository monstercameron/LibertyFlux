// original: 0x00b9a250 ADD_NAVMESH_REQUIRED_REGION
/// Native handler `ADD_NAVMESH_REQUIRED_REGION` (script context in, engine call out).
///
/// Adds navmesh coverage at a point; forwards 3 floats, stores callee's byte answer as dword.
/// `ctx.ret` points at the return slot, `ctx.args` at the argument array.
export!(cdecl, rw_00b9a250(ctx: *mut NativeCtx) -> u32 {
    unsafe {
        let a = (*ctx).args;
        let r = callee_cdecl!(1, u32, *a.add(0), *a.add(1), *a.add(2));
        // The original keeps only AL and zero-extends it into the slot.
        *(*ctx).ret = r & 0xFF;
        (*ctx).ret as u32
    }
});
