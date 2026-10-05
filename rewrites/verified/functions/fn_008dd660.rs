// original: 0x008DD660 CDrawRadarCircleDC::vf1

/// CDrawRadarCircleDC::vf1 (draw-command virtual slot 1).
///
/// Passes three embedded-point pointers with the constants 0x28
/// and 0 to the radar-circle routine.
///
/// Original: 0x008DD660 (thiscall, `this` in ECX).
lf_checker_rt::export!(thiscall, rw_008dd660(this: u32) -> u32 {
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
        lf_checker_rt::callee_cdecl!(1, u32, this.wrapping_add(0x8), this.wrapping_add(0x10), 0x28u32, this.wrapping_add(0x18), 0x0u32);
        0
    }
});
