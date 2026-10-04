// original: 0x00B8B090 GET_NAME_OF_SCRIPT_TO_AUTOMATICALLY_START
/// Returns the auto-start script name pointer in the return slot.
///
/// Calls the engine function with no arguments and stores its full
/// 32-bit answer in the return slot.
lf_rn26_rt::export!(cdecl, rw_00B8B090(ctx: *const u8) -> u32 {
    unsafe {
        let ans: u32 = lf_rn26_rt::callee_cdecl!(1, u32,);
        let ret = *(ctx as *const *mut u32);
        *ret = ans;
        ans
    }
});
