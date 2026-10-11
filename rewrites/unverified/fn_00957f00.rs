// original: 0x00957F00 task_list_pop_head

/// Remove and return the first node of a singly linked task list.
///
/// `this` points to a 32-bit list header: the head pointer is at byte offset
/// 0, the tail pointer at 4, and the node count at 8. Each node's next pointer
/// is at byte offset 16. When the list is empty, the header is untouched and
/// zero is returned. Otherwise the head advances to the removed node's next
/// link, that link is cleared, the tail is cleared when it referred to the
/// removed node, and the count is decremented with 32-bit wrapping semantics.
/// The function uses the x86 thiscall convention with the header in ECX and
/// returns the removed pointer in EAX.
lf_checker_rt::export!(thiscall, rw_00957f00(this: u32) -> u32 {
    unsafe {
        const LIST_HEAD: u32 = 0x00;
        const LIST_TAIL: u32 = 0x04;
        const LIST_LENGTH: u32 = 0x08;
        const NODE_NEXT: u32 = 0x10;

        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }

        #[inline(always)]
        unsafe fn write_u32(address: u32, value: u32) {
            unsafe { (address as *mut u32).write_unaligned(value) }
        }

        let first_node = read_u32(this + LIST_HEAD);
        if first_node != 0 {
            let next_node = read_u32(first_node + NODE_NEXT);
            write_u32(this + LIST_HEAD, next_node);
            write_u32(first_node + NODE_NEXT, 0);

            if read_u32(this + LIST_TAIL) == first_node {
                write_u32(this + LIST_TAIL, 0);
            }

            let length = read_u32(this + LIST_LENGTH);
            write_u32(this + LIST_LENGTH, length.wrapping_sub(1));
        }
        first_node
    }
});
