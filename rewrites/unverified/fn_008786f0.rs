// original: 0x008786F0 composer_list_append

/// Append `node` to the composer's pending list.
///
/// `this` points to a composer object holding a singly linked pending list:
/// head at `+0x80`, cached tail at `+0x84`, next slots at node `+4`. The
/// tail cache may lag (it is only a hint): the append point is found by
/// walking `next` slots from the tail hint, or from the head slot when no
/// tail is cached, until a null slot. `node` is stored there and becomes
/// the new cached tail.
///
/// Original: 0x008786F0 (thiscall, one stack argument). No return value.
lf_checker_rt::export!(thiscall, rw_008786F0(this: u32, node: u32) -> u32 {
    unsafe {
        const HEAD_OFF: u32 = 0x80;
        const TAIL_OFF: u32 = 0x84;
        const NEXT_OFF: u32 = 4;
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        let tail = rd32(this.wrapping_add(TAIL_OFF));
        let mut slot = if tail != 0 {
            tail.wrapping_add(NEXT_OFF)
        } else {
            this.wrapping_add(HEAD_OFF)
        };
        while rd32(slot) != 0 {
            slot = rd32(slot).wrapping_add(NEXT_OFF);
        }
        wr32(slot, node);
        wr32(this.wrapping_add(TAIL_OFF), node);
        0
    }
});
