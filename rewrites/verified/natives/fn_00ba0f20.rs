// original: 0x00ba0f20 SET_CHAR_ANGLED_DEFENSIVE_AREA
/// Native handler `SET_CHAR_ANGLED_DEFENSIVE_AREA` (script context in, engine call out).
///
/// Sets an angled defensive area; forwards the ped handle and seven floats.
/// `ctx.ret` points at the return slot, `ctx.args` at the argument array.
export!(cdecl, rw_00ba0f20(ctx: *mut NativeCtx) -> u32 {
    unsafe {
        let a = (*ctx).args;
        callee_cdecl!(1, u32, *a.add(0), *a.add(1), *a.add(2), *a.add(3), *a.add(4), *a.add(5), *a.add(6), *a.add(7))
    }
});
