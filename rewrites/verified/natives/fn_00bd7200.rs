// original: 0x00bd7200 GET_HOURS_OF_DAY
// GET_HOURS_OF_DAY: call the engine with no args, store eax in return slot.
export!(cdecl, rw_00bd7200(ctx: u32) -> u32 {
    unsafe {
        let r = callee_cdecl!(1, u32,);
        *ret_of(ctx) = r;
        r
    }
});

/// Return-slot pointer of a native call context. (Carried from r-n09 via a-01.)
#[inline(always)]
pub unsafe fn ret_of(ctx: u32) -> *mut u32 {
    *(ctx as *const u32) as *mut u32
}
