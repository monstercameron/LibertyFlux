// original: 0x00612460 physics_unlink_node

/// Detach `node` from the doubly linked list in `this`, repair its adjacent
/// nodes or the list's first and last pointers, clear both node links, and
/// return its former next pointer. The header count at +8 is decremented with
/// wrapping 32-bit arithmetic. This is a 32-bit thiscall with `node` on stack.
lf_checker_rt::export!(thiscall, rw_00612460(list: u32, node: u32) -> u32 {
    unsafe {
        const FIRST: u32 = 0;
        const LAST: u32 = 4;
        const COUNT: u32 = 8;
        const PREVIOUS: u32 = 0xf4;
        const NEXT: u32 = 0xf8;
        let previous = ((node + PREVIOUS) as *const u32).read_unaligned();
        let next = ((node + NEXT) as *const u32).read_unaligned();
        if previous == 0 {
            ((list + FIRST) as *mut u32).write_unaligned(next);
            if next != 0 {
                ((next + PREVIOUS) as *mut u32).write_unaligned(0);
            } else {
                ((list + LAST) as *mut u32).write_unaligned(0);
            }
        } else {
            ((previous + NEXT) as *mut u32).write_unaligned(next);
            if next != 0 {
                ((next + PREVIOUS) as *mut u32).write_unaligned(previous);
            } else {
                ((list + LAST) as *mut u32).write_unaligned(previous);
            }
        }
        ((node + NEXT) as *mut u32).write_unaligned(0);
        ((node + PREVIOUS) as *mut u32).write_unaligned(0);
        let count = ((list + COUNT) as *const u32).read_unaligned();
        ((list + COUNT) as *mut u32).write_unaligned(count.wrapping_sub(1));
        next
    }
});
