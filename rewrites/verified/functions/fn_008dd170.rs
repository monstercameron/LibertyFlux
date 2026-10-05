// original: 0x008DD170 CDrawMobilePhoneCameraDC::vf1

/// CDrawMobilePhoneCameraDC::vf1 (draw-command virtual slot 1).
///
/// Passes the camera id, three embedded-buffer pointers, one float
/// and two flag bytes to the phone-camera draw routine.
///
/// Original: 0x008DD170 (thiscall, `this` in ECX).
lf_checker_rt::export!(thiscall, rw_008dd170(this: u32) -> u32 {
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
        lf_checker_rt::callee_cdecl!(1, u32, rd32(this + 0x8), this.wrapping_add(0x10), this.wrapping_add(0x400), this.wrapping_add(0x410), rd32(this + 0x418), rd8(this + 0x41c) as u32, rd8(this + 0x41d) as u32);
        0
    }
});
