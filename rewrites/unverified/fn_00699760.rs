// original: 0x00699760 rage::crAnimChannelQuantizeFloat::vf1

/// Clone a quantize-float channel: header copy plus inner-array copy.
///
/// `thiscall` with the source in ECX, no stack arguments. Allocates 0x1C
/// bytes through the TLS allocator, copies the header bytes, stamps the class
/// vtable, runs the (intercepted) inner bit-array copy from `src+8` to
/// `new+8`, then copies the trailing words. Returns the clone, or null when
/// the allocation fails.
/// Original: 0x00699760, 100 bytes.
lf_checker_rt::export!(thiscall, rw_00699760(src: u32) -> u32 {
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
        let q = alloc(mgr, 0x1C, ALLOC_ALIGN, 0);
        if q == 0 {
            return 0;
        }
        wr32(q, lf_checker_rt::relocated(0x00FE3A74));
        wr8(q + 4, rd8(src + 4));
        wr8(q + 5, rd8(src + 5));
        wr16(q + 6, rd16(src + 6));
        wr32(q, lf_checker_rt::relocated(0x00FE3B8C));
        let _: u32 = lf_checker_rt::callee_thiscall!(2, u32, q + 8, src + 8);
        wr32(q + 0x14, rd32(src + 0x14));
        wr32(q + 0x18, rd32(src + 0x18));
        q
    }
});
