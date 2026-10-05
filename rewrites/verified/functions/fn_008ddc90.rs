// original: 0x008DDC90 CHud_RenderSpriteDC::vf1

/// CHud_RenderSpriteDC::vf1 (draw-command virtual slot 1).
///
/// Draws one HUD sprite; the kind word at +0x24 selects the routine:
/// 2 takes two embedded rects, three words; 3 and 4 take two embedded
/// rects and four words (the extra word at +0x28). Any other kind draws
/// nothing and returns.
///
/// Original: 0x008DDC90 (thiscall, `this` in ECX).
lf_checker_rt::export!(thiscall, rw_008ddc90(this: u32) -> u32 {
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
        const KIND: u32 = 0x24;
        match rd32(this + KIND) {
            2 => { lf_checker_rt::callee_cdecl!(1, u32, this.wrapping_add(8), this.wrapping_add(0x10),
                rd32(this + 0x18), rd32(this + 0x1c), rd32(this + 0x20)); }
            3 => { lf_checker_rt::callee_cdecl!(2, u32, this.wrapping_add(8), this.wrapping_add(0x10),
                rd32(this + 0x18), rd32(this + 0x1c), rd32(this + 0x28),
                rd32(this + 0x20)); }
            4 => { lf_checker_rt::callee_cdecl!(3, u32, this.wrapping_add(8), this.wrapping_add(0x10),
                rd32(this + 0x18), rd32(this + 0x1c), rd32(this + 0x28),
                rd32(this + 0x20)); }
            _ => {}
        }
        0
    }
});
