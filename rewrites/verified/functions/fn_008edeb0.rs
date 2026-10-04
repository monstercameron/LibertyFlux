// original: 0x008edeb0 unlink_node
/// Remove a node from its doubly linked list and count the removal.
///
/// The node stores its neighbours inline (previous at +0, next at +4):
/// the next node's back-link is always repointed at the previous node, and
/// the previous node's forward-link is repointed only when a previous node
/// exists. The instance counter at `this + 0xE04` is decremented either way.
/// Returns the next node, or zero when there was no previous node (matching
/// the value the original leaves in EAX on each path).
export!(thiscall, rw_008edeb0(this_ptr: u32, node: u32) -> u32 {
    unsafe {
        const COUNT_OFF: u32 = 0xE04;
        let prev = *(node as *const u32);
        let next = *((node + 4) as *const u32);
        *(next as *mut u32) = prev;
        let counter = (this_ptr + COUNT_OFF) as *mut u32;
        if prev != 0 {
            *((prev + 4) as *mut u32) = next;
            *counter = (*counter).wrapping_sub(1);
            next
        } else {
            *counter = (*counter).wrapping_sub(1);
            0
        }
    }
});
