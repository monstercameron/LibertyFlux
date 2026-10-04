// original: 0x00bd83d0 NETWORK_GET_HOST_MATCH_PROGRESS
/// Native handler `NETWORK_GET_HOST_MATCH_PROGRESS`.
///
/// Reads the host match progress value.
///
/// Handler mechanics: takes the native call context,
/// Forwards the host argument, then stores the engine answer in the
/// return slot.
lf_rn21_rt::export!(cdecl, rw_00bd83d0(ctx: u32) -> () {
    let args = unsafe { *((ctx.wrapping_add(8)) as *const u32) } as *const u32;
    let ret = unsafe { *(ctx as *const u32) } as *mut u32;
    let host = unsafe { *args };
    let progress = lf_rn21_rt::callee_cdecl!(1, u32, host);
    unsafe { *ret = progress };
});
