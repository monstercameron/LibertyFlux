// original: 0x00D5C480 loader_list_insert_head (proposed)

/// Insert a detached doubly linked node at the head of a list. The list stores
/// its count, tail, and head at byte offsets 0, 4, and 8. Each node stores its
/// previous and next links at offsets 0 and 4. The new node's previous link is
/// cleared, its next link receives the prior head, and the prior head is
/// updated to point back to it. For an empty list, the new node becomes the
/// tail. The function returns the old count and then increments it with
/// wrapping 32-bit arithmetic.
lf_checker_rt::export!(thiscall, rw_00D5C480(list: u32, node: u32) -> u32 {
    unsafe {
        const LIST_COUNT: u32 = 0;
        const LIST_TAIL: u32 = 4;
        const LIST_HEAD: u32 = 8;
        const NODE_PREVIOUS: u32 = 0;
        const NODE_NEXT: u32 = 4;

        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn write_u32(address: u32, value: u32) {
            unsafe { (address as *mut u32).write_unaligned(value) }
        }

        let prior_head = read_u32(list.wrapping_add(LIST_HEAD));
        write_u32(list.wrapping_add(LIST_HEAD), node);
        write_u32(node.wrapping_add(NODE_NEXT), prior_head);
        write_u32(node.wrapping_add(NODE_PREVIOUS), 0);
        if prior_head != 0 {
            write_u32(prior_head.wrapping_add(NODE_PREVIOUS), node);
        } else {
            write_u32(list.wrapping_add(LIST_TAIL), node);
        }
        let previous_count = read_u32(list.wrapping_add(LIST_COUNT));
        write_u32(list.wrapping_add(LIST_COUNT), previous_count.wrapping_add(1));
        previous_count
    }
});
