// original: 0x006123F0 physics_insert_after_last

/// Insert `node` after the current last node in the circular doubly linked
/// list. The header stores first, last and count at offsets 0, 4 and 8; each
/// node stores its previous and next pointers at +0xf4 and +0xf8. An empty
/// list initializes both ends and clears both node links. This is a 32-bit
/// thiscall with the node pointer on the stack.
lf_checker_rt::export!(thiscall, rw_006123F0(list: u32, node: u32) -> () {
    unsafe {
        const LAST: u32 = 4;
        const COUNT: u32 = 8;
        const PREVIOUS: u32 = 0xf4;
        const NEXT: u32 = 0xf8;
        let first = ((list) as *const u32).read_unaligned();
        if first == 0 {
            ((list) as *mut u32).write_unaligned(node);
            ((list + LAST) as *mut u32).write_unaligned(node);
            ((node + NEXT) as *mut u32).write_unaligned(0);
            ((node + PREVIOUS) as *mut u32).write_unaligned(0);
        } else {
            let last = ((list + LAST) as *const u32).read_unaligned();
            let next = ((last + NEXT) as *const u32).read_unaligned();
            ((node + NEXT) as *mut u32).write_unaligned(next);
            if next != 0 {
                ((next + PREVIOUS) as *mut u32).write_unaligned(node);
            }
            ((last + NEXT) as *mut u32).write_unaligned(node);
            ((node + PREVIOUS) as *mut u32).write_unaligned(last);
            if next == 0 {
                ((list + LAST) as *mut u32).write_unaligned(node);
            }
        }
        let count = ((list + COUNT) as *const u32).read_unaligned();
        ((list + COUNT) as *mut u32).write_unaligned(count.wrapping_add(1));
    }
});
