// original: 0x00698CC0 rage::crAnimChannelDeltaFloat::dtor

/// Destroy an animation-channel object, freeing its owned buffers.
///
/// `thiscall` with the object in ECX, no stack arguments. Stamps the class
/// vtable, frees each non-null owned pointer through the TLS allocator, then
/// stamps the base vtable. Returns the allocator's last answer, or the entry
/// EAX residue when nothing was freed (pinned by the contract).
/// Original: 0x00698CC0, 80 bytes.
lf_checker_rt::export!(thiscall, rw_00698CC0(this: u32) -> u32 {
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
        wr32(this, lf_checker_rt::relocated(0x00FE3B34));
        let mut ans: u32 = 0x12345678;
        let v0 = rd32(this + 0x20);
        if v0 != 0 {
            ans = free(mgr, v0);
        }
        let v1 = rd32(this + 0x14);
        if v1 != 0 {
            ans = free(mgr, v1);
        }
        let v2 = rd32(this + 0x8);
        if v2 != 0 {
            ans = free(mgr, v2);
        }
        wr32(this, lf_checker_rt::relocated(0x00FE3A74));
        ans
    }
});
