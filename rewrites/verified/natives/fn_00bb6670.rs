// original: 0x00BB6670 SET_INT_STAT
//
// Script native handler: forwards the stat id and value to one engine
// function (cdecl/2). No return value stored.
export!(cdecl, rw_00bb6670(ctx: *const u32) -> u32 {
    unsafe {
        let args = *(ctx.add(2) as *const *const u32);
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
