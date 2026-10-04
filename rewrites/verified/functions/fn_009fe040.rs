// original: 0x009FE040 frag_list_insert_head (proposed)

/// Insert `node` at the head of an intrusive doubly-linked frag list.
///
/// `list` points to the list head whose current first node is the dword at
/// `+8`; `node` is the node to insert. Links are `+4` (previous) and `+8`
/// (next). The node takes the head's old next, the old next node's previous
/// is repointed at the node, and the head points at the node. Returns the
/// head's previous first node (the value the original leaves in `eax`).
///
/// Original: 0x009FE040 (thiscall, `list` in `ecx`, `node` one stack word).
lf_checker_rt::export!(thiscall, rw_009FE040(list: u32, node: u32) -> u32 {
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
        let old_next = rd(list + NEXT_OFF);
        wr(node + NEXT_OFF, old_next);
        let ret = rd(list + NEXT_OFF);
        wr(old_next + PREV_OFF, node);
        wr(node + PREV_OFF, list);
        wr(list + NEXT_OFF, node);
        ret
    }
});
