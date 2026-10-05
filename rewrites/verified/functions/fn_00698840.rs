// original: 0x00698840 bitset_builder_698840 (proposed)

/// Bit-set copy builder: stores the two dimension words from the
/// argument block, sizes the bit set as `ceil(d0*d1/32)` words
/// (allocation size in bytes saturates to all-ones on 32-bit
/// overflow), allocates it through the thread-local allocator and
/// copies the words from the source table. Returns `this`.
///
/// Original: 0x00698840 (thiscall, argument block on the stack).
lf_checker_rt::export!(thiscall, rw_00698840(this: u32, arg: u32) -> u32 {
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
        wr32(this + 4, rd32(arg + 4));
        let d1 = rd32(arg + 8);
        wr32(this + 8, d1);
        let prod = rd32(this + 4).wrapping_mul(d1);
        let mut dwords = prod >> 5;
        if prod & 0x1F != 0 {
            dwords += 1;
        }
        let size = dwords.checked_mul(4).unwrap_or(0xFFFF_FFFF);
        let bits = alloc(heap_obj, size, 0x10, 0);
        wr32(this, bits);
        let src = rd32(arg);
        let mut i = 0u32;
        while i < dwords {
            wr32(bits.wrapping_add(i.wrapping_mul(4)),
                 rd32(src.wrapping_add(i.wrapping_mul(4))));
            i += 1;
        }
        this

    }
});
