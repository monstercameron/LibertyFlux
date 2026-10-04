// original: 0x00b8d070 PRINT_STRING_IN_STRING
/// Forward four print arguments to the text engine function.
export!(cdecl, rw_00b8d070(ctx: u32) -> u32 {
    const ARG_ARRAY: usize = 2;
    let c = ctx as *const u32;
    let args = unsafe { *c.add(ARG_ARRAY) } as *const u32;
    let a0 = unsafe { *args.add(0) };
    let a1 = unsafe { *args.add(1) };
    let a2 = unsafe { *args.add(2) };
    let a3 = unsafe { *args.add(3) };
    callee_cdecl!(1, u32, a0, a1, a2, a3)
});
