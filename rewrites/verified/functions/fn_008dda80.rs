// original: 0x008DDA80 CDrawSpriteInPerspectiveDC::vf1

/// CDrawSpriteInPerspectiveDC::vf1 (draw-command virtual slot 1).
///
/// Draws one perspective sprite. Same shape as the plain sprite draw:
/// the texture handle at +0x50 selects default-texture setup plus a
/// conditional tail-jump restore through the shared relay; the draw
/// call takes five embedded-matrix pointers.
///
/// Original: 0x008DDA80 (thiscall, `this` in ECX).
lf_checker_rt::export!(thiscall, rw_008dda80(this: u32) -> u32 {
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
        const TEX: u32 = 0x50;
        lf_checker_rt::callee_cdecl!(1, u32, rd32(this + TEX));
        if rd32(this + TEX) == 0 {
            lf_checker_rt::callee_cdecl!(2, u32,);
        }
        lf_checker_rt::callee_cdecl!(3, u32, this.wrapping_add(0x10), this.wrapping_add(0x20),
            this.wrapping_add(0x30), this.wrapping_add(0x40),
            this.wrapping_add(0x54));
        if rd32(this + TEX) == 0 {
        let g = lf_checker_rt::global::<u32>(0x011736C8).read();
        lf_checker_rt::callee_thiscall!(4, u32, g);
        lf_checker_rt::callee_fastcall!(5, u32, g, 0);
        }
        0
    }
});
