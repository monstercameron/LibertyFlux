// original: 0x0069A3D0 rage::crAnimChannelRawBool::vf0

/// Deleting destructor: stamps the vtable, frees the member at `+x8`
/// through the thread-local allocator when the gate word at `+0xE` is nonzero, stamps the base vtable, then
/// frees the object itself when the low bit of the flag argument is
/// set. Returns the object pointer.
///
/// Original: 0x0069A3D0 (thiscall, flag word on the stack).
lf_checker_rt::export!(thiscall, rw_0069A3D0(obj: u32, flag: u32) -> u32 {
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
        wr32(obj, lf_checker_rt::relocated(0x00FE3D44));
        if rd16(obj + 0xE) != 0 {
        let member = rd32(obj + 0x8);
            if member != 0 {
                free_mem(heap_obj, member);
            }
        }
        wr32(obj, lf_checker_rt::relocated(0x00FE3A74));
        if flag & 1 != 0 {
            free_mem(heap_obj, obj);
        }
        obj

    }
});
