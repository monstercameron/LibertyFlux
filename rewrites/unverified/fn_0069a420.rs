// original: 0x0069A420 rage::crAnimChannelStaticFloat::vf1

/// Clone a static animation channel (header bytes plus one payload word).
///
/// `thiscall` with the source in ECX, no stack arguments. Allocates 0xC
/// bytes through the TLS allocator, copies the two flag bytes at `+4`/`+5`
/// and the word at `+6`, stamps the class vtable and copies the payload word
/// at `+8`. Returns the clone, or null on allocation failure.
/// Original: 0x0069A420, 79 bytes.
lf_checker_rt::export!(thiscall, rw_0069A420(src: u32) -> u32 {
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
        let q = alloc(mgr, 0xC, ALLOC_ALIGN, 0);
        if q == 0 {
            return 0;
        }
        wr32(q, lf_checker_rt::relocated(0x00FE3A74));
        wr8(q + 4, rd8(src + 4));
        wr8(q + 5, rd8(src + 5));
        wr16(q + 6, rd16(src + 6));
        wr32(q, lf_checker_rt::relocated(0x00FE3C94));
        wr32(q + 8, rd32(src + 8));
        q
    }
});
