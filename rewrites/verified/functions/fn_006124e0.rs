// original: 0x006124E0 physics_insert_node_batch_at_head

/// Insert `count` consecutive records, each 0x1e0 bytes apart, using the
/// current-first node's previous link to choose the empty, linear or circular
/// list update. The header stores first, last and count at offsets 0, 4 and 8;
/// each node stores the links used here at +0xf4 and +0xf8. The header count
/// wraps as a 32-bit word. This is a 32-bit thiscall with the node-array
/// pointer and count on the stack.
lf_checker_rt::export!(thiscall, rw_006124E0(list: u32, nodes: u32, count: u32) -> () {
    unsafe {
        const FIRST: u32 = 0;
        const LAST: u32 = 4;
        const COUNT: u32 = 8;
        const NODE_STRIDE: u32 = 0x1e0;
        const PREVIOUS: u32 = 0xf4;
        const NEXT: u32 = 0xf8;

        for index in 0..count {
            let node = nodes.wrapping_add(index.wrapping_mul(NODE_STRIDE));
            let first = ((list + FIRST) as *const u32).read_unaligned();
            if first == 0 {
                ((list + FIRST) as *mut u32).write_unaligned(node);
                ((list + LAST) as *mut u32).write_unaligned(node);
                ((node + NEXT) as *mut u32).write_unaligned(0);
                ((node + PREVIOUS) as *mut u32).write_unaligned(0);
            } else {
                let previous = ((first + PREVIOUS) as *const u32).read_unaligned();
                ((node + PREVIOUS) as *mut u32).write_unaligned(previous);
                if previous != 0 {
                    ((previous + NEXT) as *mut u32).write_unaligned(node);
                }
                ((first + PREVIOUS) as *mut u32).write_unaligned(node);
                ((node + NEXT) as *mut u32).write_unaligned(first);
                if previous == 0 {
                    ((list + FIRST) as *mut u32).write_unaligned(node);
                }
            }
            let old_count = ((list + COUNT) as *const u32).read_unaligned();
            ((list + COUNT) as *mut u32).write_unaligned(old_count.wrapping_add(1));
        }
    }
});
