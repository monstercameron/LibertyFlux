// original: 0x00b9af20 SWITCH_ROADS_OFF
/// Native handler `SWITCH_ROADS_OFF` (script context in, engine call out).
///
/// Keeps cars off roads in a box; forwards six floats.
/// `ctx.ret` points at the return slot, `ctx.args` at the argument array.
export!(cdecl, rw_00b9af20(ctx: *mut NativeCtx) -> u32 {
    unsafe {
        let a = (*ctx).args;
        callee_cdecl!(1, u32, *a.add(0), *a.add(1), *a.add(2), *a.add(3), *a.add(4), *a.add(5))
    }
});
