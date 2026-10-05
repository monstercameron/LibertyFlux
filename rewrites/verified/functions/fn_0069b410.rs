// original: 0x0069B410 crAnimChannelStaticQuaternion_init (proposed)

/// In-place initializer: stamps the vtable with tag 0x900, zeroes the
/// member at `+8`, then allocates 0x10 bytes through the thread-local
/// allocator and stores the pointer there. Returns `this`.
///
/// Original: 0x0069B410 (thiscall, object in ecx).
lf_checker_rt::export!(thiscall, rw_0069B410(this: u32) -> u32 {
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
        wr32(this + 4, 0x900);
        wr32(this, lf_checker_rt::relocated(0x00FE3C3C));
        wr32(this + 8, 0);
        let member = alloc(heap_obj, 0x10, 0x10, 0);
        wr32(this + 8, member);
        this

    }
});
