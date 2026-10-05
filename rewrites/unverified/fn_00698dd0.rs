// original: 0x00698DD0 rage::crAnimChannelDeltaFloat::vf0

/// Destroy a delta-float channel, then conditionally release it.
///
/// `thiscall` with the object in ECX and a flags word on the stack. Runs the
/// (intercepted) destructor, and when bit 0 of the flags is set and the object
/// is non-null releases it through the TLS allocator. Returns the object.
/// Original: 0x00698DD0, 42 bytes.
lf_checker_rt::export!(thiscall, rw_00698DD0(this: u32, flags: u32) -> u32 {
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
        let _: u32 = lf_checker_rt::callee_thiscall!(1, u32, this);
        if flags & 1 == 0 {
            return this;
        }
        if this == 0 {
            return this;
        }
        let _ = free(mgr, this);
        this
    }
});
