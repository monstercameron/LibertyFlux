// original: 0x008DDD00 CHud_RenderWindowDC::vf1

/// CHud_RenderWindowDC::vf1 (draw-command virtual slot 1).
///
/// Passes two embedded-rectangle pointers and the style word at
/// +0x18 to the window renderer.
///
/// Original: 0x008DDD00 (thiscall, `this` in ECX).
lf_checker_rt::export!(thiscall, rw_008ddd00(this: u32) -> u32 {
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
        lf_checker_rt::callee_cdecl!(1, u32, this.wrapping_add(0x8), this.wrapping_add(0x10), rd32(this + 0x18));
        0
    }
});
