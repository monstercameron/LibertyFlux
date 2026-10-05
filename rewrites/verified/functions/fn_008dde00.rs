// original: 0x008DDE00 CSetCurrentViewportDC::vf1

/// CSetCurrentViewportDC::vf1 (draw-command virtual slot 1).
///
/// Selects the current viewport. When the flag byte at +0x400 is
/// non-zero the viewport index is 0, otherwise it is the embedded
/// viewport at +0x10; the second argument is always 1.
///
/// Original: 0x008DDE00 (thiscall, `this` in ECX).
lf_checker_rt::export!(thiscall, rw_008dde00(this: u32) -> u32 {
    unsafe {
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u32 {
            unsafe { (a as *const u16).read_unaligned() as u32 }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        const FLAG: u32 = 0x400;
        if rd8(this + FLAG) != 0 {
            lf_checker_rt::callee_cdecl!(1, u32, 0, 1);
        } else {
            lf_checker_rt::callee_cdecl!(1, u32, this.wrapping_add(0x10), 1);
        }
        0
    }
});
