// original: 0x0069A8F0 rage::crAnimChannelRawFloat::vf14

/// Rebuild a raw-float channel from a strided source table.
///
/// `thiscall` with the channel in ECX and source, count, stride plus one
/// ignored word on the stack. Frees the old sample buffer through the TLS
/// allocator, runs the (intercepted) resize for `count` samples, then copies
/// one word per sample from `src[i*stride]` (stride words are `arg2*4+4`
/// bytes apart). Returns the last word copied with its low byte forced to 1,
/// or the resize answer treated the same way when no sample is copied.
/// Original: 0x0069A8F0, 171 bytes.
lf_checker_rt::export!(thiscall, rw_0069A8F0(this: u32, src: u32, count: u32, stride: u32, _a3: u32) -> u32 {
    unsafe {
        #[allow(dead_code)]
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[allow(dead_code)]
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[allow(dead_code)]
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u16 {
            unsafe { (a as *const u16).read_unaligned() }
        }
        #[allow(dead_code)]
        #[inline(always)]
        unsafe fn wr16(a: u32, v: u16) {
            unsafe { (a as *mut u16).write_unaligned(v) }
        }
        #[allow(dead_code)]
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[allow(dead_code)]
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }
        const TLS_MGR_OFF: u32 = 0x08;
        const VT_ALLOC_SLOT: u32 = 0x08;
        const VT_FREE_SLOT: u32 = 0x0c;
        const ALLOC_ALIGN: u32 = 0x10;
        let tls = lf_checker_rt::tls_slot(0);
        let mgr = rd32(tls + TLS_MGR_OFF);
        let free: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(rd32(rd32(mgr) + VT_FREE_SLOT) as usize);
        let inner = this.wrapping_add(8);
        let old = rd32(inner);
        if old != 0 {
            let _ = free(mgr, old);
        }
        wr32(inner, 0);
        wr32(inner + 4, 0);
        let ans: u32 = lf_checker_rt::callee_thiscall!(2, u32, inner, count);
        let dst = rd32(inner);
        let step = stride.wrapping_mul(4).wrapping_add(4);
        let mut last = ans;
        let mut p = src;
        let mut i: i32 = 0;
        let total = count as i32;
        while i < total {
            let v = rd32(p);
            wr32(dst + (i as u32) * 4, v);
            last = v;
            p = p.wrapping_add(step);
            i += 1;
        }
        (last & 0xFFFF_FF00) | 1
    }
});
