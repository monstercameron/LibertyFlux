// original: 0x009cc3a0 REQUEST_MISSION_AUDIO_BANK
// REQUEST_MISSION_AUDIO_BANK: engine(arg0), low byte to return slot.
export!(cdecl, rw_009cc3a0(ctx: u32) -> u32 {
    unsafe {
        let a = args_of(ctx);
        let r = callee_cdecl!(1, u32, *a);
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
