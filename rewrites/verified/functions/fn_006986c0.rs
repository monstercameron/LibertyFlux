// original: 0x006986C0 rage::crCreatureComponentSkeleton::vf11

/// Copy constructor: allocates 0x14 bytes through the thread-local
/// allocator, stamps the base vtable, copies the header fields from the
/// source object, then stamps the final vtable. Returns the new object,
/// or null when allocation fails.
///
/// Original: 0x006986C0 (thiscall, source in ecx).
lf_checker_rt::export!(thiscall, rw_006986C0(src: u32) -> u32 {
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
        let obj = alloc(heap_obj, 0x14, 0x10, 0);
        if obj == 0 {
            return 0;
        }
        wr32(obj, lf_checker_rt::relocated(0x00FE38AC));
        wr32(obj + 0x4, rd32(src + 0x4));
        wr32(obj + 0x8, rd32(src + 0x8));
        wr32(obj, lf_checker_rt::relocated(0x00FE3B00));
        wr32(obj + 0xC, rd32(src + 0xC));
        wr8(obj + 0x10, rd8(src + 0x10));
        wr8(obj + 0x11, rd8(src + 0x11));
        obj

    }
});
