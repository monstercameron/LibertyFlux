// original: 0x0069A470 crAnimChannelStaticFloat_ctor_default (proposed)

/// Default constructor: allocates 0xc bytes through the thread-local
/// allocator (`tls[0] -> [+8] -> vtable[+8]`, thiscall with size, 0x10, 0),
/// stamps the vtable and initializes the header fields. Returns the new
/// object, or null when allocation fails.
///
/// Original: 0x0069A470 (cdecl, no arguments).
lf_checker_rt::export!(cdecl, rw_0069A470() -> u32 {
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
        wr32(obj + 0x4, 0x400);
        wr32(obj, lf_checker_rt::relocated(0x00FE3C94));
        wr32(obj + 0x8, 0x0);
        obj

    }
});
