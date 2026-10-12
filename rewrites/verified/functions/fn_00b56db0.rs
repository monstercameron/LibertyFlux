// original: 0x00B56DB0 list_erase_8c_90

/// Remove `node` from the list at `this`, updating its head and tail fields when they match. The links live at byte offsets `0x8c` and `0x90`. The same node is then passed to the intercepted unlink routine, whose return value is forwarded.
lf_checker_rt::export!(thiscall, rw_bb56db0(this: u32, node: u32) -> u32 {
    unsafe {
        const LIST_HEAD: usize = 0;
        const LIST_TAIL: usize = 4;
        const NODE_NEXT: usize = 0x8c;
        const NODE_PREV: usize = 0x90;
        let list_ptr = this as *mut u8;
        let node_ptr = node as *mut u8;
            if node == (list_ptr.add(LIST_HEAD) as *const u32).read_unaligned() {
                let new_head = (node_ptr.add(NODE_NEXT) as *const u32).read_unaligned();
                (list_ptr.add(LIST_HEAD) as *mut u32).write_unaligned(new_head);
            }

        let list_tail = (list_ptr.add(LIST_TAIL) as *const u32).read_unaligned();
        if node == list_tail {
            let previous = (node_ptr.add(NODE_PREV) as *const u32).read_unaligned();
            (list_ptr.add(LIST_TAIL) as *mut u32).write_unaligned(previous);
        }
        lf_checker_rt::callee_thiscall!(1, u32, node)
    }
});
