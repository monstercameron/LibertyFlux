// original: 0x006259b0 dllist_insert_64
// Insert a node into a doubly-linked list with head, tail and count.
//
// With a null position appends at the tail (initialising an empty list);
// before the head prepends; otherwise links the node in front of the
// position node. Increments the count and publishes the (node, list) pair
// through the out pointer. Returns the out pointer.
export!(thiscall, rw_006259b0(list: u32, out: u32, pos: u32, node: u32) -> u32 {
    unsafe {
        const NEXT_OFF: u32 = 0x64;
        const PREV_OFF: u32 = 0x68;
        const TAIL_OFF: u32 = 4;
        const COUNT_OFF: u32 = 8;
        if pos == 0 {
            let tail: u32 = *((list.wrapping_add(TAIL_OFF)) as *const u32);
            if tail == 0 {
                let slot = (list.wrapping_add(COUNT_OFF)) as *mut u32;
                *slot = (*slot).wrapping_add(1);
                *((list.wrapping_add(TAIL_OFF)) as *mut u32) = node;
                *(list as *mut u32) = node;
            } else {
                *((tail.wrapping_add(NEXT_OFF)) as *mut u32) = node;
                *((node.wrapping_add(PREV_OFF)) as *mut u32) = tail;
                *((list.wrapping_add(TAIL_OFF)) as *mut u32) = node;
                if *(list as *const u32) == 0 {
                    *(list as *mut u32) = node;
                }
                let slot = (list.wrapping_add(COUNT_OFF)) as *mut u32;
                *slot = (*slot).wrapping_add(1);
            }
        } else if pos == *(list as *const u32) {
            *((node.wrapping_add(NEXT_OFF)) as *mut u32) = pos;
            *((pos.wrapping_add(PREV_OFF)) as *mut u32) = node;
            *(list as *mut u32) = node;
            let slot = (list.wrapping_add(COUNT_OFF)) as *mut u32;
            *slot = (*slot).wrapping_add(1);
        } else {
            *((node.wrapping_add(NEXT_OFF)) as *mut u32) = pos;
            let prev: u32 = *((pos.wrapping_add(PREV_OFF)) as *const u32);
            *((node.wrapping_add(PREV_OFF)) as *mut u32) = prev;
            *((prev.wrapping_add(NEXT_OFF)) as *mut u32) = node;
            *((pos.wrapping_add(PREV_OFF)) as *mut u32) = node;
            let slot = (list.wrapping_add(COUNT_OFF)) as *mut u32;
            *slot = (*slot).wrapping_add(1);
        }
        *(out as *mut u32) = node;
        *((out.wrapping_add(4)) as *mut u32) = list;
        out
    }
});
