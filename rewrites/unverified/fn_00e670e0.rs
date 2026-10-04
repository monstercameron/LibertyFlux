// original: 0x00e670e0 CCamFollowPed::PlayerHeadLookAt
/// Call a cdecl/2 callee with a fixed address and 0, store the result.
///
/// Same shape as 0x00e66fe0 at different addresses. Calls `callee(ADDR, 0)`
/// and writes the return value to a global; eax still holds it on return.
/// No arguments (cdecl/0). Calling convention: cdecl.
lf_checker_rt::export!(cdecl, rw_00e670e0() -> u32 {
    unsafe {
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (lf_checker_rt::relocated(a) as *mut u32).write_unaligned(v) }
        }
        const ADDR: u32 = 0x00E9B470;
        const DST: u32 = 0x012DD2AC;
        let r = lf_checker_rt::callee_cdecl!(1, u32, lf_checker_rt::relocated(ADDR), 0);
        wr32(DST, r);
        r
    }
});
