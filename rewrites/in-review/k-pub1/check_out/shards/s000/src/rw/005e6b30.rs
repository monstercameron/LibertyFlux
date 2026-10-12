// original: 0x005E6B30 TASK_CHAR_ARREST_CHAR
//
// Forwards the two character handles to the engine task routine.
// No return value.
export!(cdecl, rw_005e6b30(ctx: *mut u8) -> u32 {
    unsafe {
        let args = args_of(ctx);
        __lf_callee_cdecl!(1, u32, *args, *args.add(1))
    }
});

/// Read the argument array pointer out of a native call context (r-n15).
#[inline(always)]
unsafe fn args_of(ctx: *mut u8) -> *const u32 {
    *(ctx.add(8) as *const *const u32)
}
