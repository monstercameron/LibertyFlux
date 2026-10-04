// original: 0x009FE060 frag_freelist_take (proposed)

/// Take the head node of `obj`'s free list and move it to the live list.
///
/// The free-list head is the dword at `+0x20`; the list is empty when it
/// equals `obj + 0x24`, in which case returns 0. Otherwise stores the dword
/// found at the slot through the head node, unlinks the head
/// through the shared unlink callee, inserts it into `obj`'s own list
/// through the shared insert callee, and returns the head node.
///
/// Original: 0x009FE060 (thiscall, `obj` in `ecx`, `slot` one stack word).
lf_checker_rt::export!(thiscall, rw_009FE060(obj: u32, slot: u32) -> u32 {
    unsafe {
        const HEAD_OFF: u32 = 0x20;
        const EMPTY_OFF: u32 = 0x24;
        let head = ((obj + HEAD_OFF) as *const u32).read_unaligned();
        if head == obj + EMPTY_OFF {
            return 0;
        }
        let v = (slot as *const u32).read_unaligned();
        ((head) as *mut u32).write_unaligned(v);
        lf_checker_rt::callee_thiscall!(1, u32, head);
        lf_checker_rt::callee_thiscall!(2, u32, obj, head);
        head
    }
});
