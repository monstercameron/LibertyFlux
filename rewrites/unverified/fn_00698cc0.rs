// original: 0x00698CC0 crAnimChannelDeltaFloat_dtor (proposed)

/// Destructor: stamps the vtable, frees each non-null member at
/// `+0x20`, `+0x14` and `+8` through the thread-local allocator, then
/// stamps the base vtable. Returns nothing meaningful (void).
///
/// Original: 0x00698CC0 (thiscall, object in ecx).
lf_checker_rt::export!(thiscall, rw_00698CC0(obj: u32) -> u32 {
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

        // Thread-allocator chain: tls slot 0 -> [+8] -> vtable slot +0xc.
        let heap_obj = rd32(lf_checker_rt::tls_slot(0) + 8);
        let vtable = rd32(heap_obj);
        let free_mem: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(rd32(vtable + 0xC) as usize);
        wr32(obj, lf_checker_rt::relocated(0x00FE3B34));
        let m0 = rd32(obj + 0x20);
        if m0 != 0 {
            free_mem(heap_obj, m0);
        }
        let m1 = rd32(obj + 0x14);
        if m1 != 0 {
            free_mem(heap_obj, m1);
        }
        let m2 = rd32(obj + 0x8);
        if m2 != 0 {
            free_mem(heap_obj, m2);
        }
        wr32(obj, lf_checker_rt::relocated(0x00FE3A74));
        0

    }
});
