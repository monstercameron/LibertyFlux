// original: 0x00879110 rage::crFrameFilters::vf4

/// Fold the children's answers into a rolling hash.
///
/// `this` points to a frame-filter object holding a child count at
/// `+0x2c` and child pointers from `+0xc`. An empty list returns 0.
/// Otherwise each child runs its virtual at slot `+0x10`; a zero answer
/// rejects the whole hash with 0 at once, else the accumulator rotates
/// left by 7 and xors the answer in. Indices run while below the count
/// (UNSIGNED comparison).
///
/// Original: 0x00879110 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_00879110(this: u32) -> u32 {
    unsafe {
        const COUNT_OFF: u32 = 0x2c;
        const CHILDREN_OFF: u32 = 0x0c;
        const POLL_VT_SLOT: u32 = 0x10;
        const ROT_BITS: u32 = 7;
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        let count = rd32(this.wrapping_add(COUNT_OFF));
        if count == 0 {
            return 0;
        }
        let mut acc = 0u32;
        let mut i = 0u32;
        while i < count {
            let child = rd32(
                this.wrapping_add(CHILDREN_OFF).wrapping_add(i.wrapping_mul(4)));
            let vtable = rd32(child);
            let poll: extern "thiscall" fn(u32) -> u32 = unsafe {
                core::mem::transmute(rd32(vtable.wrapping_add(POLL_VT_SLOT)) as usize)
            };
            let answer = poll(child);
            if answer == 0 {
                return 0;
            }
            acc = acc.rotate_left(ROT_BITS) ^ answer;
            i = i.wrapping_add(1);
        }
        acc
    }
});
