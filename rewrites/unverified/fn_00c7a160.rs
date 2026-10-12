// original: 0x00C7A160 module_cache_free

/// Release the module cache. When the loaded flag byte is zero there is nothing to do; otherwise free the cache buffer, clear the buffer pointer and the used size. Original: cdecl, no stack words.
lf_checker_rt::export!(cdecl, rw_c7a160() -> () {
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

        const FLAG_GVA: u32 = 0x0104B8FC;
                const BUF_GVA: u32 = 0x016F7D70;
                const USED_GVA: u32 = 0x016F7D74;
                if rd8(lf_checker_rt::relocated(FLAG_GVA)) == 0 {
                    return;
                }
                let _ = lf_checker_rt::callee_cdecl!(1, u32, rd32(lf_checker_rt::relocated(BUF_GVA)));
                wr32(lf_checker_rt::relocated(BUF_GVA), 0);
                wr32(lf_checker_rt::relocated(USED_GVA), 0);
        }
});
