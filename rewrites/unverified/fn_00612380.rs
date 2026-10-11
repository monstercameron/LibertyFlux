// original: 0x00612380 physics_insert_before_first

/// Insert `node` before the current first node in the circular doubly linked
/// list. The list header stores first, last and count at offsets 0, 4 and 8;
/// each node stores its previous and next pointers at +0xf4 and +0xf8. An
/// empty list initializes both ends and clears both node links. This is a
/// 32-bit thiscall with the node pointer on the stack.
lf_checker_rt::export!(thiscall, rw_00612380(list: u32, node: u32) -> () {
    unsafe {
        const FIRST: u32 = 0;
        const COUNT: u32 = 8;
        const PREVIOUS: u32 = 0xf4;
        const NEXT: u32 = 0xf8;
        let first = ((list + FIRST) as *const u32).read_unaligned();
        if first == 0 {
            ((list + FIRST) as *mut u32).write_unaligned(node);
            ((list + 4) as *mut u32).write_unaligned(node);
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
        let count = ((list + COUNT) as *const u32).read_unaligned();
        ((list + COUNT) as *mut u32).write_unaligned(count.wrapping_add(1));
    }
});
