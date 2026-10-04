// original: 0x00a01f20 SET_OBJECT_ROTATION
//
// Script native handler: forwards the object handle and three rotation
// floats to one engine function (cdecl/4). The original shuffles the floats
// through SSE registers into a stack argument block; the values reach the
// callee in argument order, bit for bit.
export!(cdecl, rw_00a01f20(ctx: *const u32) -> u32 {
    unsafe {
        let args = *(ctx.add(2) as *const *const u32);
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2), *args.add(3))
    }
});
