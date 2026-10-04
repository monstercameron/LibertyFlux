// original: 0x00a7e820 task_list_append
/// Append `node` to the tail of this doubly linked task list.
///
/// `this+0` is the head and `this+4` the tail; a null node is a no-op,
/// and a node appended to an empty list becomes both head and tail.
/// Otherwise the old tail's next link (`+0xc`) and the node's previous
/// link (`+0x10`) are wired and the tail updated. Returns nothing.
lf_checker_rt::export!(thiscall, rw_00a7e820(this: u32, node: u32) -> u32 {
    if node == 0 {
        return 0;
    }
    unsafe {
        let this_w = this as *mut u32;
        let tail = this_w.add(1).read_unaligned();
        if tail == 0 {
            this_w.write_unaligned(node);
            this_w.add(1).write_unaligned(node);
        } else {
            ((tail as *mut u32).add(3)).write_unaligned(node);
            let tail = this_w.add(1).read_unaligned();
            ((node as *mut u32).add(4)).write_unaligned(tail);
            this_w.add(1).write_unaligned(node);
        }
    }
    0
});
