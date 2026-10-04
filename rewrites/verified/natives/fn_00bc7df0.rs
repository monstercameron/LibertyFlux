// original: 0x00bc7df0 SET_VEHICLE_ALWAYS_RENDER
/// Native handler `SET_VEHICLE_ALWAYS_RENDER` (script context in, engine call out).
///
/// Marks a vehicle always rendered; forwards the vehicle handle.
/// `ctx.ret` points at the return slot, `ctx.args` at the argument array.
export!(cdecl, rw_00bc7df0(ctx: *mut NativeCtx) -> u32 {
    unsafe {
        let a = (*ctx).args;
        callee_cdecl!(1, u32, *a.add(0))
    }
});
