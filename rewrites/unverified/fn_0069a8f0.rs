// original: 0x0069A8F0 rage::crAnimChannelRawFloat::vf14

/// Serializer: frees the old buffer, asks the intercepted direct
/// callee to size the member at `+8`, then gathers `count` dwords
/// from the source at a stride of `count+1` dwords (four at a time,
/// then a scalar tail). Returns the last source word read with its
/// low byte set to 1 (1 when nothing was read).
///
/// Original: 0x0069A8F0 (thiscall, source, count and two unused words).
lf_checker_rt::export!(thiscall, rw_0069A8F0(this: u32, src: u32, count: u32, _u1: u32, _u2: u32) -> u32 {
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
        let mut p = src;
        if (count as i32) >= 4 {
            let stride = count.wrapping_mul(4).wrapping_add(4);
            let limit = count.wrapping_sub(3);
            while (i as i32) < (limit as i32) {
                let dst = rd32(this + 8);
                for k in 0..4u32 {
                    last = rd32(p);
                    wr32(dst.wrapping_add(i.wrapping_mul(4)).wrapping_add(k * 4), last);
                    p = p.wrapping_add(stride);
                }
                i = i.wrapping_add(4);
            }
        }
        if (i as i32) < (count as i32) {
            let stride = count.wrapping_mul(4).wrapping_add(4);
            while (i as i32) < (count as i32) {
                let dst = rd32(this + 8);
                last = rd32(p);
                wr32(dst.wrapping_add(i.wrapping_mul(4)), last);
                p = p.wrapping_add(stride);
                i = i.wrapping_add(1);
            }
        }
        (last & 0xFFFF_FF00) | 1

    }
});
