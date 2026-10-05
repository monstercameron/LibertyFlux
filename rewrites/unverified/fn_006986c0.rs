// original: 0x006986C0 rage::crCreatureComponentSkeleton::vf11

/// Clone an animation-channel header into a fresh heap object.
///
/// `thiscall` with the source object in ECX and no stack arguments. Allocates
/// 0x14 bytes through the TLS allocator and copies the header words and flag
/// bytes from the source, stamping the class vtable. Returns the clone, or
/// null when the allocation fails.
/// Original: 0x006986C0, 83 bytes.
lf_checker_rt::export!(thiscall, rw_006986C0(src: u32) -> u32 {
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
        let q = alloc(mgr, 0x14, ALLOC_ALIGN, 0);
        if q == 0 {
            return 0;
        }
        wr32(q + 0x4, rd32(src + 0x4));
        wr32(q + 0x8, rd32(src + 0x8));
        wr32(q, lf_checker_rt::relocated(0x00FE3B00));
        wr32(q + 0xC, rd32(src + 0xC));
        wr8(q + 0x10, rd8(src + 0x10));
        wr8(q + 0x11, rd8(src + 0x11));
        q
    }
});
