// original: 0x0069BB70 rage::crAnimChannelCurveFloat::copy_from

/// Copy-constructs a curve-float channel from another one.
///
/// `this` is the destination and the stack argument the source. The base
/// header (bytes at `+4`/`+5`, word at `+6`) is copied, the key list at
/// `+8` is duplicated through the key copier (callee 1, thiscall:
/// destination list, source list; its answer is discarded), and the two
/// trailer words at `+0x10`/`+0x14` are copied. The vtable is stamped with
/// the base channel's table first, then the curve table. Returns `this`.
///
/// Original: 0x0069BB70 (thiscall, one stack word, callee pops 4).
lf_checker_rt::export!(thiscall, rw_0069BB70(this: u32, src: u32) -> u32 {
    unsafe {
        const BASE_VTABLE: u32 = 0xFE3A74;
        const CURVE_VTABLE: u32 = 0xFE3E54;
        const KEYS_OFF: u32 = 8;
        const TRAIL0_OFF: u32 = 0x10;
        const TRAIL1_OFF: u32 = 0x14;
        const KEY_COPY: u32 = 1;
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
        wr32(this, lf_checker_rt::relocated(CURVE_VTABLE));
        wr32(this + KEYS_OFF + 4, 0);
        wr32(this + KEYS_OFF, 0);
        lf_checker_rt::callee_thiscall!(KEY_COPY, u32, this + KEYS_OFF, src + KEYS_OFF);
        wr32(this + TRAIL0_OFF, rd32(src + TRAIL0_OFF));
        wr32(this + TRAIL1_OFF, rd32(src + TRAIL1_OFF));
        this
    }
});
