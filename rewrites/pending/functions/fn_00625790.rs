// original: 0x00625790 dllist_remove_64
// Remove a node from a doubly-linked list with head, tail and count.
//
// Unlinks `node`: splices the head forward when it is first (emptying the
// list when it is also last), splices the tail back when it is last, else
// links its neighbours together. Clears the removed links and decrements the
// count. Returns the new head, the new tail, or the removed predecessor,
// depending on the path.
export!(thiscall, rw_00625790(list: u32, node: u32) -> u32 {
    unsafe {
        const NEXT_OFF: u32 = 0x64;
        const PREV_OFF: u32 = 0x68;
        const TAIL_OFF: u32 = 4;
        const COUNT_OFF: u32 = 8;
        if node == *(list as *const u32) {
            let next: u32 = *((node.wrapping_add(NEXT_OFF)) as *const u32);
            *(list as *mut u32) = next;
            *((node.wrapping_add(NEXT_OFF)) as *mut u32) = 0;
            let head: u32 = *(list as *const u32);
            if head != 0 {
                *((head.wrapping_add(PREV_OFF)) as *mut u32) = 0;
            } else {
                *((list.wrapping_add(TAIL_OFF)) as *mut u32) = 0;
            }
            let slot = (list.wrapping_add(COUNT_OFF)) as *mut u32;
            *slot = (*slot).wrapping_sub(1);
            return head;
        }
        if node == *((list.wrapping_add(TAIL_OFF)) as *const u32) {
            let prev: u32 = *((node.wrapping_add(PREV_OFF)) as *const u32);
            *((list.wrapping_add(TAIL_OFF)) as *mut u32) = prev;
            *((node.wrapping_add(PREV_OFF)) as *mut u32) = 0;
            let tail: u32 = *((list.wrapping_add(TAIL_OFF)) as *const u32);
            if tail != 0 {
                *((tail.wrapping_add(NEXT_OFF)) as *mut u32) = 0;
            }
            let slot = (list.wrapping_add(COUNT_OFF)) as *mut u32;
            *slot = (*slot).wrapping_sub(1);
            return tail;
        }
        let prev: u32 = *((node.wrapping_add(PREV_OFF)) as *const u32);
        let next: u32 = *((node.wrapping_add(NEXT_OFF)) as *const u32);
        *((prev.wrapping_add(NEXT_OFF)) as *mut u32) = next;
        *((next.wrapping_add(PREV_OFF)) as *mut u32) = prev;
        *((node.wrapping_add(NEXT_OFF)) as *mut u32) = 0;
        *((node.wrapping_add(PREV_OFF)) as *mut u32) = 0;
        let slot = (list.wrapping_add(COUNT_OFF)) as *mut u32;
        *slot = (*slot).wrapping_sub(1);
        prev
    }
});
