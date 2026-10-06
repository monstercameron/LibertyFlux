// original: 0x006935D0 rage::crCreatureComponent::vf2

/// Zeroes two words of the component: the dwords at +4 and +8. No return
/// value, no calls.
///
/// Original: 0x006935D0 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_006935D0(obj: u32) -> u32 {
    unsafe {
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }

        wr32(obj.wrapping_add(4), 0);
        wr32(obj.wrapping_add(8), 0);
        0
    }
});
