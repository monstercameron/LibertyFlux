// original: 0x009FF2C0 frag_list_unlink (proposed)

/// Unlink `node` from its intrusive doubly-linked frag list.
///
/// `node` points to a node whose neighbours are the dwords at `+4`
/// (previous) and `+8` (next). The previous node's next is set to the next
/// node and the next node's previous is set to the previous node; the node
/// itself is left untouched. Returns the next node (the value the original
/// leaves in `eax`).
///
/// Original: 0x009FF2C0 (thiscall, `node` in `ecx`, no stack words).
lf_checker_rt::export!(thiscall, rw_009FF2C0(node: u32) -> u32 {
    unsafe {
        const PREV_OFF: u32 = 4;
        const NEXT_OFF: u32 = 8;
        #[inline(always)]
        unsafe fn rd(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        let next = rd(node + NEXT_OFF);
        let prev = rd(node + PREV_OFF);
        wr(next + PREV_OFF, prev);
        let prev2 = rd(node + PREV_OFF);
        let ret = rd(node + NEXT_OFF);
        wr(prev2 + NEXT_OFF, ret);
        ret
    }
});
