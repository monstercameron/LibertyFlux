// original: 0x008DCB60 CCustomShaderEffectDC::vf1

/// CCustomShaderEffectDC::vf1 (draw-command virtual slot 1).
///
/// Rounds the byte count at +0x0c up to a 16-byte boundary
/// (`x + ((-x) & 0xf)`, wrapping) and passes it with the handle at +0x10.
///
/// Original: 0x008DCB60 (thiscall, `this` in ECX).
lf_checker_rt::export!(thiscall, rw_008dcb60(this: u32) -> u32 {
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
        lf_checker_rt::callee_cdecl!(1, u32, { let x = rd32(this + 0xc); x.wrapping_add(x.wrapping_neg() & 0xf) }, rd32(this + 0x10));
        0
    }
});
