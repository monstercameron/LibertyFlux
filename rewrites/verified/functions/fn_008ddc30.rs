// original: 0x008DDC30 CGrcState_SetDepthWrite::vf1

/// CGrcState_SetDepthWrite::vf1 (draw-command virtual slot 1).
///
/// Sets graphics state 6 (depth write) to the byte at +0x08.
///
/// Original: 0x008DDC30 (thiscall, `this` in ECX).
lf_checker_rt::export!(thiscall, rw_008ddc30(this: u32) -> u32 {
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
        lf_checker_rt::callee_cdecl!(1, u32, 0x6u32, rd8(this + 0x8) as u32);
        0
    }
});
