// original: 0x0069AD30 animchan_copy_init_kind0300

/// Copy a channel header into place, then run the inner copy unless aliased.
///
/// `thiscall` with the destination in ECX and the source on the stack. Copies
/// the header bytes, stamps the class vtable, zeroes the inner words, clears
/// the allocator re-entrancy flag at TLS block `+0x34`, runs the (intercepted)
/// inner copy from `src+8` to `dst+8` unless both address the same words, and
/// sets the flag again. Returns the destination.
/// Original: 0x0069AD30, 98 bytes.
lf_checker_rt::export!(thiscall, rw_0069AD30(this: u32, src: u32) -> u32 {
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
        wr32(this, lf_checker_rt::relocated(0x00FE3A74));
        wr8(this + 4, rd8(src + 4));
        wr8(this + 5, rd8(src + 5));
        wr16(this + 6, rd16(src + 6));
        let inner = this.wrapping_add(8);
        wr32(this, lf_checker_rt::relocated(0x00FE3A1C));
        wr32(inner + 4, 0);
        let s8 = src.wrapping_add(8);
        let tblock = lf_checker_rt::tls_slot(0);
        wr32(inner, 0);
        wr8(tblock + 0x34, 0);
        if inner != s8 {
            let _: u32 = lf_checker_rt::callee_thiscall!(1, u32, inner, s8);
        }
        wr8(tblock + 0x34, 1);
        this
    }
});
