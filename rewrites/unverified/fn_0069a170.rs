// original: 0x0069A170 animchan_create_kind0300

/// Allocate and initialise one animation-channel object.
///
/// Takes no arguments (cdecl). Reads the thread-local allocator through TLS
/// slot 0, allocates 0x10 bytes, and on success stamps the kind word at
/// `+4`, the vtable pointer at `+0` and zeroed payload words. Returns the new
/// object, or null when the allocator answers null.
/// Original: 0x0069A170, 55 bytes.
lf_checker_rt::export!(cdecl, rw_0069A170() -> u32 {
    unsafe {
        #[allow(dead_code)]
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[allow(dead_code)]
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[allow(dead_code)]
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u16 {
            unsafe { (a as *const u16).read_unaligned() }
        }
        #[allow(dead_code)]
        #[inline(always)]
        unsafe fn wr16(a: u32, v: u16) {
            unsafe { (a as *mut u16).write_unaligned(v) }
        }
        #[allow(dead_code)]
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[allow(dead_code)]
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }
        const TLS_MGR_OFF: u32 = 0x08;
        const VT_ALLOC_SLOT: u32 = 0x08;
        const VT_FREE_SLOT: u32 = 0x0c;
        const ALLOC_ALIGN: u32 = 0x10;
        let tls = lf_checker_rt::tls_slot(0);
        let mgr = rd32(tls + TLS_MGR_OFF);
        let alloc: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(rd32(rd32(mgr) + VT_ALLOC_SLOT) as usize);
        let p = alloc(mgr, 0x10, ALLOC_ALIGN, 0);
        if p == 0 {
            return 0;
        }
        wr32(p + 0x4, 0x300);
        wr32(p, lf_checker_rt::relocated(0x00FE3A1C));
        wr32(p + 0x8, 0x0);
        wr32(p + 0xC, 0x0);
        p
    }
});
