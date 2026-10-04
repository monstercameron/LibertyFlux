// original: 0x00b9ae60 SWITCH_PED_ROADS_BACK_TO_ORIGINAL
//
// Script native handler: forwards six float arguments (two xyz corners) to
// one engine function (cdecl/6). The original moves each through an SSE
// register into a stack argument block; the values reach the callee in
// argument order, bit for bit. No return value stored.
export!(cdecl, rw_00b9ae60(ctx: *const u32) -> u32 {
    unsafe {
        let args = *(ctx.add(2) as *const *const u32);
        callee_cdecl!(
            1,
            u32,
            *args,
            *args.add(1),
            *args.add(2),
            *args.add(3),
            *args.add(4),
            *args.add(5)
        )
    }
});
