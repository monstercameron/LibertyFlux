// original: 0x0069D500 rage::crAnimChannelRleInt::copy_from

/// Copy-constructs an RLE integer channel from another one.
///
/// `this` is the destination and the stack argument the source. The base
/// header (bytes at `+4`/`+5`, word at `+6`) is copied, the sample list at
/// `+8` and the bit stream at `+0x10` are duplicated through their copiers
/// (callees 1 and 2, thiscall: destination part, source part; both answers
/// discarded). The vtable is stamped with the base channel's table first,
/// then the RleInt table. Returns `this`.
///
/// Original: 0x0069D500 (thiscall, one stack word, callee pops 4).
lf_checker_rt::export!(thiscall, rw_0069D500(this: u32, src: u32) -> u32 {
    unsafe {
        const BASE_VTABLE: u32 = 0xFE3A74;
        const RLE_VTABLE: u32 = 0xFE3EBC;
        const LIST_OFF: u32 = 8;
        const BITS_OFF: u32 = 0x10;
        const LIST_COPY: u32 = 1;
        const BITS_COPY: u32 = 2;
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u32 {
            unsafe { (a as *const u16).read_unaligned() as u32 }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        wr32(this, lf_checker_rt::relocated(BASE_VTABLE));
        unsafe {
            let b4 = ((src + 4) as *const u8).read();
            ((this + 4) as *mut u8).write(b4);
            let b5 = ((src + 5) as *const u8).read();
            ((this + 5) as *mut u8).write(b5);
            let w6 = ((src + 6) as *const u16).read_unaligned();
            ((this + 6) as *mut u16).write_unaligned(w6);
        }
        wr32(this, lf_checker_rt::relocated(RLE_VTABLE));
        wr32(this + LIST_OFF + 4, 0);
        wr32(this + LIST_OFF, 0);
        lf_checker_rt::callee_thiscall!(LIST_COPY, u32, this + LIST_OFF, src + LIST_OFF);
        lf_checker_rt::callee_thiscall!(BITS_COPY, u32, this + BITS_OFF, src + BITS_OFF);
        this
    }
});
