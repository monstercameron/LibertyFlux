// original: 0x00bb2430 IS_PLAYER_FREE_AIMING_AT_CHAR
// IS_PLAYER_FREE_AIMING_AT_CHAR: engine(arg0, arg1), low byte to return slot.
export!(cdecl, rw_00bb2430(ctx: u32) -> u32 {
    unsafe {
        let a = args_of(ctx);
        let r = callee_cdecl!(1, u32, *a, *a.add(1));
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
