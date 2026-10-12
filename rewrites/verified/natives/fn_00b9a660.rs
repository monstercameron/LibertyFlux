// original: 0x00b9a660 GET_CLOSEST_STRAIGHT_ROAD
// GET_CLOSEST_STRAIGHT_ROAD: call the engine with (arg0..arg11); the low
// byte of the answer is zero-extended into the return slot.
export!(cdecl, rw_00b9a660(ctx: u32) -> u32 {
    unsafe {
        let a = args_of(ctx);
        let r = callee_cdecl!(1, u32, *a, *a.add(1), *a.add(2), *a.add(3),
            *a.add(4), *a.add(5), *a.add(6), *a.add(7), *a.add(8), *a.add(9),
            *a.add(10), *a.add(11));
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
