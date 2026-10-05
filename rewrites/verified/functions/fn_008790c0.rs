// original: 0x008790C0 rage::crFrameFilters::vf3

/// Poll the child filters with AND semantics.
///
/// `this` points to a frame-filter object holding a child count at
/// `+0x2c` and child pointers from `+0xc`. A count of zero or less
/// (SIGNED comparison) accepts at once. Otherwise each child runs its
/// virtual at slot `+0xc` with (`a0`, `a1`, `a2`): the first zero low
/// byte rejects the whole poll, surviving every child accepts.
///
/// Original: 0x008790C0 (thiscall, three stack arguments; low byte of
/// the return only).
lf_checker_rt::export!(thiscall, rw_008790C0(this: u32, a0: u32, a1: u32, a2: u32) -> u32 {
    unsafe {
        const COUNT_OFF: u32 = 0x2c;
        const CHILDREN_OFF: u32 = 0x0c;
        const POLL_VT_SLOT: u32 = 0x0c;
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        let count = rd32(this.wrapping_add(COUNT_OFF)) as i32;
        if count <= 0 {
            return 1;
        }
        let mut i = 0i32;
        while i < count {
            let child = rd32(
                this.wrapping_add(CHILDREN_OFF).wrapping_add((i as u32).wrapping_mul(4)));
            let vtable = rd32(child);
            let poll: extern "thiscall" fn(u32, u32, u32, u32) -> u32 = unsafe {
                core::mem::transmute(rd32(vtable.wrapping_add(POLL_VT_SLOT)) as usize)
            };
            if (poll(child, a0, a1, a2) as u8) == 0 {
                return 0;
            }
            i += 1;
        }
        1
    }
});
