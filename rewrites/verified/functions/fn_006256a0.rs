// original: 0x006256a0 task_unlink_from_manager
// Unlink a task node from its manager's singly-linked list.
//
// Marks the node with the in-list tag, asks the manager singleton for the
// list head, walks the `next` chain until the node is found, splices it out
// and clears its link. Decrements the manager's live count, and the flagged
// count when the node's flag bit is set. Returns the removed node's old
// successor. Retags the node on exit.
export!(thiscall, rw_006256a0(node: u32) -> u32 {
    unsafe {
        const ENTER_TAG: u32 = 0x00fe2264;
        const EXIT_TAG: u32 = 0x00fcb270;
        const NEXT_OFF: u32 = 0x18;
        const FLAG_OFF: u32 = 0x1c;
        const COUNT_OFF: u32 = 8;
        const FLAGGED_COUNT_OFF: u32 = 0x0c;
        *(node as *mut u32) = relocated(ENTER_TAG);
        let answer: u32 = callee_cdecl!(1, u32,);
        let mgr = answer;
        let mut prev: u32 = 0;
        let mut cur: u32 = *(mgr as *const u32);
        if cur != node {
            loop {
                prev = cur;
                cur = *((cur.wrapping_add(NEXT_OFF)) as *const u32);
                if cur == node {
                    break;
                }
            }
        }
        if cur == 0 {
            // Only reachable with a null node, which faults on the tag store
            // above; kept so the control flow matches the original exactly.
            *(node as *mut u32) = relocated(EXIT_TAG);
            return answer;
        }
        let next: u32 = *((cur.wrapping_add(NEXT_OFF)) as *const u32);
        if prev == 0 {
            *(mgr as *mut u32) = next;
        } else {
            *((prev.wrapping_add(NEXT_OFF)) as *mut u32) = next;
        }
        *((cur.wrapping_add(NEXT_OFF)) as *mut u32) = 0;
        if *((node.wrapping_add(FLAG_OFF)) as *const u8) & 1 != 0 {
            let slot = (mgr.wrapping_add(FLAGGED_COUNT_OFF)) as *mut u32;
            *slot = (*slot).wrapping_sub(1);
        }
        let slot = (mgr.wrapping_add(COUNT_OFF)) as *mut u32;
        *slot = (*slot).wrapping_sub(1);
        *(node as *mut u32) = relocated(EXIT_TAG);
        next
    }
});
