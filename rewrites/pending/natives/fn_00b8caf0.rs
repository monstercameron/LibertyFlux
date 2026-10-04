// original: 0x00B8CAF0 GET_ROUTE_SIZE
/// Returns the route count in the return slot.
///
/// Calls the engine function with no arguments and stores its full
/// 32-bit answer in the return slot.
lf_rn26_rt::export!(cdecl, rw_00B8CAF0(ctx: *const u8) -> u32 {
    unsafe {
        let ans: u32 = lf_rn26_rt::callee_cdecl!(1, u32,);
        let ret = *(ctx as *const *mut u32);
        *ret = ans;
        ans
    }
});
