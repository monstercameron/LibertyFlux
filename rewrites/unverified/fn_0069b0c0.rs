// original: 0x0069B0C0 rage::crAnimChannelRawInt::vf17

/// Serializer: frees the old buffer, asks the intercepted direct
/// callee to size the member at `+8`, then copies `count` dwords
/// from the source. Returns the last source word read with its low
/// byte set to 1 (1 when nothing was read).
///
/// Original: 0x0069B0C0 (thiscall, source and count on the stack).
lf_checker_rt::export!(thiscall, rw_0069B0C0(this: u32, src: u32, count: u32) -> u32 {
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
        let old = rd32(this + 8);
        if old != 0 {
            free_mem(heap_obj, old);
        }
        wr32(this + 8, 0);
        wr32(this + 8 + 4, 0);
        lf_checker_rt::callee_thiscall!(3, u32, this + 8, count);
        let mut last: u32 = 0;
        let mut i: u32 = 0;
        while (i as i32) < (count as i32) {
            let dst = rd32(this + 8);
            last = rd32(src.wrapping_add(i.wrapping_mul(4)));
            wr32(dst.wrapping_add(i.wrapping_mul(4)), last);
            i = i.wrapping_add(1);
        }
        (last & 0xFFFF_FF00) | 1

    }
});
