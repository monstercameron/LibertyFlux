// original: 0x0069A1E0 rage::crAnimChannelRawInt::vf1

/// Copy constructor: allocates 0x10 bytes through the thread-local
/// allocator, stamps the base vtable, copies the header bytes from the
/// source, stamps the final vtable, then copies the embedded member at
/// `+8` through the intercepted direct callee. Returns the new object,
/// or null when allocation fails.
///
/// Original: 0x0069A1E0 (thiscall, source in ecx).
lf_checker_rt::export!(thiscall, rw_0069A1E0(src: u32) -> u32 {
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
        let obj = alloc(heap_obj, 0x10, 0x10, 0);
        if obj == 0 {
            return 0;
        }
        wr32(obj, lf_checker_rt::relocated(0x00FE3A74));
        wr8(obj + 0x4, rd8(src + 0x4));
        wr8(obj + 0x5, rd8(src + 0x5));
        wr16(obj + 0x6, rd16(src + 0x6));
        wr32(obj, lf_checker_rt::relocated(0x00FE3BE4));
        wr32(obj + 0x8, 0);
        wr32(obj + 0xC, 0);
        lf_checker_rt::callee_thiscall!(3, u32, obj + 8, src + 8);
        obj

    }
});
