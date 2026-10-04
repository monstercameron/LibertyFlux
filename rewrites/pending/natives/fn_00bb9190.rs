// original: 0x00bb9190 TASK_CAR_MISSION
//
// Script native handler: forwards eight script arguments to one engine
// function (cdecl/8). The speed argument (slot 4) travels through an SSE
// register and is bit-preserved. No return value stored.
export!(cdecl, rw_00bb9190(ctx: *const u32) -> u32 {
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
            *args.add(5),
            *args.add(6),
            *args.add(7)
        )
    }
});
