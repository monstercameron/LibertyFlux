// original: 0x00ca2790 release_and_reset

/// Drop a counted face reference, clear the slots, tear the face down.
///
/// Zeroes the words at `+0x10`/`+0x14`. When the link at `+0x18` is live,
/// its 16-bit count at `+0xa` is decremented (a zero count skips the rest);
/// when the count reaches zero and the kind byte at `+0x08` is 2 or 4,
/// vtable slot 0 is called with the object and 1 (callee 1). The link is
/// then cleared and the face teardown (callee 2, a tail call) runs, whose
/// answer is returned.
///
/// Original: 0x00ca2790 (thiscall, no stack words).
lf_checker_rt::export!(thiscall, rw_00ca2790(this: u32) -> u32 {
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
    unsafe {
        const LINK: u32 = 0x18;
        const COUNT: u32 = 0x0a;
        const KIND: u32 = 0x08;
        let o = rd32(this + LINK);
        wr32(this + 0x10, 0);
        wr32(this + 0x14, 0);
        if o != 0 {
            let rc = rd16(o + COUNT);
            if rc != 0 {
                let kind = rd8(o + KIND);
                let next = rc.wrapping_sub(1);
                wr16(o + COUNT, next);
                if next == 0 && (kind == 2 || kind == 4) {
                    let slot = rd32(rd32(o));
                    let f: extern "thiscall" fn(u32, u32) -> u32 =
                        core::mem::transmute(slot as usize);
                    f(o, 1);
                }
            }
            wr32(this + LINK, 0);
        }
        lf_checker_rt::callee_thiscall!(2, u32, this)
    }
});
