// original: 0x00698610 rage::crCreatureComponentSkeleton::ctor_tls

/// Initialise a TLS-bound skeleton handle and resolve its group value.
///
/// `thiscall` with the object in ECX and one ignored stack word. Zeroes the
/// header, stamps the class vtable, looks the group up through TLS slot 0 at
/// offset 4, and when present runs the two (intercepted) resolver calls: the
/// first fills the word at `+12`, the second's answer is added to it. A
/// missing group, a -1 answer or an empty slot leaves `+12` zeroed or
/// untouched respectively. Returns the object.
/// Original: 0x00698610, 94 bytes.
lf_checker_rt::export!(thiscall, rw_00698610(this: u32, _a0: u32) -> u32 {
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
        wr32(this + 8, 0);
        wr32(this + 4, 0);
        wr32(this, lf_checker_rt::relocated(0x00FE3B00));
        let slot = lf_checker_rt::tls_slot(0);
        let field = this.wrapping_add(0x0C);
        let grp = rd32(slot + 4);
        if grp == 0 {
            wr32(field, 0);
            return this;
        }
        let key = rd32(grp);
        let r: u32 = lf_checker_rt::callee_thiscall!(1, u32, key, field);
        if r == 0xFFFF_FFFF {
            wr32(field, 0);
            return this;
        }
        let v = rd32(field);
        if v == 0 {
            return this;
        }
        let add: u32 = lf_checker_rt::callee_thiscall!(2, u32, grp, v);
        wr32(field, v.wrapping_add(add));
        this
    }
});
