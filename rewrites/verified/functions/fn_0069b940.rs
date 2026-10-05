// original: 0x0069B940 rage::crAnimChannelStaticVector3::vf20

/// Serializes one static Vector3 slot through the stream helper.
///
/// `this` is the channel object and the stack argument is the target object
/// the helper runs against. The slot word at `[this+8]` is pushed and the
/// helper (callee 1, thiscall: target, slot) is called; its answer is the
/// return value in `eax`.
///
/// Original: 0x0069B940 (thiscall, one stack word, callee pops 4).
lf_checker_rt::export!(thiscall, rw_0069B940(this: u32, target: u32) -> u32 {
    unsafe {
        const SLOT_OFF: u32 = 8;
        const HELPER: u32 = 1;
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u32 {
            unsafe { (a as *const u16).read_unaligned() as u32 }
        }
        lf_checker_rt::callee_thiscall!(HELPER, u32, target, rd32(this + SLOT_OFF))
    }
});
