// original: 0x006988C0 bitset_builder_6988C0 (proposed)

/// Bit-set zero builder: frees the old buffer through the
/// thread-local allocator, stores the two dimension arguments,
/// allocates `ceil(d0*d1/32)` words (byte size saturates to
/// all-ones on overflow) and zeroes them. Returns the word count.
///
/// Original: 0x006988C0 (thiscall, two dimension words on the stack).
lf_checker_rt::export!(thiscall, rw_006988C0(this: u32, d0: u32, d1: u32) -> u32 {
    unsafe {
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u16 {
            unsafe { (a as *const u16).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr16(a: u32, v: u16) {
            unsafe { (a as *mut u16).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }

        // Thread-allocator chain: tls slot 0 -> [+8] -> vtable slot +8.
        let heap_obj = rd32(lf_checker_rt::tls_slot(0) + 8);
        let vtable = rd32(heap_obj);
        let alloc: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(rd32(vtable + 8) as usize);

        // Thread-allocator chain: tls slot 0 -> [+8] -> vtable slot +0xc.
        let heap_obj = rd32(lf_checker_rt::tls_slot(0) + 8);
        let vtable = rd32(heap_obj);
        let free_mem: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(rd32(vtable + 0xC) as usize);
        let old = rd32(this);
        if old != 0 {
            free_mem(heap_obj, old);
        }
        wr32(this + 4, d0);
        wr32(this + 8, d1);
        let prod = d0.wrapping_mul(d1);
        let mut n = prod >> 5;
        if prod & 0x1F != 0 {
            n += 1;
        }
        let size = n.checked_mul(4).unwrap_or(0xFFFF_FFFF);
        let bits = alloc(heap_obj, size, 0x10, 0);
        wr32(this, bits);
        let prod2 = rd32(this + 4).wrapping_mul(rd32(this + 8));
        let mut m = prod2 >> 5;
        if prod2 & 0x1F != 0 {
            m += 1;
        }
        let mut i = 0u32;
        while i < m {
            wr32(bits.wrapping_add(i.wrapping_mul(4)), 0);
            i += 1;
        }
        m

    }
});
