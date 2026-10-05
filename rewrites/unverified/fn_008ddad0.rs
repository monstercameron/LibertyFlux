// original: 0x008DDAD0 CDrawSpriteUVDC::vf1

/// CDrawSpriteUVDC::vf1 (draw-command virtual slot 1).
///
/// Draws one UV-mapped sprite. Same shape as the plain sprite draw:
/// the texture handle at +0x48 selects default-texture setup plus a
/// conditional tail-jump restore through the shared relay; the draw
/// call takes nine embedded pointers (corners and UV rects).
///
/// Original: 0x008DDAD0 (thiscall, `this` in ECX).
lf_checker_rt::export!(thiscall, rw_008ddad0(this: u32) -> u32 {
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
        const TEX: u32 = 0x48;
        lf_checker_rt::callee_cdecl!(1, u32, rd32(this + TEX));
        if rd32(this + TEX) == 0 {
            lf_checker_rt::callee_cdecl!(2, u32,);
        }
        lf_checker_rt::callee_cdecl!(3, u32, this.wrapping_add(8), this.wrapping_add(0x10),
            this.wrapping_add(0x18), this.wrapping_add(0x20),
            this.wrapping_add(0x28), this.wrapping_add(0x30),
            this.wrapping_add(0x38), this.wrapping_add(0x40),
            this.wrapping_add(0x4c));
        if rd32(this + TEX) == 0 {
        let g = lf_checker_rt::global::<u32>(0x011736C8).read();
        lf_checker_rt::callee_thiscall!(4, u32, g);
        lf_checker_rt::callee_fastcall!(5, u32, g, 0);
        }
        0
    }
});
