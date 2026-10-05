// original: 0x008DD570 CDrawPolyLoadingClockDC::vf1

/// CDrawPolyLoadingClockDC::vf1 (draw-command virtual slot 1).
///
/// Passes five embedded-vertex pointers and three trailing words
/// to the loading-clock polygon routine.
///
/// Original: 0x008DD570 (thiscall, `this` in ECX).
lf_checker_rt::export!(thiscall, rw_008dd570(this: u32) -> u32 {
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
        lf_checker_rt::callee_cdecl!(1, u32, this.wrapping_add(0x8), this.wrapping_add(0x10), this.wrapping_add(0x18), this.wrapping_add(0x20), this.wrapping_add(0x28), rd32(this + 0x30), rd32(this + 0x34), rd32(this + 0x38));
        0
    }
});
