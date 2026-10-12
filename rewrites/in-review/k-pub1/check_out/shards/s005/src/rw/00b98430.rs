// original: 0x00b98430 IS_AUTO_AIMING_ON
// IS_AUTO_AIMING_ON: engine(), zero-extend low byte into return slot.
export!(cdecl, rw_00b98430(ctx: u32) -> u32 {
    unsafe {
        let r = __lf_callee_cdecl!(1, u32,);
        *ret_of(ctx) = r & 0xFF;
        r
    }
});

/// Return-slot pointer of a native call context. (Carried from r-n09 via a-01.)
#[inline(always)]
pub unsafe fn ret_of(ctx: u32) -> *mut u32 {
    *(ctx as *const u32) as *mut u32
}
