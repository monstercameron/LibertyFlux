// original: 0x00698840 bit_matrix_copy_init

/// Attach a copied bit matrix: dimensions stored, bits duplicated.
///
/// `thiscall` with the destination in ECX and a source descriptor on the
/// stack. Stores the width/height, allocates room for `ceil(w*h/32)` words
/// through the TLS allocator and copies the source words. Returns the
/// destination. The size multiply saturates to all-ones on overflow.
/// Original: 0x00698840, 119 bytes.
lf_checker_rt::export!(thiscall, rw_00698840(this: u32, st: u32) -> u32 {
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
        let w = rd32(st + 4);
        wr32(this + 4, w);
        let h = rd32(st + 8);
        let prod = w.wrapping_mul(h);
        let mut nwords = prod >> 5;
        if prod & 0x1F != 0 {
            nwords = nwords.wrapping_add(1);
        }
        wr32(this + 8, h);
        let (sz, ov) = nwords.overflowing_mul(4);
        let size = if ov { 0xFFFF_FFFF } else { sz };
        let p = alloc(mgr, size, ALLOC_ALIGN, 0);
        wr32(this, p);
        let arr = rd32(st);
        let mut i: u32 = 0;
        while i < nwords {
            wr32(p + i * 4, rd32(arr + i * 4));
            i += 1;
        }
        this
    }
});
