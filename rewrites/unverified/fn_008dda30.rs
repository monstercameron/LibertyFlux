// original: 0x008DDA30 CDrawSpriteDC::vf1

/// CDrawSpriteDC::vf1 (draw-command virtual slot 1).
///
/// Draws one sprite. The texture handle at +0x28 selects the setup:
/// zero means the default texture (an extra setup call) and a state
/// restore at the end; the restore is a conditional tail jump into a
/// shared relay that the contract intercepts inside (its call and its
/// own tail jump), so both paths are fully compared.
///
/// Original: 0x008DDA30 (thiscall, `this` in ECX).
lf_checker_rt::export!(thiscall, rw_008dda30(this: u32) -> u32 {
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
        const TEX: u32 = 0x28;
        lf_checker_rt::callee_cdecl!(1, u32, rd32(this + TEX));
        if rd32(this + TEX) == 0 {
            lf_checker_rt::callee_cdecl!(2, u32,);
        }
        lf_checker_rt::callee_cdecl!(3, u32, this.wrapping_add(8), this.wrapping_add(0x10),
            this.wrapping_add(0x18), this.wrapping_add(0x20),
            this.wrapping_add(0x2c));
        if rd32(this + TEX) == 0 {
        let g = lf_checker_rt::global::<u32>(0x011736C8).read();
        lf_checker_rt::callee_thiscall!(4, u32, g);
        lf_checker_rt::callee_fastcall!(5, u32, g, 0);
        }
        0
    }
});
