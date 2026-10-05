// original: 0x008DDD40 CRAGE_SetRenderStateDC::vf1

/// CRAGE_SetRenderStateDC::vf1 (draw-command virtual slot 1).
///
/// Sets render state (id at +0x08, value at +0x0c).
///
/// Original: 0x008DDD40 (thiscall, `this` in ECX).
lf_checker_rt::export!(thiscall, rw_008ddd40(this: u32) -> u32 {
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
        lf_checker_rt::callee_cdecl!(1, u32, rd32(this + 0x8), rd32(this + 0xc));
        0
    }
});
