// original: 0x006989B0 bitset_builder_6989B0 (proposed)

/// Bit-set copy builder: stores the count word and flag byte from
/// the argument block, sizes the set as `ceil(count/32)` words
/// (byte size saturates to all-ones on overflow), allocates it
/// through the thread-local allocator and copies the words. The
/// original spills the count to its incoming argument slot and
/// reloads it as the loop bound; the rewrite keeps it in a local.
/// Returns `this`.
///
/// Original: 0x006989B0 (thiscall, argument block on the stack).
lf_checker_rt::export!(thiscall, rw_006989B0(this: u32, arg: u32) -> u32 {
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
        wr8(this + 8, rd8(arg + 8));
        let count = rd32(this + 4);
        let mut dwords = count >> 5;
        if count & 0x1F != 0 {
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
