// original: 0x008DCBC0 CDrawDefLight::vf1

/// CDrawDefLight::vf1 (draw-command virtual slot 1).
///
/// Sets the two light words at +0x18/+0x1c, passes four embedded
/// vectors (+0x08/+0x0c/+0x10/+0x14) to the light-upload routine, then
/// tail-calls the light-apply routine. ECX at the tail jump is not
/// compared: both intermediate callees use ECX as scratch and the
/// tail target overwrites ECX from its global on entry, so the value
/// is indeterminate and ignored. The inventory size (66) runs 22
/// bytes past the tail jump into the next function; the body is 44.
///
/// Original: 0x008DCBC0 (thiscall, `this` in ECX).
lf_checker_rt::export!(thiscall, rw_008dcbc0(this: u32) -> u32 {
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
        lf_checker_rt::callee_cdecl!(1, u32, rd32(this + 0x18), rd32(this + 0x1c));
        lf_checker_rt::callee_cdecl!(2, u32, this.wrapping_add(8), this.wrapping_add(0xc),
            this.wrapping_add(0x10), this.wrapping_add(0x14));
        lf_checker_rt::callee_fastcall!(3, u32, this, 0);
        0
    }
});
