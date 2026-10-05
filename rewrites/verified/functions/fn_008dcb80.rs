// original: 0x008DCB80 CDrawCurvedWindowDC::vf1

/// CDrawCurvedWindowDC::vf1 (draw-command virtual slot 1).
///
/// Passes four float parameters (+0x08..+0x14, moved bit-exact,
/// no arithmetic) and the flags word at +0x18.
///
/// Original: 0x008DCB80 (thiscall, `this` in ECX).
lf_checker_rt::export!(thiscall, rw_008dcb80(this: u32) -> u32 {
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
        lf_checker_rt::callee_cdecl!(1, u32, rd32(this + 0x8), rd32(this + 0xc), rd32(this + 0x10), rd32(this + 0x14), rd32(this + 0x18));
        0
    }
});
