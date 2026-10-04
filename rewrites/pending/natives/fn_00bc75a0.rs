// original: 0x00bc75a0 SET_CAR_GENERATORS_ACTIVE_IN_AREA
/// Native handler `SET_CAR_GENERATORS_ACTIVE_IN_AREA` (script context in, engine call out).
///
/// Toggles car generators in a box; forwards six floats and a coerced flag.
/// `ctx.ret` points at the return slot, `ctx.args` at the argument array.
export!(cdecl, rw_00bc75a0(ctx: *mut NativeCtx) -> u32 {
    unsafe {
        let a = (*ctx).args;
        callee_cdecl!(1, u32, *a.add(0), *a.add(1), *a.add(2), *a.add(3), *a.add(4), *a.add(5), coerced(ctx, *a.add(6)))
    }
});
