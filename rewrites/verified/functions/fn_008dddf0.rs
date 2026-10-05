// original: 0x008DDDF0 CSetColorWrite::vf1

/// CSetColorWrite::vf1 (draw-command virtual slot 1).
///
/// Sets graphics state 0xf (colour write mask) to the word at +0x08.
///
/// Original: 0x008DDDF0 (thiscall, `this` in ECX).
lf_checker_rt::export!(thiscall, rw_008dddf0(this: u32) -> u32 {
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
        lf_checker_rt::callee_cdecl!(1, u32, 0xfu32, rd32(this + 0x8));
        0
    }
});
