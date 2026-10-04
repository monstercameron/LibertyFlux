// original: 0x00ba2ad0 WARP_CHAR_FROM_CAR_TO_COORD
/// Native handler `WARP_CHAR_FROM_CAR_TO_COORD` (script context in, engine call out).
///
/// Pulls the char out of the car at a coord; forwards handle and three floats.
/// `ctx.ret` points at the return slot, `ctx.args` at the argument array.
export!(cdecl, rw_00ba2ad0(ctx: *mut NativeCtx) -> u32 {
    unsafe {
        let a = (*ctx).args;
        callee_cdecl!(1, u32, *a.add(0), *a.add(1), *a.add(2), *a.add(3))
    }
});
