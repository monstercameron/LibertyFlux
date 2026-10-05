// original: 0x0069AD30 crAnimChannelRawQuaternion_copy_ctor (proposed)

/// Member-copy constructor: stamps the base vtable, copies the header
/// bytes from the source, stamps the final vtable, zeroes the member
/// at `+8`, then copies that member through the intercepted direct
/// callee unless source and destination are the same object. A flag
/// byte at `+0x34` of the thread-local block is cleared around the
/// call. Returns `this`.
///
/// Original: 0x0069AD30 (thiscall, source on the stack).
lf_checker_rt::export!(thiscall, rw_0069AD30(this: u32, arg: u32) -> u32 {
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
        wr32(this, lf_checker_rt::relocated(0x00FE3A74));
        wr8(this + 4, rd8(arg + 4));
        wr8(this + 5, rd8(arg + 5));
        wr16(this + 6, rd16(arg + 6));
        wr32(this, lf_checker_rt::relocated(0x00FE3A1C));
        wr32(this + 12, 0);
        wr32(this + 8, 0);
        let tls = lf_checker_rt::tls_slot(0);
        wr8(tls + 0x34, 0);
        if this.wrapping_add(8) != arg.wrapping_add(8) {
            lf_checker_rt::callee_thiscall!(3, u32, this + 8, arg + 8);
        }
        wr8(tls + 0x34, 1);
        this

    }
});
