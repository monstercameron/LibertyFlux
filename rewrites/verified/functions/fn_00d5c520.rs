// original: 0x00D5C520 loader_list_remove_node (proposed)

/// Unlink a node from a doubly linked list. The list stores its count, tail,
/// and head at byte offsets 0, 4, and 8; each node stores its previous and next
/// links at offsets 0 and 4. The neighbor links are repaired, or the list head
/// or tail is replaced when the node is at an end. The function returns the
/// old count and then decrements it with wrapping 32-bit arithmetic. It does
/// not clear the removed node's links.
lf_checker_rt::export!(thiscall, rw_00D5C520(list: u32, node: u32) -> u32 {
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

        let previous = read_u32(node.wrapping_add(NODE_PREVIOUS));
        let next = read_u32(node.wrapping_add(NODE_NEXT));
        if previous != 0 {
            write_u32(previous.wrapping_add(NODE_NEXT), next);
        } else {
            write_u32(list.wrapping_add(LIST_HEAD), next);
        }
        if next != 0 {
            write_u32(next.wrapping_add(NODE_PREVIOUS), previous);
        } else {
            write_u32(list.wrapping_add(LIST_TAIL), previous);
        }
        let previous_count = read_u32(list.wrapping_add(LIST_COUNT));
        write_u32(list.wrapping_add(LIST_COUNT), previous_count.wrapping_sub(1));
        previous_count
    }
});
