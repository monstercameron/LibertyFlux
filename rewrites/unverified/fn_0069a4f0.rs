// original: 0x0069A4F0 crAnimChannelStaticQuaternion_ctor_default (proposed)

/// Default constructor: allocates the 0xc-byte object through the
/// thread-local allocator, stamps the vtable with tag 0x900, then allocates
/// a 0x10-byte member the same way and stores it at `+8`. Returns the new
/// object, or null when the first allocation fails.
///
/// Original: 0x0069A4F0 (cdecl, no arguments).
lf_checker_rt::export!(cdecl, rw_0069A4F0() -> u32 {
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
        let obj = alloc(heap_obj, 0xC, 0x10, 0);
        if obj == 0 {
            return 0;
        }
        wr32(obj + 4, 0x900);
        wr32(obj, lf_checker_rt::relocated(0x00FE3C3C));
        wr32(obj + 8, 0);
        let member = alloc(heap_obj, 0x10, 0x10, 0);
        wr32(obj + 8, member);
        obj

    }
});
