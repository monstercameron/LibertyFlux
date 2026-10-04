// original: 0x00B8CFF0 PRINT_HELP_WITH_STRING
//
// Script native handler: forwards two string slots to one engine function
// (cdecl/2). No return value stored.
export!(cdecl, rw_00b8cff0(ctx: *const u32) -> u32 {
    unsafe {
        let args = *(ctx.add(2) as *const *const u32);
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
