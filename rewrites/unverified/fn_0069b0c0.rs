// original: 0x0069B0C0 rage::crAnimChannelRawInt::vf17

/// Rebuild a raw-int channel from a dense source array.
///
/// `thiscall` with the channel in ECX and source plus count on the stack.
/// Frees the old buffer through the TLS allocator, runs the (intercepted)
/// resize for `count` words, then copies the words. Returns the last word
/// copied with its low byte forced to 1, or the resize answer treated the
/// same way when the count is not positive.
/// Original: 0x0069B0C0, 87 bytes.
lf_checker_rt::export!(thiscall, rw_0069B0C0(this: u32, src: u32, count: u32) -> u32 {
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
        let mut last = ans;
        let mut i: i32 = 0;
        let total = count as i32;
        while i < total {
            let v = rd32(src + (i as u32) * 4);
            wr32(dst + (i as u32) * 4, v);
            last = v;
            i += 1;
        }
        (last & 0xFFFF_FF00) | 1
    }
});
