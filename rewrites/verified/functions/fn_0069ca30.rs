// original: 0x0069CA30 rage::crAnimChannelCurveFloat::copy_keys

/// Duplicates the key segments of a curve-float key list.
///
/// `this` is the destination list `{base+0, count+4, capacity+6}` and the
/// stack argument the source list. When the counts differ the destination
/// count and capacity are set to the source count and a fresh array is
/// allocated (callee 1, thiscall: destination list, count; a zero count
/// stores null instead). Every segment is then copied through the segment
/// copier (callee 2, thiscall: destination element, source element; 8
/// bytes per element). Returns the last copier answer, or 0 when no
/// element was copied.
///
/// Original: 0x0069CA30 (thiscall, one stack word, callee pops 4).
lf_checker_rt::export!(thiscall, rw_0069CA30(this: u32, src: u32) -> u32 {
    unsafe {
        const COUNT_OFF: u32 = 4;
        const CAP_OFF: u32 = 6;
        const ELEM_SIZE: u32 = 8;
        const ALLOC: u32 = 1;
        const SEG_COPY: u32 = 2;
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
        let want = rd16(src + COUNT_OFF);
        if rd16(this + COUNT_OFF) != want {
            unsafe {
                ((this + COUNT_OFF) as *mut u16).write_unaligned(want as u16);
                ((this + CAP_OFF) as *mut u16).write_unaligned(want as u16);
            }
            if want != 0 {
                let fresh = lf_checker_rt::callee_thiscall!(ALLOC, u32, this, want);
                wr32(this, fresh);
            } else {
                wr32(this, 0);
            }
        }
        let n = rd16(this + COUNT_OFF);
        let mut last: u32 = 0;
        let mut i: u32 = 0;
        while i < n {
            let s = rd32(src).wrapping_add(i.wrapping_mul(ELEM_SIZE));
            let d = rd32(this).wrapping_add(i.wrapping_mul(ELEM_SIZE));
            last = lf_checker_rt::callee_thiscall!(SEG_COPY, u32, d, s);
            i += 1;
        }
        last
    }
});
