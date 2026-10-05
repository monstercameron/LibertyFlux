// original: 0x00698610 skeleton_component_init_698610 (proposed)

/// Initializer: zeroes the header words, stamps the vtable, then runs
/// two intercepted direct callees reached through the thread-local
/// block (`tls[0][+4]`): the first is asked about the member at
/// `+0xC` and a `-1` answer (or a missing block) zeroes it, otherwise
/// the second answer is added to it. Returns `this`.
///
/// Original: 0x00698610 (thiscall, one unread stack word).
lf_checker_rt::export!(thiscall, rw_00698610(this: u32, _a0: u32) -> u32 {
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
        wr32(this + 8, 0);
        wr32(this + 4, 0);
        wr32(this, lf_checker_rt::relocated(0x00FE3B00));
        let b = rd32(lf_checker_rt::tls_slot(0) + 4);
        if b == 0 {
            wr32(this + 0xC, 0);
            return this;
        }
        let c = rd32(b);
        let r1: u32 = lf_checker_rt::callee_thiscall!(3, u32, c, this + 0xC);
        if r1 == 0xFFFF_FFFF {
            wr32(this + 0xC, 0);
            return this;
        }
        let v = rd32(this + 0xC);
        if v == 0 {
            return this;
        }
        let r2: u32 = lf_checker_rt::callee_thiscall!(4, u32, b, v);
        wr32(this + 0xC, v.wrapping_add(r2));
        this

    }
});
