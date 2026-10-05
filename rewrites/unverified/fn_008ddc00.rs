// original: 0x008DDC00 CGrcState_SetCullMode::vf1

/// CGrcState_SetCullMode::vf1 (draw-command virtual slot 1).
///
/// Sets graphics state 0 (cull mode) to the word at +0x08.
///
/// Original: 0x008DDC00 (thiscall, `this` in ECX).
lf_checker_rt::export!(thiscall, rw_008ddc00(this: u32) -> u32 {
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
        lf_checker_rt::callee_cdecl!(1, u32, 0x0u32, rd32(this + 0x8));
        0
    }
});
