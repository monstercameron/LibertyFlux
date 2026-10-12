// original: 0x00C7A190 cache_buffer_append

/// Append bytes to the cache buffer. Copies a1 bytes from a0 at the current used size through the block helper, then grows the used size by a1. Original: cdecl, two stack words.
lf_checker_rt::export!(cdecl, rw_c7a190(src: u32, len: u32) -> () {
    unsafe {
        #[inline(always)]
        unsafe fn rd32(address: u32) -> u32 { unsafe { (address as *const u32).read_unaligned() } }
        #[inline(always)]
        unsafe fn wr32(address: u32, value: u32) { unsafe { (address as *mut u32).write_unaligned(value) } }
        #[inline(always)]
        unsafe fn rd16(address: u32) -> u16 { unsafe { (address as *const u16).read_unaligned() } }
        #[inline(always)]
        unsafe fn wr16(address: u32, value: u16) { unsafe { (address as *mut u16).write_unaligned(value) } }
        #[inline(always)]
        unsafe fn rd8(address: u32) -> u8 { unsafe { (address as *const u8).read() } }
        #[inline(always)]
        unsafe fn wr8(address: u32, value: u8) { unsafe { (address as *mut u8).write(value) } }

        const USED_GVA: u32 = 0x016F7D74;
                let used = rd32(lf_checker_rt::relocated(USED_GVA));
                let _ = lf_checker_rt::callee_cdecl!(1, u32, used, src, len);
                wr32(lf_checker_rt::relocated(USED_GVA), used.wrapping_add(len));
        }
});
