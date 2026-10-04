// original: 0x00e5fd70 push_cleanup_node_0110ffe0
/// Push a fixed node onto the shared cleanup list, returning the old head.
///
/// The previous head pointer is stored into the new node's link slot and
/// the list head is updated to the new node.
export!(cdecl, rw_00e5fd70() -> u32 {
    unsafe {
        /// Head of the shared cleanup list.
        const LIST_HEAD: u32 = 0x017ACD24;
        /// Link slot inside the new node (holds the previous head).
        const NODE_LINK: u32 = 0x0110FFE4;
        /// The new node to install as head.
        const NEW_NODE: u32 = 0x0110FFE0;
        let old_head = *global::<u32>(LIST_HEAD);
        *global::<u32>(NODE_LINK) = old_head;
        *global::<u32>(LIST_HEAD) = relocated(NEW_NODE);
        old_head
    }
});
