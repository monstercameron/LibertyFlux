// original: 0x00bba4d0 TASK_SAY
//
// Script native handler: forwards two script arguments to one engine
// function (cdecl/2). No return value stored.
export!(cdecl, rw_00bba4d0(ctx: *const u32) -> u32 {
    unsafe {
        let args = *(ctx.add(2) as *const *const u32);
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
