// original: 0x0069A4C0 rage::crAnimChannelStaticQuaternion::vf1

/// Copy constructor: allocates 0xc bytes through the thread-local
/// allocator and runs the intercepted direct constructor on the fresh
/// object with the source. Returns the constructor result, or null when
/// allocation fails.
///
/// Original: 0x0069A4C0 (thiscall, source in ecx).
lf_checker_rt::export!(thiscall, rw_0069A4C0(src: u32) -> u32 {
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
        lf_checker_rt::callee_thiscall!(3, u32, obj, src)

    }
});
