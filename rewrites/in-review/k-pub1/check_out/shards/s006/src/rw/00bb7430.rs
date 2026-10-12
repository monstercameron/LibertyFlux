// original: 0x00bb7430 IS_THIS_MODEL_A_PED
// IS_THIS_MODEL_A_PED: engine(arg0), zero-extend low byte into return slot.
export!(cdecl, rw_00bb7430(ctx: u32) -> u32 {
    unsafe {
        let a = args_of(ctx);
        let r = __lf_callee_cdecl!(1, u32, *a);
        *ret_of(ctx) = r & 0xFF;
        r
    }
});

/// Argument array of a native call context. (Carried from r-n09 via a-01.)
#[inline(always)]
pub unsafe fn args_of(ctx: u32) -> *const u32 {
    *((ctx as *const u32).add(2)) as *const u32
}

/// Return-slot pointer of a native call context. (Carried from r-n09 via a-01.)
#[inline(always)]
pub unsafe fn ret_of(ctx: u32) -> *mut u32 {
    *(ctx as *const u32) as *mut u32
}
