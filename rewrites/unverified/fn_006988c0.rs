// original: 0x006988C0 bit_matrix_alloc_zero

/// Replace a bit matrix with a zeroed one of new dimensions.
///
/// `thiscall` with the object in ECX and width/height on the stack. Frees the
/// old word array through the TLS allocator when non-null, stores the new
/// dimensions, allocates `ceil(w*h/32)` words and zeroes them. Returns the
/// word count. The size multiply saturates to all-ones on overflow.
/// Original: 0x006988C0, 136 bytes.
lf_checker_rt::export!(thiscall, rw_006988C0(this: u32, w: u32, h: u32) -> u32 {
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
        let free: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(rd32(rd32(mgr) + VT_FREE_SLOT) as usize);
        let old = rd32(this);
        if old != 0 {
            let _ = free(mgr, old);
        }
        wr32(this + 4, w);
        let prod = w.wrapping_mul(h);
        wr32(this + 8, h);
        let mut nwords = prod >> 5;
        if prod & 0x1F != 0 {
            nwords = nwords.wrapping_add(1);
        }
        let (sz, ov) = nwords.overflowing_mul(4);
        let size = if ov { 0xFFFF_FFFF } else { sz };
        let p = alloc(mgr, size, ALLOC_ALIGN, 0);
        wr32(this, p);
        let mut i: u32 = 0;
        while i < nwords {
            wr32(p + i * 4, 0);
            i += 1;
        }
        nwords
    }
});
