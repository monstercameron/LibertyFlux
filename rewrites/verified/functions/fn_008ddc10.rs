// original: 0x008DDC10 CGrcState_SetDepthBias::vf1

/// CGrcState_SetDepthBias::vf1 (draw-command virtual slot 1).
///
/// Sets graphics state 0xb (depth bias) to the float at +0x08,
/// passed twice (the original pushes it twice).
///
/// Original: 0x008DDC10 (thiscall, `this` in ECX).
lf_checker_rt::export!(thiscall, rw_008ddc10(this: u32) -> u32 {
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
        lf_checker_rt::callee_cdecl!(1, u32, 0xbu32, rd32(this + 0x8), rd32(this + 0x8));
        0
    }
});
