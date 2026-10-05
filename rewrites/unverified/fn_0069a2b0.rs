// original: 0x0069A2B0 rage::crAnimChannelRawInt::vf0

/// Conditionally destroy and release an animation channel.
///
/// `thiscall` with the object in ECX and a flags word on the stack. Stamps
/// the class vtable, frees the buffer at `+8` when the count word at `+0x0E` is non-zero, stamps the base vtable, and when bit 0 of the
/// flags is set releases the object itself through the TLS allocator.
/// Returns the object.
/// Original: 0x0069A2B0, 73 bytes.
lf_checker_rt::export!(thiscall, rw_0069A2B0(this: u32, flags: u32) -> u32 {
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
        wr32(this, lf_checker_rt::relocated(0x00FE3BE4));
        if rd16(this + 0x0E) != 0 {
            let inner = rd32(this + 8);
            if inner != 0 {
                let _ = free(mgr, inner);
            }
        }
        wr32(this, lf_checker_rt::relocated(0x00FE3A74));
        if flags & 1 != 0 {
            let _ = free(mgr, this);
        }
        this
    }
});
