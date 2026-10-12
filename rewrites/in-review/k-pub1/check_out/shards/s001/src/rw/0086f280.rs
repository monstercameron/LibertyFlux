// original: 0x0086F280 EXP
//
// Loads the float argument and calls the CRT exponential; the callee takes
// its input in a vector register and returns the result the same way (both
// unobservable under interception), and the handler stores the result word
// into the return slot.
export!(cdecl, rw_0086f280(ctx: *mut u8) -> u32 {
    unsafe {
        let args = args_of(ctx);
        let _input = *args;
        let result = __lf_callee_cdecl!(1, u32,);
        *retslot_of(ctx) = result;
        result
    }
});

/// Read the argument array pointer out of a native call context (r-n15).
#[inline(always)]
unsafe fn args_of(ctx: *mut u8) -> *const u32 {
    *(ctx.add(8) as *const *const u32)
}

/// Read the return-slot pointer out of a native call context (r-n15).
#[allow(dead_code)]
#[inline(always)]
unsafe fn retslot_of(ctx: *mut u8) -> *mut u32 {
    *(ctx as *const *mut u32)
}
