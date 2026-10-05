// original: 0x0069A710 rage::crAnimChannelStaticFloat::eval_indexed

/// Evaluates a static float channel: loads its single value.
///
/// `this` is the channel object whose value is the `f32` at `+8`; the three
/// stack arguments (index, time, flags) are ignored. Returns the value on
/// the x87 stack (`st0`).
///
/// Original: 0x0069A710 (thiscall, three stack words, callee pops 12).
lf_checker_rt::export!(thiscall, rw_0069A710(this: u32, a: u32, b: u32, c: u32) -> f32 {
    unsafe {
        const VALUE_OFF: u32 = 8;
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u32 {
            unsafe { (a as *const u16).read_unaligned() as u32 }
        }
        f32::from_bits(rd32(this + VALUE_OFF))
    }
});
