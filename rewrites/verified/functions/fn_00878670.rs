// original: 0x00878670 rage::crmtComposerOptimized::CombiningFilter::vf3

/// Poll the child filters with per-child AND/OR semantics.
///
/// `this` points to a combining-filter object holding a child count at
/// `+0x2c`, child pointers from `+0xc` and a 32-bit mode mask at `+0x30`.
/// A count of zero or less (SIGNED comparison) accepts at once. Otherwise
/// each child runs its virtual at slot `+0xc` with (`a0`, `a1`, `a2`):
/// children whose mask bit is set are OR-terms (a non-zero answer accepts
/// the whole poll at once), children whose bit is clear are AND-terms (a
/// zero answer rejects at once). Surviving every child accepts.
///
/// Original: 0x00878670 (thiscall, three stack arguments; low byte of
/// the return only).
lf_checker_rt::export!(thiscall, rw_00878670(this: u32, a0: u32, a1: u32, a2: u32) -> u32 {
    unsafe {
        const COUNT_OFF: u32 = 0x2c;
        const CHILDREN_OFF: u32 = 0x0c;
        const MASK_OFF: u32 = 0x30;
        const POLL_VT_SLOT: u32 = 0x0c;
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        let count = rd32(this.wrapping_add(COUNT_OFF)) as i32;
        if count <= 0 {
            return 1;
        }
        let mask = rd32(this.wrapping_add(MASK_OFF));
        let mut i = 0i32;
        let mut bit = 1u32;
        while i < count {
            let child = rd32(
                this.wrapping_add(CHILDREN_OFF).wrapping_add((i as u32).wrapping_mul(4)));
            let vtable = rd32(child);
            let poll: extern "thiscall" fn(u32, u32, u32, u32) -> u32 = unsafe {
                core::mem::transmute(rd32(vtable.wrapping_add(POLL_VT_SLOT)) as usize)
            };
            let answer = poll(child, a0, a1, a2);
            if mask & bit != 0 {
                if (answer as u8) != 0 {
                    return 1;
                }
            } else if (answer as u8) == 0 {
                return 0;
            }
            i += 1;
            bit = bit.wrapping_shl(1);
        }
        1
    }
});
