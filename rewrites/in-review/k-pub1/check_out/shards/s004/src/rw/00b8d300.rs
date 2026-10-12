// original: 0x00B8D300 PRINT_WITH_6_NUMBERS_NOW
//
// Forwards all nine print arguments (string plus numbers) to the engine
// text routine. No return value.
export!(cdecl, rw_00b8d300(ctx: *mut u8) -> u32 {
    unsafe {
        let args = args_of(ctx);
        __lf_callee_cdecl!(
            1, u32, *args, *args.add(1), *args.add(2), *args.add(3), *args.add(4),
            *args.add(5), *args.add(6), *args.add(7), *args.add(8)
        )
    }
});

/// Read the argument array pointer out of a native call context (r-n15).
#[inline(always)]
unsafe fn args_of(ctx: *mut u8) -> *const u32 {
    *(ctx.add(8) as *const *const u32)
}
