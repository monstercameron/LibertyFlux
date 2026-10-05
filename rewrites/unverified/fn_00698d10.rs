// original: 0x00698D10 rage::crAnimChannelDeltaFloat::vf1

/// Clone an animation channel by allocating plus a constructor call.
///
/// `thiscall` with the source in ECX, no stack arguments. Allocates 0x38
/// bytes through the TLS allocator, runs the (intercepted) copy constructor
/// on the new object with the source as its argument, and returns whatever
/// the constructor returns; null when the allocation fails.
/// Original: 0x00698D10, 43 bytes.
lf_checker_rt::export!(thiscall, rw_00698D10(src: u32) -> u32 {
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
        let alloc: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(rd32(rd32(mgr) + VT_ALLOC_SLOT) as usize);
        let q = alloc(mgr, 0x38, ALLOC_ALIGN, 0);
        if q == 0 {
            return 0;
        }
        let ans: u32 = lf_checker_rt::callee_thiscall!(2, u32, q, src);
        ans
    }
});
