// original: 0x00e66fe0 query_and_store_global_1 (proposed)
/// Call a cdecl/2 callee with a fixed address and 0, store the result.
///
/// Calls `callee(ADDR, 0)`, cleans the two pushed words, and writes the
/// return value to a global; eax still holds it on return. No arguments
/// (cdecl/0). Calling convention: cdecl.
lf_checker_rt::export!(cdecl, rw_00e66fe0() -> u32 {
    unsafe {
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (lf_checker_rt::relocated(a) as *mut u32).write_unaligned(v) }
        }
        const ADDR: u32 = 0x00E9AC24;
        const DST: u32 = 0x012BD1D8;
        let r = lf_checker_rt::callee_cdecl!(1, u32, lf_checker_rt::relocated(ADDR), 0);
        wr32(DST, r);
        r
    }
});
