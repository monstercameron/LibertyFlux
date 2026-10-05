// original: 0x006989B0 bit_array_copy_init

/// Attach a copied bit array, keeping the word count on the stack.
///
/// `thiscall` with the destination in ECX and a source descriptor on the
/// stack. Stores the width and flag byte, allocates `ceil(w/32)` words
/// through the TLS allocator and copies the source words, spilling the count
/// into its own incoming argument slot as scratch (the contract switches the
/// stack comparison off for this). Returns the destination.
/// Original: 0x006989B0, 122 bytes.
lf_checker_rt::export!(thiscall, rw_006989B0(this: u32, st: u32) -> u32 {
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
        wr8(this + 8, rd8(st + 8));
        let mut nwords = w >> 5;
        if w & 0x1F != 0 {
            nwords = nwords.wrapping_add(1);
        }
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
