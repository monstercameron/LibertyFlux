// original: 0x00e670c0 global_copy_3 (proposed)
/// Copy one global word to another global word.
///
/// Same shape as 0x00e66fc0 at different addresses. No arguments (cdecl/0),
/// no calls. Calling convention: cdecl.
lf_checker_rt::export!(cdecl, rw_00e670c0() -> u32 {
    unsafe {
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (lf_checker_rt::relocated(a) as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (lf_checker_rt::relocated(a) as *mut u32).write_unaligned(v) }
        }
        const SRC: u32 = 0x01048AB4;
        const DST: u32 = 0x0103C060;
        wr32(DST, rd32(SRC));
        0
    }
});
